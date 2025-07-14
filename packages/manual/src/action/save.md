# Save Files

Handling game's running and closed state, and simulation of save files

- <skyb>save</skyb> command saves to the *manual save* slot
- <skyb>save-as file-name</skyb> command saves to a *named save* slot.
    You can later reload this save by its name. This is used to simulate auto-saves,
    but you can have unlimited number of them.
- <skyb>reload</skyb> command reloads a manual or named save
- <skyb>new-game</skyb> reloads an imaginary save with the state of a new game
- <skyb>close-game</skyb> closes the game

## Syntax

> `save` <br>
> `save-as FILE-NAME` <br>
> `reload` <br>
> `reload FILE-NAME` <br>
> `close-game` <br>
> `new-game` <br>

Example
```skybook
# Save to the manual save slot
save
# Save to a named save called `my-save`
save-as my-save
# Reload the manual save
reload
# Reload a named save
reload my-save
```

## Inspecting Save Files in the App
Amongst other things, a save file for the game includes a copy of savable flags in GameData,
which contains the items in the save. Therefore, the simulator displays save files
similar to how it displays GameData.

Using the `Save Files` extension, you will see a list of save names available at the
current step in the simulation. Clicking on a save will then display the items in that save.

Note that you can inspect saves even on steps where the game isn't open.

## New Game and Restarting the Game
The <skyb>reload</skyb> and <skyb>new-game</skyb> are the only 2 commands that can
restart the game after it's closed, either due to crash or was manually closed.

Currently, <skyb>new-game</skyb> is implemented as reloading an imaginary save
made at new game. The implementation is as follows:

| Game State | Action | Implementation |
|-|-|-|
| Running | <skyb>reload</skyb> | Reload the save |
| Running | <skyb>new-game</skyb> | Reload the "new game" save |
| Closed | <skyb>reload</skyb> | Start a new game, then reload the save |
| Closed | <skyb>new-game</skyb> | Start a new game |

This is not 100% accurate to what the game does, but should be close enough.

## Simulating the System menu
<skyb>save</skyb> (and <skyb>save-as</skyb> if in inventory) will currently automatically <skyb>unhold</skyb>
the currently held items, similar to how the game does that when you switch to the System menu.

## Detail

- <skyb>save</skyb> requires [`Inventory`](../user/screen_system.md) screen
- <skyb>save-as</skyb> can be performed in either `Inventory` or `Overworld`
- <skyb>reload</skyb> requires `Inventory` screen if the game is running.
  - <skyb>new-game</skyb> is similar, see implementation detail above.
