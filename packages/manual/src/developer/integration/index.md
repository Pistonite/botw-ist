# Integration

There are 2 ways to integrate the IST simulator into your application:
- With a Custom Extension in TypeScript
- By using the Rust Crate (Not Ready Yet)

A TypeScript Custom Extension is a web app that can be opened
as a popout from the IST Simulator app. Your extension and the main app
can then talk to each other through window messaging.
If you are already familiar with frontend web development,
this is the best way to integrate the IST simulator, as all the simulator
internals like parsing and runtime scheduling are wrapped in the extension API.

If a Custom Extension does not fit your use case, you can use the Rust Crate,
which exposes all simulator internals, such as the BlueFlame interpreter,
game memory and states, and script parsing.

The Rust Crate is not ready at the moment. Please reach out on Discord if you need it.
