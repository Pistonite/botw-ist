# Build and Run

```admonish tip
For all the commands like this:
   
    task exec -- app:dev

You can also `cd` to the package and execute the task:

    cd packages/app
    task dev


```


## Web Application
To run the web application:
```
task exec -- app:dev
```
The app will generally automatically hot-reload as you make changes, except:
- Manual reload is needed if the `Script Editor` extension or related code is modified
  to re-enable syntax highlighting
- Building the Runtime worker and manual reload is needed for changes in the Runtime worker.
  Run `task exec -- runtime-wasm:build` to rebuild, then refresh the page.

Note that DirectLoad will not work when running the application like this since
it's a server feature.

## Manual
To run the manual (this website):
```
task exec -- manual:dev
```
The manual will automatically reload when making changes

## Server
To run the server, first build and pull the application assets locally
```
task exec -- server:pull-local
```
Then the dev workflow can be started with
```
task exec -- server:dev
```
Changes in the server code will reload the server automatically. However,
changes in the client code requires re-running the client build and restarting
the server (sometimes)
