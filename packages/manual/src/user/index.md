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

## Systems in the Simulator
Skybook aims to be a 100% accurate IST simulator. To achieve that, it *emulates*
subsystems of the game as much as possible. However, not all subsystems can be emulated,
especially those that are not reversed-engineered fully or not at all. Some subsystems
also may not be worth to emulate since a simulation is good enough.

The systems that are involved in the simulator include:
- Inventory
- GameData
- Saves
- [Screens](./screen_system.md)
- [Overworld](./overworld_system.md)
