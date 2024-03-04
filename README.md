# botw-ist
Simulator for Inventory Slot Transfer in BOTW

Visit the app at https://ist.itntpiston.app

**Notice**

Bug fixes and improvements for the current simulation core will not be continued because of the intrinsic difficulty with the core
to accurately simulate the remaining cases. A rewrite of the core is planned but not actively worked on at the moment. Please still open issues
if you want, but don't expect it to be fixed until much later


## Contribute
Contributions are welcomed. For small issues, you can open PRs directly. For big issues contact me on discord (username: Pistonight)

## Development
### Clone Repo
Clone repo with `--recurse-submodules`

If you have already cloned, run `git submodule update --init --recursive`

### Install/Update
To update submodules, go to the submodule directory and pull. Example:
```
cd scripts/base-lint
git pull
cd scripts/typescript-layers
git pull
```

To install node modules `npm install`

### Local
Run `npm run start` to start webpack dev server.
When code changes, the dev server will hot reload

### Grammar
The simulator uses a LL parser with infinite look ahead to generate an Abstract Syntax Tree from which commands are parsed.

The grammar is at `src/core/command/ast/grammar.txt` and the ast parser is generated with `npm run generate` (Python needed)

If you want to introduce new commands, most of the heavy lifting for the parsers is already done. You should be able to define a new derivation in the grammar and implement the parser by looking at one of the `parse.cmd.*` files. However, you should probably contact the maintainer to have a discussion about the new command.

### PR
Do before PR:
- Lint your code
  1. `npm run lint-base`: This checks that your files have unix line endings, have no traling whitespaces, and have exactly 1 trailing new line. This might fail if you have auto crlf on git for windows. If you do, **please make sure the remote still has UNIX line ending so the PR automation passes**. To debug unexpected errors, run again as `npm run lint-base -- -v` to see which file is failing
  - `npm run layer`: This makes sure your imports follow the layer rules (and are sorted correctly)
    - `src/data` is the bottom layer. It cannot depend on core or ui components
    - `src/core` is the core logic. It can depend on data, but not ui
    - `src/ui` is the ui layer. It can depend on everything
  - `npm run lint-ts`: This is the standard eslint
- Run tests `npm run test-all`
