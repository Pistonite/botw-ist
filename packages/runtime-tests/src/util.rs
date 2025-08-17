use std::backtrace::Backtrace;
use std::cell::Cell;
use std::path::Path;
use std::sync::Arc;

use blueflame::env::GameVer;
use blueflame::processor::Process;
use cu::pre::*;
use sha2::{Digest, Sha256};
use skybook_runtime::{exec, sim};

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
        let payload = PanicPayload { message, backtrace };
        cu::error!("panic: {payload}");
        PANIC_INFO.with(|b| b.set(Some(payload)))
    }));
}

/// Get the last panic payload, only works for panic on the same thread
pub fn take_last_panic() -> PanicPayload {
    PANIC_INFO
        .with(|b| b.take())
        .expect("no panic payload captured")
}

pub fn setup_test_process(image_file: &str) -> cu::Result<Arc<sim::Runtime>> {
    cu::info!("loading {image_file}");

    let image_bytes = cu::fs::read(image_file).context("failed to read BFI")?;

    let runtime = sim::Runtime::new(exec::Spawner::new()?);
    let threads = if cfg!(feature = "single-thread") {
        1
    } else {
        // 2
        // leave 1 thread for tokio runtime
        num_cpus::get().clamp(2, 9) - 1
    };
    runtime
        .init(
            &image_bytes,
            threads,
            Some(&sim::RuntimeInitParams {
                dlc: 3,
                program_start: "".to_string(),
                stack_start: "0x0000008888800000".to_string(),
                stack_size: 0,
                heap_free_size: 0,
                pmdm_addr: "0x0000002222200000".to_string(),
            }),
        )
        .context("failed to initialize runtime")?;

    Ok(Arc::new(runtime))
}

#[cfg(feature = "trace-memory")]
pub fn collect_memory_trace(process: &Process, update_hash: bool) -> cu::Result<()> {
    cu::info!("collecting memory read trace");
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

    let mut hash = Sha256::new();
    hash.update(read_report.as_bytes());
    let new_hash = hash
        .finalize()
        .into_iter()
        .map(|x| format!("{x:02x}"))
        .collect::<String>();
    let mut hash_changed = true;
    let hash_file = Path::new("trace-hash.txt");
    if hash_file.exists() {
        if let Ok(old_hash) = std::fs::read_to_string(hash_file) {
            if old_hash.trim() == new_hash.trim() {
                hash_changed = false;
            }
        }
    }
    if update_hash {
        cu::info!("updating trace-hash");
        cu::fs::write(hash_file, new_hash)?;
    }
    cu::fs::write("trace.txt", read_report)?;

    if hash_changed {
        if !update_hash {
            cu::bail!(
                "the trace hash is generated or changed, please re-generate and push the mini image to artifacts"
            );
        } else {
            cu::warn!(
                "the hash file is updated now. Make sure you build and push the mini image later, otherwise the hash will be incorrect!"
            );
        }
    }

    Ok(())
}
