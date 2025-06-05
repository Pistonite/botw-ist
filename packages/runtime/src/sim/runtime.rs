use std::sync::Mutex;

use blueflame::env::{DlcVer, Environment, GameVer};
use blueflame::processor::{Cpu1, Process};
use blueflame::{program, linker};

use crate::exec::{self, Executor, Spawner};
use crate::error::{RuntimeInitError};

pub use skybook_api::runtime::sim::CustomImageInitParams;


pub struct Runtime {
    pub env: Mutex<Environment>,
    executor: Executor,
    initial_process: Mutex<Option<Process>>,
}

impl Runtime {
    /// Create the runtime, but do not initialize it yet
    pub fn new(spawner: Spawner) -> Self {
        let executor = Executor::new(spawner);
        Self {
            env: Mutex::new(Environment::new(GameVer::X150, DlcVer::None)),
            executor,
            initial_process: Mutex::new(None),
        }
    }

    /// Get the initial process, or return an error if not initialized
    pub fn initial_process(&self) -> Result<Process, crate::error::Error> {
        self.initial_process
            .lock()
            .unwrap()
            .clone()
            .ok_or(crate::error::Error::Uninitialized)
    }

    pub fn game_version(&self) -> GameVer {
        self.env.lock().unwrap().game_ver
    }
    pub fn dlc_version(&self) -> DlcVer {
        self.env.lock().unwrap().dlc_ver
    }

    /// Initialize the runtime
    pub fn init(
        &self,
        custom_image: Option<(Vec<u8>, CustomImageInitParams)>,
    ) -> Result<(), RuntimeInitError> {
        if let Err(e) = self.executor.ensure_threads(4) {
            log::error!("failed to create threads: {}", e);
            return Err(RuntimeInitError::Executor);
        }
        let Some((bytes, params)) = custom_image else {
            log::error!("must provide custom image for now");
            return Err(RuntimeInitError::BadImage);
        };

        log::debug!(
            "initializing runtime with custom image params: {:?}",
            params
        );

        let mut program_bytes = Vec::new();
        let program = match program::unpack_zc(&bytes, &mut program_bytes) {
            Err(e) => {
                log::error!("failed to unpack blueflame image: {}", e);
                return Err(RuntimeInitError::BadImage);
            }
            Ok(program) => program,
        };

        if program.ver == GameVer::X160 {
            log::error!(">>>> + LOOK HERE + <<<< Only 1.5 is supported for now");
            return Err(RuntimeInitError::BadImage);
        }

        log::debug!("program start: {:#x}", program.program_start);

        // TODO: program should not have DLC version, since
        // it doesn't matter statically
        {
            let mut env = self.env.lock().unwrap();
            env.game_ver = program.ver.into();
            env.dlc_ver = match DlcVer::from_num(params.dlc) {
                Some(dlc) => dlc,
                None => return Err(RuntimeInitError::BadDlcVersion(params.dlc)),
            };
        }

        // TODO: take the param
        log::debug!("initializing memory");

        let process = match linker::init_process(
            &program,
            DlcVer::V300, // TODO: take from param
            0x8888800000,
            0x4000, // stack size
            0x2222200000,
            20000000, // this heap looks hug
        ) {
            Err(e) => {
                log::error!("failed to initialize memory: {}", e);
                // TODO: actual error
                return Err(RuntimeInitError::BadImage);
            }
            Ok(x) => x,
        };

        log::debug!("memory initialized successfully");
        {
            let mut initial_memory = self.initial_process.lock().unwrap();
            *initial_memory = Some(process);
        }

        Ok(())
    }

    pub async fn execute<F, T>(&self, f: F) -> Result<T, exec::Error>
    where
        F: FnOnce(&mut Cpu1) -> T + Send + 'static,
        T: Send + 'static,
    {
         self.executor.execute(f).await
    }
}

