use std::{panic::UnwindSafe, path::Path};

use blueflame::processor::{Cpu1, Cpu2, CrashReport, Process};
use threadpool::ThreadPool;

use crate::util::{self, PanicPayload};

mod get_tests;
mod hold_tests;
mod pmdm_initialize;

macro_rules! run {
    ($pool:expr, $process:expr, $test_fn:expr) => {
        do_run_linker_test(stringify!($test_fn), $pool, $process, $test_fn)
    };
}

pub fn run(process: &Process, failures_dir: &Path) -> cu::Result<bool> {
    cu::debug!("running linker tests");
    let threads = if cfg!(feature = "single-thread") {
        1
    } else {
        4
    };
    let pool = ThreadPool::new(threads);

    let handles = vec![
        run!(&pool, process, pmdm_initialize::run),
        run!(&pool, process, get_tests::get_item_basic),
        run!(&pool, process, get_tests::get_sword),
        run!(&pool, process, get_tests::get_arrow),
        run!(&pool, process, get_tests::get_bow),
        run!(&pool, process, get_tests::get_shield),
        run!(&pool, process, get_tests::get_material),
        run!(&pool, process, get_tests::get_food),
        run!(&pool, process, get_tests::get_food_with_effect),
        run!(&pool, process, get_tests::get_armor),
        run!(&pool, process, get_tests::get_orb),
        run!(&pool, process, hold_tests::hold_material),
    ];

    let total_count = handles.len();
    let mut passed_count = 0;
    {
        let bar = cu::progress_bar(total_count, "linker tests");
        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.recv.recv().unwrap();
            match result {
                LinkerTestResult::Ok => {
                    cu::info!("PASS {}", handle.name);
                    passed_count += 1;
                }
                LinkerTestResult::Panic(trace) => {
                    cu::error!("FAIL {} - panic", handle.name);
                    let file_path = failures_dir.join(handle.name.replace(':', "_"));
                    let _ = std::fs::write(file_path, trace.to_string());
                }
                LinkerTestResult::Crash(crash) => {
                    cu::error!("FAIL {} - crash", handle.name);
                    let file_path = failures_dir.join(handle.name.replace(':', "_"));
                    let _ = std::fs::write(file_path, format!("{crash:?}"));
                }
            }
            let failed_count = i + 1 - passed_count;
            cu::progress!(&bar, i + 1, "{failed_count} failed");
        }
    }

    cu::info!("{passed_count}/{total_count} linker tests passed");
    pool.join();

    Ok(passed_count == total_count)
}

struct TestHandle {
    name: &'static str,
    recv: oneshot::Receiver<LinkerTestResult>,
}

fn do_run_linker_test<F>(
    name: &'static str,
    pool: &ThreadPool,
    process: &Process,
    f: F,
) -> TestHandle
where
    F: FnOnce(&mut Cpu2) -> Result<(), blueflame::processor::Error> + Send + UnwindSafe + 'static,
{
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
                let info = util::take_last_panic();
                let _ = send.send(LinkerTestResult::Panic(info));
            }
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
    Panic(PanicPayload),
    Crash(CrashReport),
}
