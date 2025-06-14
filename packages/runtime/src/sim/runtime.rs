use std::sync::Mutex;

use blueflame::env::{DlcVer, Environment, GameVer};
use blueflame::processor::{Cpu1, Process};
use blueflame::{linker, program};
use hashlink::LruCache;
use skybook_parser::cir;

use crate::error::RuntimeInitError;
use crate::exec::{self, Executor, Spawner};
use crate::sim;

#[doc(inline)]
pub use skybook_api::runtime::sim::RuntimeInitParams;

pub struct Runtime {
    executor: Executor,
    initial_process: Mutex<Option<Process>>,
    state_cache: Mutex<LruCache<Vec<cir::Command>, sim::State>>,
}

impl Runtime {
    /// Create the runtime, but do not initialize it yet
    pub fn new(spawner: Spawner) -> Self {
        let executor = Executor::new(spawner);
        Self {
            executor,
            initial_process: Mutex::new(None),
            state_cache: Mutex::new(LruCache::new(1024)),
        }
    }

    /// Get the initial process, or return an error if not initialized
    pub fn initial_process(&self) -> Result<Process, crate::error::Error> {
        self.initial_process
            .lock()
            .expect("cannot acquire")
            .clone()
            .ok_or(crate::error::Error::Uninitialized)
    }

    /// Initialize the runtime
    pub fn init(
        &self,
        image: &[u8],
        threads: usize,
        params: Option<&RuntimeInitParams>,
    ) -> Result<Environment, RuntimeInitError> {
        log::info!("initializing runtime");
        if let Err(e) = self.executor.ensure_threads(threads.max(1)) {
            log::error!("failed to create threads: {e}");
            return Err(RuntimeInitError::Executor);
        }

        log::debug!("initializing runtime with custom image params: {params:?}");

        let mut program_bytes = Vec::new();
        let program = match program::unpack_zc(image, &mut program_bytes) {
            Err(e) => {
                log::error!("failed to unpack blueflame image: {e}");
                return Err(RuntimeInitError::BadImage);
            }
            Ok(program) => program,
        };

        log::debug!("program start: 0x{:016x}", program.program_start);
        if let Some(program_start) = params.map(|x| &x.program_start)
            && !program_start.is_empty()
        {
            match parse_region_addr(program_start) {
                None => {
                    log::error!(
                        "cannot parse program_start from the params, assuming we are OK with the default"
                    );
                }
                Some(expected_program_start) => {
                    if expected_program_start != program.program_start {
                        return Err(RuntimeInitError::ProgramStartMismatch(
                            format!("0x{:016x}", program.program_start),
                            format!("0x{expected_program_start:016x}"),
                        ));
                    }
                }
            }
        }

        let env = {
            let game_ver = program.ver.into();
            log::info!("game version is {game_ver:?}");
            let params_dlc = params.map(|x| x.dlc).unwrap_or(3);
            let dlc_ver = match DlcVer::from_num(params_dlc) {
                Some(dlc) => dlc,
                None => return Err(RuntimeInitError::BadDlcVersion(params_dlc)),
            };
            log::info!("dlc version is {dlc_ver:?}");

            Environment::new(game_ver, dlc_ver)
        };

        if env.game_ver == GameVer::X160 {
            log::error!(">>>> + LOOK HERE + <<<< Only 1.5 is supported for now");
            return Err(RuntimeInitError::UnsupportedVersion);
        }

        let stack_start = match params.map(|x| &x.stack_start).take_if(|x| !x.is_empty()) {
            None => 0x8888800000,
            Some(x) => match parse_region_addr(x) {
                Some(x) => x,
                None => {
                    log::error!("failed to parse stack_start");
                    return Err(RuntimeInitError::InvalidStackStart);
                }
            },
        };

        let pmdm_addr = match params.map(|x| &x.pmdm_addr).take_if(|x| !x.is_empty()) {
            None => 0x2222248358,
            Some(x) => match parse_region_addr(x) {
                Some(x) => x,
                None => {
                    log::error!("failed to parse pmdm_addr");
                    return Err(RuntimeInitError::InvalidPmdmAddr);
                }
            },
        };

        let heap_free_size = params.map(|x| x.heap_free_size).take_if(|x| *x!=0).unwrap_or(20480000);
        if heap_free_size > 40960000 {
            return Err(RuntimeInitError::HeapTooBig);
        }

        let stack_size = params.map(|x| x.stack_size).take_if(|x| *x != 0).unwrap_or(0x4000);

        let process = match linker::init_process(
            program,
            env.dlc_ver,
            stack_start,
            stack_size,
            pmdm_addr,
            heap_free_size,
        ) {
            Err(e) => {
                log::error!("failed to initialize process: {e}");
                return Err(RuntimeInitError::InitializeProcess);
            }
            Ok(x) => x,
        };

        {
            let mut p = self
                .initial_process
                .lock()
                .expect("failed to acquire lock for initial process");
            *p = Some(process);
        }

        Ok(env)
    }

    pub async fn execute<F, T>(&self, f: F) -> Result<T, exec::Error>
    where
        F: FnOnce(&mut Cpu1) -> T + Send + 'static,
        T: Send + 'static,
    {
        self.executor.execute(f).await
    }

    pub fn find_cached_state(&self, commands: &[cir::Command]) -> Option<sim::State> {
        self.state_cache.lock().unwrap().get(commands).cloned()
    }

    pub fn set_state_cache(&self, commands: &[cir::Command], state: &sim::State) {
        self.state_cache
            .lock()
            .unwrap()
            .insert(commands.to_vec(), state.clone());
    }
}

fn parse_hex(s: &str) -> Option<u64> {
    let s = s.strip_prefix("0x")?;
    s.parse::<u64>().ok()
}

fn parse_region_addr(s: &str) -> Option<u64> {
    let addr = parse_hex(s)?;
    if (addr & 0xffffff00000fffffu64) != 0 {
        log::error!("region address is not the correct format: {s}");
        return None;
    }
    Some(addr)
}
