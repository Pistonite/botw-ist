# User Manual

```admonish info
This section covers how to use the Simulator App. While not
required, understanding IST itself could make it easier to understand
some of the concepts here. You can read about IST [here](../ist/index.md).
```

## How the Simulator works
The **Simulator** runs on a **Script**, which is a text file that contains
**Commands** for the simulator. Usually, the commands are the **steps** or **actions**
you perform in the IST setup.

Here's an example of such script; each line is a command.
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

To learn more about commands, see [Command Syntax](./syntax.md) and [Command Reference](./commands.md).

In the simulator UI, you can edit the script in the **Script Editor**.
Whenever the script changes, the simulation will automatically rerun in the background.
You can navigate different steps of the simulation by moving your cursor in the editor.
The UI will display the state of the inventory *after* the command the cursor is on.

## Modes
The simulator app has 3 editing modes:
- `Auto Saved`: This is the default mode. Any change you make to the script will be saved locally in your browser,
  so the same script will be there when you open the app the next time.
- `Not Saving`: When editing script in this mode, the changes won't be saved to your browser.
- `View Only`: This is the default mode when you open a URL that directly loads a script.
  The script is read-only in this mode. You can switch to `Not Saving` to enable editing.
  Note that errors will NOT show in the editor in this mode.

You can switch the mode anytime in the top-left corner of the header.

~~~admonish warning
When switching to `Auto Saved`, your locally-saved script will be overwritten with whatever
script that's currently in the script editor!

If you accidentally overwrite your local script and you need it, you can still recover it
by open the devtool console (F12) and type in the following:

```typescript
console.log(localStorage.getItem("Skybook.AutoBackupScript"))
```

Press enter, and copy the output.

This entry is updated whenever you switch to `Auto Saved` from the other modes. If the backup is
lost, your script will be lost forever.
~~~

## Migration from V3

URLs with a V3 script embedded (one that starts with `https://ist.itntpiston.app`)
can be migrated automatically to V4, by simply replacing `itntpiston` with `pistonite`
in the URL.

Since the script is mechanically converted, it might not work out of the box.
You can change the mode to `Not Saving` (see above), and see if there are any errors
in the script. Or, you can simply check if the last step has the correct outcome.


Notable differences:
- <skyb>drop</skyb> in V4 only allows dropping droppable items. For example,
  <skyb>drop hasty-elixir</skyb> will not work.
  - **Workaround**: Manually change to <skyb>!remove</skyb> or <skyb>eat</skyb>.
- `pick up` is translated to <skyb>get</skyb>, since <skyb>pick-up</skyb> in V4
  specifically targets items that are on the ground. This won't lead to errors,
  but there will be extra items on the ground.
  - **Workaround**: Add a <skyb>!system [clear-ground]</skyb> at the end of the script.
- For setups with Prompt Entanglement, you need to activate PE in V4 with <skyb>entangle</skyb>
  command.

```admonish info
While it's technically more consistent to translate the old script using supercommands
like <skyb>!remove</skyb> that mimics the old behavior more consistently, supercommands
are not meant to be overused, so it's not worth to change the semantic of the script
just for the edge cases.
```
