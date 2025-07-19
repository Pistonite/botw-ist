use std::path::Path;
use std::time::Instant;

use anyhow::{Context, bail};

mod linker_tests;
mod script_tests;
mod util;

fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let result = main_internal();
    let elapsed = start.elapsed().as_secs_f32();
    log::info!("finished in {elapsed:02}s");
    result
}

fn main_internal() -> anyhow::Result<()> {
    env_logger::init();
    util::setup_panic_capture();

    let failures_dir = Path::new("failures");
    if failures_dir.exists() {
        std::fs::remove_dir_all(failures_dir).context("failed to clean failures dir")?;
    }
    std::fs::create_dir_all(failures_dir).context("failed to create failures dir")?;

    let runtime = util::setup_test_process()?;
    let process = runtime
        .initial_process()
        .context("failed to get initial process")?;

    let linker_test_passed = linker_tests::run(&process, failures_dir)?;
    if !linker_test_passed {
        bail!("linker tests failed, not executing further tests");
    }
    let script_test_passed = script_tests::run(runtime)?;
    if !script_test_passed {
        bail!("script tests failed");
    }
    let only_test = std::env::var("SKYBOOK_TEST_ONLY").unwrap_or_default();
    if !only_test.is_empty() {
        log::info!("not collecting extra info since SKYBOOK_TEST_ONLY was specified");
        return Ok(());
    }
    if !cfg!(feature = "trace-memory") {
        bail!(
            "The tests always fail when trace-memory is not enabled to ensure it's not accidentally disabled"
        );
    }
    #[cfg(feature = "trace-memory")]
    {
        util::collect_memory_trace(&process)?;
        // this is the emulated game heap, not the actual host memory
        let max_heap = blueflame::memory::get_max_heap_alloc();
        log::info!("max game heap alloc: {max_heap} bytes")
    }

    Ok(())
}
