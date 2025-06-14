#![feature(panic_payload_as_str)]

use std::backtrace::Backtrace;
use std::cell::Cell;
use std::panic::UnwindSafe;
use std::path::Path;

use anyhow::Context;
use blueflame::env::{DlcVer, GameVer};
use blueflame::processor::{Cpu1, Cpu2, CrashReport, Process};
use blueflame::{program, linker};
use threadpool::ThreadPool;

mod linker_test;

struct PanicInfo {
    message: String,
    backtrace: Backtrace,
}
impl std::fmt::Display for PanicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n", self.message)?;
        writeln!(f, "Backtrace:\n{}", self.backtrace)
    }
}

thread_local! {
    static PANIC_INFO: Cell<Option<PanicInfo>> = Cell::new(None)
}

macro_rules! run_linker {
    ($handles:ident, $pool:expr, $process:expr, $test_fn:expr) => {
        $handles.push(do_run_linker_test(stringify!($test_fn), $pool, $process, $test_fn))
    }
}

fn main() -> anyhow::Result<()> {
    colog::init();

    let failures_dir = Path::new("failures");

    if failures_dir.exists() {
        std::fs::remove_dir_all(failures_dir).context("failed to clean failures dir")?;
    }
    std::fs::create_dir_all(failures_dir).context("failed to create failures dir")?;

    std::panic::set_hook(Box::new(|info| {
        let backtrace = Backtrace::capture();
        let mut message = match info.location() {
            Some(loc) => format!("file: {}, line: {}\n", loc.file(), loc.line()),
            None => "unknown panic location\n".to_string()
        };
        match info.payload_as_str() {
            Some(x) => message += x,
            None => message += "unknown panic info",
        };
        PANIC_INFO.with(|b| b.set(Some(PanicInfo{ message, backtrace })))
    }));

    let image_file = std::env::var("SKYBOOK_RUNTIME_TEST_IMAGE").context("please define SKYBOOK_RUNTIME_TEST_IMAGE")?;
    log::info!("loading {image_file}");

    let image_bytes = std::fs::read(image_file).context("failed to read BFI")?;
    let mut program_bytes = Vec::new();
    let program = program::unpack_zc(&image_bytes, &mut program_bytes).context("failed to deserialize BFI")?;
    let process = linker::init_process(program, DlcVer::V300, 0x8888800000, 0x8000, 0x2222200000, 0x20000000).context("failed to initialize process")?;

    log::info!("running linker_test");
    let pool = ThreadPool::new(4);
    let mut handles = Vec::new();

    run_linker!(handles, &pool, &process, linker_test::pmdm_initialized);
    run_linker!(handles, &pool, &process, linker_test::get_item_basic);
    run_linker!(handles, &pool, &process, linker_test::get_sword);
    run_linker!(handles, &pool, &process, linker_test::get_arrow);
    run_linker!(handles, &pool, &process, linker_test::get_bow);
    run_linker!(handles, &pool, &process, linker_test::get_shield);
    run_linker!(handles, &pool, &process, linker_test::get_material);
    run_linker!(handles, &pool, &process, linker_test::get_food);
    run_linker!(handles, &pool, &process, linker_test::get_food_with_effect);
    run_linker!(handles, &pool, &process, linker_test::get_armor);
    run_linker!(handles, &pool, &process, linker_test::get_orb);

    let total_count = handles.len();
    let mut passed_count = 0;
    for handle in handles {
        let result = handle.recv.recv().unwrap();
        match result {
            LinkerTestResult::Ok => {
                log::info!("PASS {}", handle.name);
                passed_count += 1;
            }
            LinkerTestResult::Panic(trace) => {
                log::error!("FAIL {} - panic", handle.name);
                let file_path = failures_dir.join(handle.name.replace(':', "_"));
                let _ = std::fs::write(file_path, trace.to_string());
            }
            LinkerTestResult::Crash(crash) => {
                log::error!("FAIL {} - crash", handle.name);
                let file_path = failures_dir.join(handle.name.replace(':', "_"));
                let _ = std::fs::write(file_path, format!("{:?}", crash));
            }
        }
    }

    log::info!("{passed_count}/{total_count} linker tests passed");
    pool.join();

    log::info!("collecting memory read trace");

    let main_start = process.main_start();
    let main_end = match process.memory().env().game_ver {
        GameVer::X150 => {
            main_start + 0x26af000 - 0x4000
        },
        GameVer::X160 => {
            main_start + 0x381e000 - 0x4000
        }
    };
    let mut read_report = String::new();
    for (mut start, mut end) in blueflame::memory::get_read_page_ranges() {
        if start < main_start {
            start = main_start;
        }
        if end > main_end {
            end = main_end;
        }
        if start >= end {
            continue;
        }
        let rel_start = start - main_start;
        let rel_end = end - main_start;
        read_report += &format!("-r [main]:0x{rel_start:08x}-0x{rel_end:08x}\n")
    }

    std::fs::write("trace.txt", read_report).context("failed to save trace report")?;



    Ok(())
}



struct TestHandle {
    name: &'static str,
    recv: oneshot::Receiver<LinkerTestResult>
}

fn do_run_linker_test<F>(name: &'static str, pool: &ThreadPool, process: &Process, f: F) -> TestHandle
where F: FnOnce(&mut Cpu2) -> Result<(), blueflame::processor::Error> + Send + UnwindSafe + 'static {
    let (send, recv) = oneshot::channel();
    let mut process = process.clone();
    pool.execute(move || {
        let result = std::panic::catch_unwind(move || {
            let mut cpu1 = Cpu1::default();
            let mut cpu2 = Cpu2::new(&mut cpu1, &mut process);
            cpu2.with_crash_report(f)
        });
        match result {
            Err(_) => {
                let info = PANIC_INFO.with(|b| b.take()).unwrap();
                let _ = send.send(LinkerTestResult::Panic(info));
            },
            Ok(Err(crash)) => {
                let _ = send.send(LinkerTestResult::Crash(crash));
            }
            _ => {
                let _ = send.send(LinkerTestResult::Ok);
            }
        }
    });
    TestHandle { name, recv }
}

enum LinkerTestResult {
    Ok,
    Panic(PanicInfo),
    Crash(CrashReport),
}
