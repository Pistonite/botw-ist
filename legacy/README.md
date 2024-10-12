# botw-ist
Simulator for Inventory Slot Transfer in BOTW

Visit the app at https://ist.itntpiston.app

**Notice**

Bug fixes and improvements for the current simulation core will not be continued because of the intrinsic difficulty with the core
to accurately simulate the remaining cases. A rewrite of the core is actively being developed and should be expected to be ready by June 2025


## Contribute
Contributions are welcomed. For small issues, you can open PRs directly. For big issues contact me on discord (username: Pistonight)

## Development
[task](https://taskfile.dev) is recommended to run provided scripts. You can also look at `Taskfile.yml`
and run the scripts manually.

Node v18 and Python 3 are required.

### Install
Run `task install` to install dependencies

### Local
Run `task dev` to start vite dev server. Some features require secure context. 
The dev server is configured to look for `cert/cert.pem` and `cert/cert.pfx` for HTTPS certificates.

### Grammar
The simulator uses a LL parser with infinite look ahead to generate an Abstract Syntax Tree from which commands are parsed.

The grammar is at `src/core/command/ast/grammar.txt` and the ast parser is generated with `npm run generate` (Python needed)

If you want to introduce new commands, most of the heavy lifting for the parsers is already done. You should be able to define a new derivation in the grammar and implement the parser by looking at one of the `parse.cmd.*` files. However, you should probably contact the maintainer to have a discussion about the new command.

### Test
Run `task test --watch` to run tests in watch mode

### PR
Run `task check` and `task test` before PR.

