use std::path::Path;

use cu::pre::*;

mod linker_tests;
mod script_tests;
mod util;

/// Skybook runtime testing framework
#[derive(clap::Parser, Clone)]
struct Args {
    /// Use the mini image instead of the full image
    #[clap(short = 'm', long)]
    mini: bool,
    /// Only run one script test. The arg is name of the script without .txt suffix.
    #[clap(short = 't', long)]
    only: Option<String>,
    /// Refresh script tests snapshots
    #[clap(short = 'R', long)]
    refresh: bool,
    /// Refresh trace hash
    #[clap(short = 'H', long)]
    trace_hash: bool,

    #[clap(flatten)]
    common: cu::cli::Flags,
}

#[cu::cli(flags="common")]
fn main(args: Args) -> cu::Result<()> {
    util::setup_panic_capture();

    let failures_dir = Path::new("failures");
    if failures_dir.exists() {
        std::fs::remove_dir_all(failures_dir).context("failed to clean failures dir")?;
    }
    std::fs::create_dir_all(failures_dir).context("failed to create failures dir")?;

    let image_file = if args.mini  {
        "./data/program-mini.bfi"
    }else {
        "./data/program-full.bfi"
    };

    let runtime = util::setup_test_process(image_file)?;
    let process = runtime
        .initial_process()
        .context("failed to get initial process")?;

    let linker_test_passed = linker_tests::run(&process, failures_dir)?;
    if !linker_test_passed {
        cu::bail!("linker tests failed, not executing further tests");
    }
    let has_only = args.only.is_some();
    let script_test_passed = script_tests::run(runtime, args.refresh, args.only)?;
    if !script_test_passed {
        cu::bail!("script tests failed");
    }
    if has_only {
        cu::info!("not collecting extra info since --only was specified");
        return Ok(());
    }
    if !cfg!(feature = "trace-memory") {
        cu::bail!(
            "The tests always fail when trace-memory is not enabled to ensure it's not accidentally disabled"
        );
    }
    #[cfg(feature = "trace-memory")]
    {
        util::collect_memory_trace(&process, args.trace_hash)?;
        // this is the emulated game heap, not the actual host memory
        let max_heap = blueflame::memory::get_max_heap_alloc();
        cu::info!("max game heap alloc: {max_heap} bytes")
    }

    Ok(())
}
