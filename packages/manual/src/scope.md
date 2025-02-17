# Scopes

Scope is a concept that the simulator runtime uses to enforce
the validity of the setup, and automatically update
different parts of the simulation. For example:

- You cannot use an item while talking to a NPC
- GDT Inventory is synced when closing inventory
- If the game crashes, you can't do anything until you restart it

Currently, there are three scopes in the simulator:
- Game Scope: The game is running, and the player has control of Link
- Inventory Scope: The player is looking at the inventory (pressed `+`)
- Dialog Scope: The player doesn't have control of Link (for example, talking to an NPC)

## Automatic Scope Management
For the most part, scopes are managed automatically by the simulator runtime
based on the command, so you don't have to worry about them. 

For example,
consider the following script

```skybook
get 1 apple
eat 1 apple
get 1 apple
```

At first, the simulation state is not in any scope.
To get an item, you must have control of Link, so the <skyb>get</skyb> action requires `game, !paused` scope, so the simulator automatically
activate the `game` scope by starting a new game.

The next <skyb>eat</skyb> action requires `game, inventory` scope because you need
to be in the inventory to eat an item. The simulator infers
that you want to pause the game to eat the item, so it automatically
activates the `inventory` scope.

Finally, the last <skyb>get</skyb> action requires the game to be not paused,
so the simulator automatically deactivates the `inventory` scope to allow
the action to be performed.

## Manual Scope Management
Certain actions like `pause` can be used to change the scope manually.
When the scope is activated manually, the simulator will not automatically
change the scope until the manual scope is deactivated.

For example, consider the following script, which is the same as above except
that it manually pauses the game before `eat`
```
get 1 apple
pause
eat 1 apple
get 1 wood # Error!
```

Now the simulator will not automatically deactivate the `inventory` scope
for `get 1 wood`. Instead, it will give an error saying you cannot get new 
item while paused.

## Scope Conflict
Another error that the scope system checks for is conflicting scopes.
For example, you cannot access inventory while talking to an NPC.
This is implemented by you cannot activate `inventory` scope 
while the `dialog` scope is active.

The following script is valid:
```
sell 1 apple
eat 1 apple
```
Here, `sell` activates the `dialog` scope, and `eat` deactivates it to
activate the `inventory` scope.

However, if you manually activate the `inventory` scope before `sell`,
you will have an error because the `dialog` scope cannot be automatically
activated while `inventory` scope is in use.

```
talk-to shopkeeper
sell 1 apple
eat 1 apple # Error!
```

## Crashes
Most commands require the `game` scope, which can be automatically activated.
The only exception is when the game crashes. In this case, you must
manually activate the `game` scope with an action like `new-game` or `reload`

## Testing
Scope can be tested using the `!assert-scope` command. The special `not-paused`
keyword can be used to test that `game` is the top-level scope

```
!assert-scope game
!assert-scope inventory
!assert-scope game not-paused
```
