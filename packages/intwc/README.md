# intwc
I-Need-To-Write-Code

Code editor wrapper component for writing code in a browser for my projects.

Currently, it only supports Vite + React.

## Features
- Syntax highlighting and validation
- TypeScript custom type library and type checking
- Support for a custom language:
  - semantic token
  - completion
  - diagnostics
  - definition
- Tuned Cattppuccin theme for supported languages
- Vim and Emacs mode

## Non-features
- Project-wide TypeScript language features (e.g. go to definition, find references)
- ES Modules
- Run sandboxed TypeScript code (will be another project)

## Tech
Under the hood, this project uses [monaco-editor](https://github.com/microsoft/monaco-editor),
which is VS Code with shims to run in a browser. This ensures
the code editor has all the basic features, including accessbility.
This project does not use the existing wrappers/replacements for monaco-editor that adds support
for VSCode API. As a result, it has much less complexity at the cost
of not having the full feature and extension support of VS Code.

## TODO
- [ ] Add multiple file support
