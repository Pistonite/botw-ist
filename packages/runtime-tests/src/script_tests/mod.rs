use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use anyhow::Context;
use skybook_parser::ParseOutput;
use skybook_parser::search::QuotedItemResolver;
use skybook_parser::search::ResolvedItem;
use skybook_runtime::sim;

pub fn run(runtime: Arc<sim::Runtime>) -> anyhow::Result<bool> {
    log::info!("running script tests");

    let snapshots_dir = Path::new("snapshots");
    if !snapshots_dir.exists() {
        std::fs::create_dir_all(snapshots_dir).context("failed to create snapshots directory")?;
    }

    let refresh_snapshot = !std::env::var("SKYBOOK_RUNTIME_TEST_REFRESH")
        .unwrap_or_default()
        .is_empty();
    if refresh_snapshot {
        log::info!("will refresh snapshot");
    }

    let mut test_names = vec![];
    let test_dir = std::fs::read_dir("src/script_tests").context("failed to read script dir")?;
    for entry in test_dir {
        let Ok(entry) = entry else {
            continue;
        };
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if !file_name.ends_with(".txt") {
            continue;
        }
        let test_name = file_name
            .strip_suffix(".txt")
            .expect("test should end with .txt")
            .to_string();
        test_names.push(test_name);
    }

    let total_count = test_names.len();
    let passed_count = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to create tokio runtime")?
        .block_on(async { run_tests(runtime, test_names, refresh_snapshot).await })
        .context("there were failures running the script tests")?;

    log::info!("{passed_count}/{total_count} script tests passed");

    Ok(passed_count == total_count)
}

async fn run_tests(
    runtime: Arc<sim::Runtime>,
    test_names: Vec<String>,
    refresh: bool,
) -> anyhow::Result<usize> {
    let mut handles = vec![];
    for test in test_names {
        let test_file = std::fs::read_to_string(format!("src/script_tests/{test}.txt"))
            .context("cannot read test file")?;
        let parsed = skybook_parser::parse(&StubQuotedItemResolver, &test_file).await;
        if ENCOUNTERED_QUOTED_SEARCH.load(std::sync::atomic::Ordering::SeqCst) {
            ENCOUNTERED_QUOTED_SEARCH.store(false, std::sync::atomic::Ordering::SeqCst);
            log::error!("FAIL {test} - quoted item search not supported");
            continue;
        };

        let parsed = Arc::new(parsed);
        let runtime = Arc::clone(&runtime);
        let handle =
            tokio::spawn(
                async move { run_test(&runtime, refresh, &test, &test_file, parsed).await },
            );
        handles.push(handle);
    }

    let mut passed_count = 0;
    for handle in handles {
        match handle.await {
            Err(e) => {
                log::error!("join failed: {e}");
            }
            Ok(Err(e)) => {
                log::error!("error occured while running test: {e}");
            }
            Ok(Ok(passed)) => {
                if passed {
                    passed_count += 1;
                }
            }
        }
    }

    Ok(passed_count)
}

static ENCOUNTERED_QUOTED_SEARCH: AtomicBool = AtomicBool::new(false);

struct StubQuotedItemResolver;
impl QuotedItemResolver for StubQuotedItemResolver {
    type Future = Pin<Box<dyn Future<Output = Option<ResolvedItem>>>>;

    fn resolve_quoted(&self, word: &str) -> Self::Future {
        log::error!("quoted item search is not supported in tests, searching: {word}");
        ENCOUNTERED_QUOTED_SEARCH.store(true, std::sync::atomic::Ordering::Relaxed);
        Box::pin(async { None })
    }
}

async fn run_test(
    runtime: &sim::Runtime,
    refresh: bool,
    test_name: &str,
    test_script: &str,
    parsed_output: Arc<ParseOutput>,
) -> anyhow::Result<bool> {
    let mut new_snapshot = String::from(
        "// This has RS extension since that usually gives a minimal syntax highlighting.\n//This is not an actual RS file\n\nx!{ SKYBOOK RUNTIME SNAPSHOT V1\n\n",
    );

    let run_handle = sim::RunHandle::new();
    let run = sim::Run::new(Arc::new(run_handle));
    // unwrap: we will never abort the run so it will always be finished
    let output = run
        .run_parsed(Arc::clone(&parsed_output), runtime)
        .await
        .unwrap();

    // also write diagnostics into output
    for error in output.errors {
        let prefix = if error.is_warning {
            "warning: "
        } else {
            "error: "
        };
        new_snapshot += &format!("{}: {}\n", prefix, error.error);
        new_snapshot += &format!("  span: {}..{}\n", error.span.0, error.span.1);
        new_snapshot += "-----\n";
        new_snapshot += &test_script[error.span.0..error.span.1];
        new_snapshot += "-----\n";
    }

    new_snapshot += "=====\n";

    for (i, step) in parsed_output.steps.iter().enumerate() {
        let span = step.span;
        new_snapshot += "\n";
        let script = &test_script[span.lo..span.hi];
        new_snapshot += &format!("----- Step[{i}]: {}", script);
        new_snapshot += "\n";

        let state = &output.states[i];
        let snapshot = state.to_snapshot();
        new_snapshot += &snapshot.to_string();
    }

    new_snapshot += "\n}\n";

    let snapshot_file_path = PathBuf::from(format!("snapshots/{test_name}.snap.rs"));
    if refresh || !snapshot_file_path.exists() {
        std::fs::write(snapshot_file_path, new_snapshot).context("failed to write snapshot")?;
        log::info!("UPDATE {test_name}");
        return Ok(true);
    }

    let old_snapshot_content =
        std::fs::read_to_string(snapshot_file_path).context("failed to read snapshot")?;

    if old_snapshot_content != new_snapshot {
        log::error!("FAIL {test_name}");
        let wip_dir = Path::new("snapshots/wip");
        if !wip_dir.exists() {
            std::fs::create_dir_all(wip_dir).context("cannot create wip directory")?;
        }
        std::fs::write(format!("snapshots/wip/{test_name}.snap.rs"), new_snapshot)
            .context("cannot write wip snapshot")?;
        return Ok(false);
    }

    log::info!("PASS {test_name}");
    Ok(true)
}
