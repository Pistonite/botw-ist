use std::backtrace::Backtrace;
use std::cell::Cell;
use std::sync::Arc;

use anyhow::Context;
use blueflame::env::{GameVer};
use blueflame::processor::Process;
use skybook_runtime::{sim, exec};

pub struct PanicPayload {
    message: String,
    backtrace: Backtrace,
}
impl std::fmt::Display for PanicPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n", self.message)?;
        writeln!(f, "Backtrace:\n{}", self.backtrace)
    }
}

thread_local! {
    static PANIC_INFO: Cell<Option<PanicPayload>> = const { Cell::new(None) };
}

/// Setup capture backtrace and panic info when panicking
pub fn setup_panic_capture() {
    std::panic::set_hook(Box::new(|info| {
        let backtrace = Backtrace::capture();
        let mut message = match info.location() {
            Some(loc) => format!("file: {}, line: {}\n", loc.file(), loc.line()),
            None => "unknown panic location\n".to_string(),
        };
        // FIXME: unstable
        // match info.payload_as_str() {
        //     Some(x) => message += x,
        //     None => message += "unknown panic info",
        // };
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            message += s;
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            message += s;
        } else {
            message += "unknown panic info"
        }
        PANIC_INFO.with(|b| b.set(Some(PanicPayload { message, backtrace })))
    }));
}

/// Get the last panic payload, only works for panic on the same thread
pub fn take_last_panic() -> PanicPayload {
    PANIC_INFO.with(|b| b.take()).expect("no panic payload captured")
}

pub fn setup_test_process() -> anyhow::Result<Arc<sim::Runtime>> {
    let image_file = std::env::var("SKYBOOK_RUNTIME_TEST_IMAGE")
        .context("please define SKYBOOK_RUNTIME_TEST_IMAGE")?;
    log::info!("loading {image_file}");

    let image_bytes = std::fs::read(image_file).context("failed to read BFI")?;

    let runtime = sim::Runtime::new(exec::Spawner::default());
    runtime.init(&image_bytes, 4, Some(&sim::RuntimeInitParams {
        dlc: 3,
        program_start: "".to_string(),
        stack_start: "0x0000008888800000".to_string(),
        stack_size: 0,
        heap_free_size: 0,
        pmdm_addr: "0x0000002222200000".to_string(),
    })).context("failed to initialize runtime")?;

    Ok(Arc::new(runtime))
}

pub fn collect_memory_trace(process: &Process) -> anyhow::Result<()> {
    log::info!("collecting memory read trace");
    let main_start = process.main_start();
    let main_end = match process.memory().env().game_ver {
        GameVer::X150 => main_start + 0x26af000 - 0x4000,
        GameVer::X160 => main_start + 0x381e000 - 0x4000,
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
