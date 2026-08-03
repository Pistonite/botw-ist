# Getting Started


> [!IMPORTANT]
> Please refer to [`mono-dev`](https://mono.pistonite.dev/standard.html)
> for contributing guidelines and tools setup for my projects.
>
> Please setup tools for Rust, TypeScript, and Python. The instructions are
> in the `mono-dev` link above.

The first step to contributing is to setup a development environment locally
on your PC.

I aim to make the setup process as streamlined as possible. If you encounter
any issues, please feel free to reach out and suggest to me how it can be improved!

## Clone repository and one-time setup

Assuming you have the repo cloned already, 
run the following commands from the repo root

```
magoo install
task setup
task install
task build-artifacts
```

Breakdown:
- `magoo install` checks out the submodules
- `task setup` performs one time setup, including build additional tools required
  with `cargo`
- `task install` installs the dependencies and runs post-install scripts.
- `task build-artifacts` generates required artifacts for development locally.

> [!TIP]
> When updating your local copy of the repo (i.e. with `git pull`),
> run `task install` again to stay up-to-date with the latest dependencies.
> You do not need to run the other setups in most cases.

Then proceed to [Build and run](./run.md) to start development!

## Building Runtime

If you are only working on the frontend UI, or the manual (this website),
then you don't need to do this setup for building the runtime.
The project is setup to automatically download the runtime from the hosted app
and use that for local development.

However if you do need to make changes to the runtime:
- Make sure you already ran the commands above to build other artifacts as they are needed
  to run the integration tests.
- Run `task exec -- runtime-tests:install` to install `uking-relocate`, an experimental tool
  for making BlueFlame images
- Obtain a dump of the game's ExeFS, decompress the NSO (with hactool) and convert it to ELF (with nx2elf).
- Put the ExeFS at `packages/runtime-tests/data/botw150/`. Name the files with `.elf` suffix.
- You need one extra file `Actor/ActorInfo.product.sbyml` from RomFS. Obtain the file and put it
  at `packages/runtime-tests/data/botw150/romfs/Actor/ActorInfo.product.sbyml`.
- Run `task exec -- runtime-tests:build-mini` to generate `program-mini.bfi`
- Now the WASM runtime should build: `task exec -- runtime-wasm:build`

Then proceed to [Build and run](./run.md) to start development!
