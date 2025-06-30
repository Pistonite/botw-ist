# Getting Started

The first step to contributing is to setup a development environment locally
on your PC.

I aim to make the setup process as streamlined as possible. If you encounter
any issues, please feel free to reach out and suggest to me how it can be improved!

```admonish info
Before starting the setup, follow the [`mono-dev` Standard](https://mono.pistonite.dev/standard.html)
to setup the required tools:
- Rust Toolchain
- Node, PNPM, and Bun
- Python
- Task
- Magoo

Coreutils is required for Windows development
```

## Clone repository and one-time setup

Run the following commands
```
git clone git@github.com:Pistonite/botw-ist --depth 1
cd botw-ist
magoo install
task exec -- research:install
task install-cargo-extra-tools
task build-artifacts
task install
task check
```

This will:
- Clone the repository to your PC using your GitHub account
  - If you don't have GitHub account or don't have SSH key setup, use
    `https://github.com/Pistonite/botw-ist` as the URL instead
- `magoo` will setup the submodules for you
- Research scripts will be ran to ensure data files are setup
- Data artifacts will be built from the data files
- Dependency packages will be downloaded
- Configuration files will be generated

## Keeping up-to-date
After pulling, you need to update the repo locally to sync tools to the latest state.

Run:
```
task install
```

That's it!

## 

The setup above will let you build and run the web app without building
the Runtime locally. To build the Runtime, you need to set up a [BlueFlame image](../../user/custom_image.md)
in addition to the steps above.

Rename your image `program.blfm` and put it under `/packages/runtime/`, then run:
```
task exec runtime-wasm:build
```

Now the local build of the app will use the locally built Runtime.
