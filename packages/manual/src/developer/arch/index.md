# Architecture

Skybook can be divided conceptually into 5 layers (or systems, or components,
whatever you want to call them):

- The **`Core`**: The Core is named **`BlueFlame`**. This layer is essentially a stripped-down, mini-emulator
  that interprets the game's code. The core's functionality is not limited
  to simulating IST, but can be expanded to other areas of the game if enough research
  is put into it.
- The **`Runtime`**: The Runtime layer parses the script into steps and keeps track
  of the memory state of those steps in a simulation. It orchestrates the core for most
  of the simulation, and implements some sub-systems that are not supported by the core.
  The Runtime layer also manages multiple Cores to utilize multiple processors efficiently.
- The **`Worker`**: This is the front-end of the Runtime.
  The Runtime layer is implemented in Rust, which is bad at handling async processes and requests.
  However, TypeScript is very good at that. So, the Worker layer essentially wraps the native interface
  with a TypeScript interface that the Application works with.
- The **`Application`**: This is the main UI of the simulator web app, such as Pouch and GameData display.
- The **`Extensions`**: The extensions are extra UI widgets that interface with the Application
  to easily add new features without changing the underlying architecture.

```admonish info
For the web application, there is also a server that handles DirectLoad - 
loading script from another source such as GitHub or embedded URL. The
server doesn't really have anything to do with the core functionality, so it's
not discussed here.
```

These layers can be composed based on different requirements for endpoints.
For example, the configuration for the web app is:
- `Core` and `Runtime` are built into WASM
- `Worker` is bundled into a WebWorker that loads the WASM
- `Application` is bundled into the main entry point `index.html`
- `Extensions` are both bundled into the main entry point (for displaying
   directly in the page), and secondary entry point `popout.html` for displaying
   as popouts.

## Restricted imports in the UI Application
To simplify bundling, all UI code is in the `app` package.
However, be careful when importing from a different part of the package,
since that can affect the code splitting for bundling.

Current endpoints and what they import:
- `main`:
  - `application`
  - `extensions`
  - `ui/components`
  - `ui/surfaces`
- `popout`:
  - `extensions`
  - `ui/components`
