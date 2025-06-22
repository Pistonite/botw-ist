use std::path::Path;

use anyhow::{bail, Context};

mod linker_tests;
mod script_tests;
mod util;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    util::setup_panic_capture();

    let failures_dir = Path::new("failures");
    if failures_dir.exists() {
        std::fs::remove_dir_all(failures_dir).context("failed to clean failures dir")?;
    }
    std::fs::create_dir_all(failures_dir).context("failed to create failures dir")?;

    let runtime = util::setup_test_process()?;
    let process = runtime.initial_process().context("failed to get initial process")?;

    let linker_test_passed = linker_tests::run(&process, failures_dir)?;
    let script_test_passed = script_tests::run(runtime)?;

    util::collect_memory_trace(&process)?;

    if !linker_test_passed || !script_test_passed {
        bail!("there are tests failures");
    }

    Ok(())
}
