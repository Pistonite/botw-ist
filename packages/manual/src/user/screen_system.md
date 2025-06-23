# Screen System

The **Screen System** simulates the game's behavior when you switch
screens, for example when pausing the game or talking to a shop keeper (which
opens the shop dialog).

There are 3 main types of screen the game can be on:
- **Overworld**: When you have control of the player
- **Inventory**: When you open the inventory menu (i.e. pause menu)
- Other dialogs such as selling items

Most of the time, the simulator can switch between screens automatically
depending on the actions, so the effect of this system should be transparent
to those who are used to previous versions of the simulator. 

Understanding this system could be useful, if you want to explicitly control
when you open a screen, which can be helpful when optimizing and verifying IST setups.

## Game State
While not technically a screen, the *Game* itself can also have 2 different states:
`Running` and, well, not `Running` (closed).

The "initial state" of the simulator is similar to the state of a new game.
When executing most commands, the game will keep running, unless:
- You manually closed the game with the <skyb>close-game</skyb> command.
- The game crashed when executing some command.

```admonish note
The simulator also simulates game crashes. However, when you encounter a game crash,
note that it's also possible it's a bug in the simulator. 
Please report it on [GitHub](https://github.com/Pistonite/botw-ist/issues)
if the simulator crashes on a step that you don't think is supposed to crash in game.

You can also view the detail of the crash in the `Crash Viewer` Extension
```

Whenever the game is closed in the middle of a simulation (either closed manually or crashed), it will not automatically
restart. You have to use either of the commands below:
- <skyb>new-game</skyb> to start a new game
- <skyb>reload</skyb> or <skyb>reload SAVE_NAME</skyb> to start the game and reload a save
  - `SAVE_NAME` is the name of the save, see Reload (link TODO)

## Screen Types

The `Overworld` screen is the default state when you start a game.
For example, you can:
- Getting new items, such as <skyb>get</skyb>, <skyb>pick-up</skyb> and <skyb>cook</skyb>
- Dropping items with <skyb>drop</skyb>
- ... all the other things you can do in the overworld

The `Inventory` screen is when you pause the game and can see the inventory.
The Quick Menu (Dpad Menu) also counts as the inventory screen, although
there are some minor differences in some edge cases. You can specify
if Quick Menu must or must not be used for an action. See (link TODO) for details.

Other types of screens correspond to different stuff you do in the overworld.
They are differentiated to track when you have to return to the overworld to perform
the next action:
- `Shop`: When talking to a shop owner to sell things, or sometimes buy things.
  - The simulator does not track if you are allowed to buy/sell at the shop owner. It
    will assume you can always both buy and sell.
- `Statue`: When talking to a Goddess Statue for trading Spirit Orbs
  - The event flow is currently simulated and does not detect if the game softlocks,
    if there are multiple stacks of orbs.

The screen system works like a state machine. It will only transition one screen
at a time. For those transitions, it can only be from a screen to `Overworld`,
or from `Overworld` to another screen. If you need to transition between other screens,
it will first return to `Overworld`, then open the other screen.

For example, if you are in the inventory menu and need to talk to a shop owner to sell something
(i.e. to execute the <skyb>sell</skyb> command):
- The simulator first checks if you are already in the `Shop` screen
- Since you are in the inventory screen, the simulator must return to `Overworld` by closing the inventory menu
- Then, it simulates talking to a shop owner by transitioning to the `Shop` screen
- Finally, it sells the item
- After all of that, it will stay in the `Shop` screen until it has to transition again

That all happens in a single <skyb>sell</skyb> command!

## Switching Screens
TODO

