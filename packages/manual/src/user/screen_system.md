# Screen System

The `Screen` system simulates different dialogs and pause menus in the game.
For example, when pausing to access the inventory or sell items by talking
to a shop keeper.

Most of the time, the simulator can switch between screens automatically
depending on the actions, so the effect of this system should be transparent
to those who are used to previous versions of the simulator. 

Understanding this system could be useful, if you want to explicitly control
when you open a screen, which can be helpful when optimizing and verifying IST setups.

```admonish tip
The simulator UI has a little icon next to the "Visible Inventory" title
to indicate which screen you are currently on
```

## Game State
While not technically a screen, the *Game* itself can also have 2 different states:
`Running` and, well, not `Running` (closed).

The "initial state" of the simulator is similar to the state of a new game.
When executing most commands, the game will keep running, unless:
- You manually closed the game with the <skyb>close-game</skyb> command.
- The game crashed when executing some command.

```admonish note
When you encounter a game crash, note that it's also possible it's a bug in the simulator. 
Please report it on [GitHub](https://github.com/Pistonite/botw-ist/issues)
if the simulator crashes on a step that you don't think is supposed to crash in game.

You can also view the detail of the crash in the `Crash Viewer` Extension.

```

Whenever the game is closed in the middle of a simulation (either closed manually or crashed), it will not automatically
restart. You have to use either of the commands below:
- <skyb>new-game</skyb> to start a new game
- <skyb>reload</skyb> or <skyb>reload SAVE_NAME</skyb> to start the game and reload a save
  - `SAVE_NAME` is the name of the save, see Reload (link TODO)

## Screen Types
While in game, there are 3 screens that are simulated:

- `Overworld`:
  - The default state when you start a game.
  - Player is able to move
  - You can get/drop items
  - ... all the other things you can do in the overworld
- `Inventory`:
  - When pause the game with the `+` button or DPad
  - Player can interact with items in the inventory
- `Shop Selling`:
  - When talking to a shop owner to sell items
  - Player can select items in the inventory to sell
- `Shop Buying`:
  - When talking to a shop owner to buy items
  - Player can select from a list of items to buy

The `Screen` system works like a state machine, when an action needs a certain
screen, it will try to transition to that screen state if possible, and display
an error if it couldn't. The transition looks something like this:

```
             Overworld
         /---------|---------\
        /          |          \
       /           |           \
Inventory     Shop Buying  ---  Shop Selling
```

For example, if you are in the inventory menu and need to talk to a shop owner to sell something
(i.e. to execute the <skyb>sell</skyb> command):
- The simulator first checks if you are already in the `Shop Selling` screen
- Since you are in the inventory screen, the simulator must return to `Overworld` by closing the inventory menu
- Then, it simulates talking to a shop owner by transitioning to the `Shop` screen
- Finally, it sells the item
- After all of that, it will stay in the `Shop` screen until it has to transition again

That all happens in a single <skyb>sell</skyb> command!

Some commands, like <skyb>get-pause</skyb>, also explicitly performs screen switching.
Note that these case do not count as manually switching screens (see below), and the simulator
will still automatically switch screens afterwards.

## Manually switching screens
The following actions count as *manually* switching the screen.
If the screen has been manually switched, the simulator will prevent
certain automatic screen switches.

- `Inventory`:
  - <skyb>pause</skyb> to open the inventory
    - No automatic screen switches can happen until returned to overworld
  - <skyb>unpause</skyb> to close the inventory and return to overworld
- `Shop` (buying and selling):
  - <skyb>talk-to NPC</skyb> to start buying or selling (`NPC` can be any `-` or `_` connected word)
    - Screen can be automatically switched between `Buying` and `Selling`, but not to `Overworld`
    - When returned to overworld, screen can be automatically switched again to all types
  - <skyb>untalk</skyb> or <skyb>close-dialog</skyb> to return to overworld

```admonish note
Note about buying: The <skyb>buy</skyb> command performs buying from items in `Overworld`,
and it will trigger automatic switch to `Overworld` if possible.
This is because when buying from a shop, you get a dialog, but it returns to overworld
after that. The only exceptions are wondering merchants and Beedle. Most of the time,
it will not matter. When it does matter, it's recommended you always use manual screen switch
actions to indicate to other people if they should buy from the same screen, or close the dialog
and talk again to buy. You can use the <skyb>:same-dialog</skyb> annotation
so the next <skyb>buy</skyb> command switches to the buy screen instead of returning
to overworld
```





