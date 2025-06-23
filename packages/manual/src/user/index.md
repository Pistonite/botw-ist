# User Manual

```admonish info
This section covers how to use the Simulator App. While not
required, understanding IST itself could make it easier to understand
some of the concepts here. You can read about IST [here](../ist/index.md)
```

## How the Simulator works
The **Simulator** runs on a **Script**, which is a text file that contains
**Commands** for the simulator. Usually, the commands are the **steps** or **actions**
you perform in the IST setup.

Here's an example of such script, each line is a command.
```skybook
get 1 pot-lid 1 apple 1 slate 1 glider
equip Shield
!break 3 slots
save
unequip shield
hold apple; drop
reload
save
drop apple
reload
```

In the simulator UI, you can edit the script in the **Script Editor**.
Whenever the script changes, the simulation will automatically rerun in the background.
You can navigate different steps of the simulation by moving your cursor in the editor.
The UI will display the state of the inventory *after* the command the cursor is on.

To learn more about commands, see [Command Syntax](./syntax.md) and [Command Reference](./commands.md).

## Simulation vs Emulation
The difference of simulation and emulation, given by AI:

> Simulation models a system's behavior to understand and predict outcomes, while emulation replicates a system's exact functionality to run original software or hardware.

It is important to know that the App is a *simulator*, not an *emulator*. In other words,
there will be inaccuracies. In some way, this is a good thing since it allows us to sometimes
to things not possible directly or indirectly in game, to make our lives a bit easier.

The simulator still uses emulation in some areas to achieve maximum accuracy.

The following subsystems are emulated:
- Inventory (`PauseMenuDataMgr`)
- GameData (`GdtManager`)

The rest of the subsystems are simulated, including:
- Saves
- [Screens](./screen_system.md)
- Overworld

```admonish important
The app DOES NOT actually run the game, even when emulating a subsystem.
Emulating the whole game has major performance and legal issues, and is a non-goal
of this project.
```
