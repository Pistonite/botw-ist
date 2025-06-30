# Build and Run

This applies to development of:
- The Web App
- The Manual (this thing you are reading)
- The Server

```admonish tip
For all the commands like this:
   
    task exec -- app:dev

You can also `cd` to the package and execute the task:

    cd packages/app
    task dev


```


## Web Application
```admonish warning
Running the web app locally requires Secure Origin, which means either running from `localhost`
or setting up HTTPS, see [here](https://mono.pistonite.dev/standard/setup_https.html)
```

To run the web application:
```
task exec -- app:dev
```
The UI will automatically reload as you make changes.

Note that DirectLoad will not work when running the application locally since
it's a server feature.

## Manual
To run the manual (this website):
```
task exec -- manual:dev
```
The manual will automatically reload when making changes

## Server
```admonish warning
Currently, running the server locally also requires building the runtime as
part of the assets, which requires a dump of the game to build
```

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
