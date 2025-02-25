# Contributing
This page contains important information for contributing for the project.
You should read everything here before thinking about contributing.

## Allowed Files
The repository must not contain any assets or data from BOTW, or any file
derived from assets or data from the game. PRs containing such files
will be closed.
```admonish info
Please reach out to me if you want to add something that depends on
the game files
```

## Requirement
The following tools are required:
- [Rust Toolchain](https://rustup.rs)
  - `MSVC` on Windows is likely needed (Install Visual Studio Build Tools)
- Node v20
- [PNPM](https://pnpm.io/installation)
- [Bun](https://https://bun.sh/docs/installation)
- Python v3
  - Pip packages: `pip install pyyaml`
- [Task](https://taskfile.dev)
- [Magoo](https://github.com/Pistonite/magoo) (`cargo install magoo`)
- `jq` is only needed for some workflows

For developing the runtime, you also need a copy of the game.

Development workflows are only tested (by me) on Linux, for Windows, there are a few extra requirements
to get everything working. The recommended option is use WSL unless you enjoy tinkering
with these stuff:
- PowerShell 7 is recommended since it doesn't have weird aliases like PowerShell 5 (such as `curl`)
- MSVC: See https://rust-lang.github.io/rustup/installation/windows-msvc.html
- GNU Coreutils
  - You might need to replace some shell alias/commands with the GNU version
- GNU `wget` and `sed` - Both should have Windows binary release
- `which` - You can use my implementation `cargo install dotbin-windows --git https://github.com/Pistonite/dotbin`

It's also a good idea to join my [Discord](../welcome.md#discord)
in case you have questions or need to discuss something.

## Setup
```admonish danger
These steps don't work yet
```
You only need to run these steps once to setup the development environment

1. Install all the tools as listed above
2. Clone the `Pistonite/botw-ist` repository from GitHub
   ```
   git clone git@github.com:Pistonite/botw-ist
   ```
3. Run the commands below to setup the dependencies
   ```
   magoo install
   task exec -- research:install
   task install-cargo-extra-tools
   task build-artifacts
   task install
   ```

Now, you can run the web app in development mode. This is enough
if you are not planning on changing the Runtime. 

If you do need to build the Runtime, you need to:
1. Follow the steps [here](https://github.com/Pistonight/symbotw/tree/main/packages/uking-relocate)
to acquire a BlueFlame image with the `uking-relocate` tool. 
2. Rename the image `program.blfm` and put it in `<repo>/packages/runtime/`
3. Run `task exec -- runtime-wasm:build`
```admonish note
Everytime the parser or runtime is changed, you need to run the `runtime-wasm:build`
task to re-build the WASM module to see the changes in the web app
```

## Keeping Up-to-date
If you are a returning contributor, it's always a good idea to check this page again
in case the requirements change.

You generally only need to run `task install` to install the dependencies again.
If you are returning after a long period of time (> 1 month), it's a good idea to
run `rustup update && cargo clean && task icets` to update the Rust toolchain
and tools.

## Repo Structure
All the code are organized into packages in the `packages` directory,
you should only need to care about what's inside `packages`.

You can list all the packages with:
```
ls packages
```
To see what tasks are available, run:
```
task list
```
To execute an task from the repo root, run:
```
task <task> # replace <task> with the task to execute
```
To list and execute a task from a package, you can run:
```
task list -- <package>
task exec -- <package>:<task>

# OR:

cd packages/<package>
task --list
task <task>
```
```admonish tip
The most common use case is to automatically fix formatting issues.
For example `task exec -- parser:fix` will fix all formatting issues with the parser
```

## Build and Testing
TODO

## PR Guidelines
Please use these guidelines to ensure your PR can be effectively reviewed and merged.

1. **Make small changes**: Small fixes can be reviewed faster. Please reach out first if you
   are planning on a moderate or big change, or adding something new
2. **Run checks locally**: PRs must pass the workflows to be merged, and the workflows require
   my approval to run. To reduce the turn-around time, you should always run the checks locally to make sure the workflows
   pass. To run the checks:
      ```
      task check
      task test
      task build
      ```
3. **Test**: You almost always want to add unit tests for your change. If something should be
   tested but no new tests are added, I will ask for it. See above for how each component can be
   tested. If you are changing the UI, consider adding screenshots for before and after, if it's not a simple fix.
4. **Documentation**: If you are adding new functions, make sure it has a doc comment. The comment should detail
   the behavior of the function. i.e. I should be able to tell what the function returns for an input, without looking
   at the implementation

