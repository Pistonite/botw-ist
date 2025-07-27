# Checking Testing

You should make sure your changes are properly checked and tested.

Most of the time, this means you should at least manually tested the change,
and if needed, add at least one [Snapshot Test](#snapshot-tests) if the runtime
is changed.

## Checking
Running `task check` in the root of the repo will run a handful of checks and lints.
You can also run `task exec -- XXX:check` to only run checks for one package.

When there are formatting issues, `task exec -- XXX:fix` can automatically fix that.

### `skybook-api` modification checks
If you changed code that affects the generated `skybook-api` code, you need
to commit those changes to git for `skybook-api` checks to pass.

### `localization` checks
Certain changes require adding keys to localization (e.g. new error types).
Please reach out to me for adding localizations.

The modification check also requires the changes to be committed to pass.

## Unit and Manual Tests
We use [Vitest](https://vitest.dev/) for unit testing TypeScript code
and standard Cargo test for Rust.

Also make sure to test your changes manually in the app. See [Build and Run](./run.md)
for how to run the app locally.

## Snapshot Tests
Snapshot tests are the most important tests in the project.
These tests are simulator scripts that the test tool runs and captures
the state at each step, and saves them to a text file in a human readable format. Any diff
in the snapshot is considered a failure.

To add a snapshot test, put a `.txt` file containing the script in `/packages/runtime-tests/src/script_tests/`,
then run `task exec -- runtime-tests:run-full` (or `run-mini`).
This will generate a new snapshot in `snapshots/`. Open the file
and make sure the state is what you expect at every step.

If a snapshot test fails, the new snapshot will be saved to `snapshots/wip`.
You can diff the snapshot with `task diff -- NAME_OF_TEST`. (Requires
the [`delta`](https://github.com/dandavison/delta) tool).
When you think the new snapshot is ready, copy it to `snapshots` and replace the old snapshot.

You can also run the `ust` task to update all snapshots.
