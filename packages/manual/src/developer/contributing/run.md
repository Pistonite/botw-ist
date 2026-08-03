# Build and Run

> [!NOTE]
> Please see [Getting Started](./setup.md) to setup the development environment first
> before following the steps here

## Web Application
> [!WARNING]
> Running the web app locally requires Secure Origin, which means:
> - You have to access the app through `http://localhost` using the same machine
>   the dev server is running from, or
> - You have to setup your browser to trust HTTPS connections from the app.
>
> The first method is going to be easier. If you prefer the second method please reach out to me

To run the web application:

```
task exec -- app:dev
```

The UI will automatically reload as you make changes.

Note that DirectLoad (embed a script directly in the url) will not work when running the application locally since
it's a server feature.

## Manual
To run the manual (this website):

```
task exec -- manual:dev
```

The manual will automatically reload when making changes.

## Bindings
`skybook-api` is the package with the bindings needed for the application
to talk to the runtime. if you changed the binding, run
```
task exec -- skybook-api:build
```
This will rebuild the binding as well as the skybook-api JS package.

## Server
> [!WARNING]
> Currently, running the server locally also requires building the runtime as
> part of the assets, which requires a dump of the game to build.
> See [Building Runtime](./setup.md)

To run the server, first build and pull the application assets locally:

```
task exec -- server:pull-local
```

Then the dev workflow can be started with:

```
task exec -- server:dev
```

Changes in the server code will reload the server automatically. However,
changes in the client code requires re-running the `pull-local` task to rebuild
the client.

## Runtime
> [!WARNING]
> Building the runtime requires a dump of the game.
> See [Building Runtime](./setup.md)

To build the runtime WASM module, run
```
task exec -- runtime-wasm:build
```
Refresh the application (or `task exec -- app:dev` to start it), it will now
use the local runtime you built.
