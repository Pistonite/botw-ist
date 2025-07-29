# System Operations

The <skyb>!system</skyb> command provides low-level access to the simulated systems.
Since these commands expose internals of the simulator, they should be considered **unstable**.
They could change without warning in future versions, if the simulator internals change.

## Syntax
> `!system [SYSTEM_META]`

The `SYSTEM_META` meta properties are parsed into a list of "system commands", then executed in order.

<div class="skybook--wide-table">

| Property | Description |
| - | - |
| `dlc` | (`int` or `string`) Change the DLC version stored in `AocManager` in the game memory. <br><br> Numbers `0`, `1`, and `2` correspond to No DLC, Day-1 (ver1), and Master Trials (ver2). Any other value means Champion's Ballad (ver3). See [DLC version](../generated/constants.md) for supported string values. |
| `delete-save` | (omit value or `string`) Delete save data. A save name should be specified (omit value to mean manual save). |
| `clear-ground` | (omit value) Delete all items on the ground, including the ones that are being spawned. |
| `clear-overworld` | (omit value) Delete all items in the overworld, including the ones equipped by the player. |
| `sync-overworld` | (omit value) Sync (i.e. re-create) player equipments in the overworld. |
| `reload-gdt` | (omit value or `string`) Load save data into GDT, but do not load the inventory. A save name should be specified (omit value to mean manual save). |
| `loading-screen` | (omit value or `string`) Trigger loading screen. The special value `no-remove-translucent` means trigger a loading screen without first attempting to remove translucent items. |

</div>

Examples:
```skybook
# Get Travel Medallion, save, then uninstall DLC and reload
get travel-medallion; save; close-game
!system [dlc=master-trials]
reload # Travel Medallion is gone!

# Simulate entering a shrine and clearing it
!system [loading-screen]
get spirit-orb
!system [loading-screen]
```
