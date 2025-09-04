use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use cu::pre::*;

use skybook_parser::ParseOutput;
use skybook_parser::cir;
use skybook_runtime::MaybeAborted;
use skybook_runtime::sim;

pub fn run(
    runtime: Arc<sim::Runtime>,
    refresh_snapshot: bool,
    only_test: Option<String>,
) -> cu::Result<bool> {
    cu::debug!("running script tests");

    let snapshots_dir = Path::new("snapshots");
    cu::fs::make_dir(snapshots_dir)?;

    if refresh_snapshot {
        cu::info!("will refresh snapshot");
    }

    let mut test_names = vec![];

    if let Some(only_test) = only_test {
        cu::info!("only testing {only_test}");
        test_names.push(only_test);
    } else {
        let test_dir =
            std::fs::read_dir("src/script_tests").context("failed to read script dir")?;
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
    }

    let total_count = test_names.len();
    let passed_count =
        cu::co::run(async move { run_tests(runtime, test_names, refresh_snapshot).await })
            .context("there were failures running script tests")?;

    cu::info!("{passed_count}/{total_count} script tests passed");

    Ok(passed_count == total_count)
}


async fn run_tests(
    runtime: Arc<sim::Runtime>,
    test_names: Vec<String>,
    refresh: bool,
) -> cu::Result<usize> {
    let mut handles = vec![];
    let total_count = test_names.len();
    let bar = cu::progress_bar(total_count, "script tests");
    for test in test_names {
        let test_file = std::fs::read_to_string(format!("src/script_tests/{test}.txt"))
            .context("cannot read test file")?;
        let resolver = StubQuotedItemResolver(AtomicBool::new(false));
        let parsed = skybook_parser::parse(&resolver, &test_file).await;
        let encountered_quoted_search = resolver.0.load(std::sync::atomic::Ordering::Acquire);
        if encountered_quoted_search {
            cu::error!("FAIL {test} - quoted item search not supported");
            continue;
        };

        let parsed = Arc::new(parsed);
        let runtime = Arc::clone(&runtime);
        let handle =
            cu::co::spawn(
                async move { run_test(&runtime, refresh, &test, &test_file, parsed).await },
            );
        handles.push(handle);
    }

    let mut handles = cu::co::set(handles);

    let mut passed_count = 0;
    let mut finished_count = 0;

    while let Some(result) = handles.next().await {
        finished_count += 1;
        match result {
            Err(e) => {
                cu::error!("join failed: {e}");
            }
            Ok(Err(e)) => {
                cu::error!("error occured while running test: {e}");
            }
            Ok(Ok(passed)) => {
                if passed {
                    passed_count += 1;
                }
            }
        }
        let failed_count = finished_count - passed_count;
        cu::progress!(&bar, finished_count, "{failed_count} failed");
    }

    Ok(passed_count)
}


struct StubQuotedItemResolver(AtomicBool);
impl cir::QuotedItemResolver for StubQuotedItemResolver {
    type Future = cu::BoxedFuture<Option<cir::ResolvedItem>>;

    fn resolve_quoted(&self, word: &str) -> Self::Future {
        cu::error!("quoted item search is not supported in tests, searching: {word}");
        self.0.store(true, std::sync::atomic::Ordering::Release);
        Box::pin(async { None })
    }
}

async fn run_test(
    runtime: &sim::Runtime,
    refresh: bool,
    test_name: &str,
    test_script: &str,
    parsed_output: Arc<ParseOutput>,
) -> cu::Result<bool> {
    cu::debug!("TESTING\n{test_script}");
    let mut new_snapshot = String::from(
        "// This has RS extension since that usually gives a minimal syntax highlighting.\n//This is not an actual RS file\n\nx!{ SKYBOOK RUNTIME SNAPSHOT V1\n\n",
    );

    let run_handle = sim::RunHandle::new();
    let run = sim::Run::new(Arc::new(run_handle));
    // unwrap: we will never abort the run so it will always be finished
    let MaybeAborted::Ok(output) = run.run_parsed(&parsed_output, runtime).await else {
        cu::error!("CANCEL {test_name}");
        return Ok(false);
    };

    // also write diagnostics into output
    for error in &parsed_output.errors {
        let prefix = if error.is_warning {
            "parse warning: "
        } else {
            "parse error: "
        };
        new_snapshot += &format!("{}: {}\n", prefix, error.error);
        new_snapshot += &format!("  span: {}..{}\n", error.span.0, error.span.1);
        new_snapshot += "-----\n";
        new_snapshot += &test_script[error.span.0..error.span.1];
        new_snapshot += "\n-----\n";
    }
    for error in output.errors {
        let prefix = if error.is_warning {
            "runtime warning: "
        } else {
            "runtime error: "
        };
        new_snapshot += &format!("{}: {}\n", prefix, error.error);
        new_snapshot += &format!("  span: {}..{}\n", error.span.0, error.span.1);
        new_snapshot += "-----\n";
        new_snapshot += &test_script[error.span.0..error.span.1];
        new_snapshot += "\n-----\n";
    }

    new_snapshot += "=====\n";

    let mut previous_snapshot: Option<sim::StateSnapshot> = None;

    for (i, step) in parsed_output.steps.iter().enumerate() {
        let span = step.span();
        new_snapshot += "\n";
        let script = &test_script[span.lo..span.hi];
        new_snapshot += &format!("----- Step[{i}]: {script}");
        new_snapshot += "\n\n";

        let state = &output.states[i];
        let snapshot = state.to_snapshot();
        if let Some(previous) = &previous_snapshot
            && previous == &snapshot
        {
            new_snapshot += "<same>";
        } else {
            new_snapshot += &snapshot.to_string();
            previous_snapshot = Some(snapshot);
        }
    }

    new_snapshot += "\n}\n";

    let snapshot_file_path = PathBuf::from(format!("snapshots/{test_name}.snap.rs"));
    if refresh || !snapshot_file_path.exists() {
        std::fs::write(snapshot_file_path, new_snapshot).context("failed to write snapshot")?;
        cu::info!("UPDATE {test_name}");
        return Ok(true);
    }

    let old_snapshot_content =
        std::fs::read_to_string(snapshot_file_path).context("failed to read snapshot")?;

    if old_snapshot_content != new_snapshot {
        cu::error!("FAIL {test_name}");
        let wip_dir = Path::new("snapshots/wip");
        if !wip_dir.exists() {
            std::fs::create_dir_all(wip_dir).context("cannot create wip directory")?;
        }
        std::fs::write(format!("snapshots/wip/{test_name}.snap.rs"), new_snapshot)
            .context("cannot write wip snapshot")?;
        return Ok(false);
    }

    cu::info!("PASS {test_name}");
    Ok(true)
}
