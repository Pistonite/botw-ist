use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use skybook_parser::ParseOutput;
use teleparse::Span;

use crate::error::{ErrorReport, MaybeAborted};
use crate::sim;

pub struct Run {
    /// Handle for the running task
    handle: Arc<RunHandle>,
    /// Data produced by the run
    output: sim::RunOutput,
}

impl Run {
    pub fn new(handle: Arc<RunHandle>) -> Self {
        Run {
            handle,
            output: Default::default(),
        }
    }

    /// Execute the parsed simulation script
    ///
    /// All errors that happened, including internal (e.g. game crash) or
    /// external are collected in the `RunOutput`.
    pub async fn run_parsed(
        self,
        parsed: Arc<ParseOutput>,
        runtime: &sim::Runtime,
    ) -> MaybeAborted<sim::RunOutput> {
        self.run_parsed_with_notify(parsed, runtime,  |_, _|async {vec![]}).await
    }

    /// See [`run_parsed`](Self::run_parsed). In addition, call `notify_fn`
    /// with the end byte pos of the step finished whenever new positions in `notify_at_byte_pos`
    /// are after the end byte pos of a step. The notify function takes the end span
    /// of the last step executed (`up_to_byte_pos`) and the state after the last step.
    /// The notify_fn can return another `notify_at_byte_pos` array which include new byte
    /// positions that should receive notification
    ///
    /// Use this if you need to use partial output before the whole run is finished.
    /// Note that if the run is aborted, then the notification will not be sent after the run is aborted.
    ///
    /// If the position in `notify_at_byte_pos` is at or after the last step is finished,
    /// the notification will not be sent.
    pub async fn run_parsed_with_notify<TFuture, F>(
        mut self,
        parsed: Arc<ParseOutput>,
        runtime: &sim::Runtime,
        // notify_at_byte_pos: &[usize],
        mut notify_fn: F
    ) -> MaybeAborted<sim::RunOutput> 
    where F: FnMut(usize, &sim::RunOutput) -> TFuture,
        TFuture: std::future::Future<Output=Vec<usize>>
    {
        // let mut notify_at_byte_pos = {
        //     let mut temp =  notify_at_byte_pos.to_vec();
        //     temp.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
        //     temp
        // };
        self.output.states.reserve(parsed.steps.len());

        let mut state = sim::State::default();
        let mut commands = Vec::with_capacity(parsed.steps.len());

        for i in 0..parsed.steps.len() {
            let step = &parsed.steps[i];
            let percentage = step.pos as f32 / parsed.script_len as f32 * 100.0;
            log::info!("running: bytes {}/{} ({:.2}%)", step.pos, parsed.script_len, percentage);

            // notify if needed - only if it's not the first step
            if i > 0 {
                // let mut should_notify = false;
                // while let Some(&next) = notify_at_byte_pos.last() {
                //     if next >= step.pos {
                //         break;
                //     }
                //     should_notify = true;
                //     notify_at_byte_pos.pop();
                // }
                // if should_notify {
                    log::info!("notifying at position {}", step.pos);
                    let _new_positions = notify_fn(step.pos, &self.output).await;
                    // if !new_positions.is_empty() {
                    //     log::info!("extending new notification positions: {:?}", new_positions);
                    //     notify_at_byte_pos.extend(new_positions);
                    //     notify_at_byte_pos.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
                    // }
                // }
            }

            commands.push(step.command.clone());
            // skip execution if found in cache
            if let Some((cache_state, cache_errors)) = runtime.find_cached(&commands) {
                self.output.states.push(cache_state.clone());
                self.output.errors.extend(cache_errors);
                state = cache_state;
                continue;
            }

            let span_end = parsed
                .steps
                .get(i + 1)
                .map(|step| step.pos)
                .unwrap_or(parsed.script_len);

            let span = Span::new(step.pos, span_end);

            let report = match state.execute_step(span, step, runtime).await {
                Err(e) => {
                    log::error!("failed to execute step {i}: {e}");
                    if self.handle.is_aborted() {
                        log::warn!("the run is aborted, so the error is ignored");
                        return MaybeAborted::Aborted;
                    }
                    self.output
                        .errors
                        .push(ErrorReport::error(&span, crate::Error::Executor));
                    return MaybeAborted::Ok(self.output);
                }
                Ok(report) => report,
            };

            runtime.set_cache(&commands, &report.value, &report.errors);
            self.output.states.push(report.value.clone());
            self.output.errors.extend(report.errors);
            if self.handle.is_aborted() {
                return MaybeAborted::Aborted;
            }
            state = report.value;
        }

        MaybeAborted::Ok(self.output)
    }
}

/// Handle for a running simulation task.
///
/// See [`Run`] for more information.
#[derive(Default)]
#[repr(transparent)]
pub struct RunHandle {
    is_aborted: AtomicBool,
}

impl RunHandle {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_aborted(&self) -> bool {
        self.is_aborted.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn abort(&self) {
        self.is_aborted
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
    /// Convert the handle into a raw pointer to interface with external code
    pub fn into_raw(s: Arc<Self>) -> *const Self {
        Arc::into_raw(s)
    }
    /// Convert the handle from a raw pointer back into rust object.
    ///
    /// The pointer must be one that's previously [`into_raw`](Self::into_raw)-ed
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn from_raw(ptr: *const Self) -> Arc<Self> {
        if ptr.is_null() {
            // make sure it's safe
            return Arc::new(Self::new());
        }
        unsafe { Arc::from_raw(ptr) }
    }
}
