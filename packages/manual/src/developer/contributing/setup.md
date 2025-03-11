# Setup

```admonish tip
If you are only editing files such as test cases or translations,
then you might not need to setup tools for development
```
```admonish info
Before starting the setup, follow the [`mono-dev` Standard](https://mono.pistonite.dev/standard.html)
to setup the required tools:
- Rust Toolchain
- Node, PNPM, and Bun
- Python
- Task
- Magoo

WSL is recommended for Windows.
```

For first-time setup, run the following commands
```
git clone git@github.com:Pistonite/botw-ist
cd botw-ist
magoo install
task exec -- research:install
task install-cargo-extra-tools
task build-artifacts
task install
task check
```

```admonish warning
`task check` generates config files for IDE and build tools and 
is not optional!
```

You also just need to run `task install` and sometimes `task check` after merging others' work from the `main`
branches.

The setup above will let you build and run the web app without building
the Runtime locally. To build the Runtime, you need to set up a [BlueFlame image](../../user/custom_image.md)
in addition to the steps above.

Rename your image `program.blfm` and put it under `/packages/runtime/`, then run:
```
task exec runtime-wasm:build
```

Now the local build of the app will use the locally built Runtime.
