# skybook-api

API and bindings for the 3 parts of Skybook, the BOTW IST Simulator

## Runtime

The Runtime handles parsing the commands and executes the simulation

## Application

The Application handles the UI and states. It drives the Runtime to
run the simulation whenever user changes something.

## Extension

Extensions provide additional UI and functionality to the application.
Each extension talks to the Application to get and subscribe to
state changes, and it uses the Application as a middle
man for driving the simulation using the Runtime.
