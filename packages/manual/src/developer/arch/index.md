# Architecture

On the architectural level, Skybook can be divided into 5 layers 
(or components, whatever you want to call them):

- The **`Core`**: The Core is named **`BlueFlame`**. This layer is essentially a stripped-down, mini-emulator
  that interprets the game's code. The core's functionality is not limited
  to simulating IST, but can be expanded to other areas of the game if enough research
  is put into it.
- The **`Runtime`**: The Runtime layer parses the script into steps and keeps track
  of the memory state of those steps in a simulation. It orchestrates the core for most
  of the simulation, and implements some sub-systems that are not supported by the core.
  The Runtime layer also manages multiple Cores to utilize multiple processors efficiently.
- The **`Runtime Worker`**: Or simply the Worker layer, is the front-end of the Runtime.
  The Runtime layer is implemented in Rust, which is bad at handling async processes and requests.
  However, TypeScript is very good at that. So, the Worker layer essentially wraps the native interface
  with a TypeScript interface that the Application works with.
- The **`Application`**: This is the main UI of the simulator web app, such as Pouch and GameData display.
- The **`Extensions`**: The extensions are extra UI widgets that interface with the Application
  to easily add new features without changing the underlying architecture.

For the web application, there is also a server that handles DirectLoad - 
loading script from another source such as GitHub or embedded URL. The
server doesn't really have anything to do with the core functionality, so it's
not discussed here.
