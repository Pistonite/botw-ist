# Trials

Trials are special quests in the game where the game takes away
your items, and give you a temporary inventory until the trial is completed.

The trials in the game include:
- Trial of the Sword
- Stranded on Eventide
- 4 Blight Refights for Champion's Ballad

Since each trial has a event flow associated with it, it would
be too complex to try to simulate every possible interaction with the trials.
Therefore, the simulator only provides <skyb>!trial-start</skyb> and <skyb>!trial-end</skyb>
commands to put the inventory in or out of the trial mode.

## Syntax

> `!trial-start` <br>
> `!trial-end`

- Starting a trial will:
  - delete all items except key-items, and set the trial mode flag.
  - re-create equipments in overworld.
- Leaving the trial will reset the trial flag and reload your items from GDT.
  - Most of the time <skyb>reload</skyb> a save to exit the trial should work as expected with no issues.

Most of the time, a single <skyb>!trial-start</skyb> or <skyb>!trial-end</skyb>
is accurate enough and you don't need to worry about the specific details
for each trial. See below if you are in an edge case where the specific event flow matters.

```admonish danger
You should almost always make sure you are in the overworld to start a trial.
Starting trial with inventory open is supported but may have unintended results.
The simulator will NOT automatically switch the screen for you.
```

## Eventide
When walking on eventide, the held items will be dropped if Arrowless Smuggle is active,
or unheld if not. You should handle this manually

```skybook
# Make sure you are not holding items (hold smuggle is fine)
unhold
# Make sure you are in the overworld to walk onto Eventide
unpause
# Trigger the trial
!trial-start
```

If you give up the Eventide challenge, there will be a loading screen. However there will NOT
be one if you complete the challenge.
```skybook
!trial-end
!system [loading-screen]
```

## Trial of the Sword
Trial of the sword event is more complicated to simulate correctly:
```skybook
# Make sure you are not holding items (hold smuggle is fine)
unhold
# The event flow will automatically equip master sword, remove it, then add it back
# This is also not fully accurate
equip master-sword; unpause; !remove master-sword; get master-sword
# enter trial mode
!trial-start
# there's a loading screen that sends you to TOTS map
!system [loading-screen]
```

Leaving TOTS:
```skybook
# First end the trial and go through the loading screen
!trial-end
!system [loading-screen]
# The game automatically gives you a master sword
get master-sword
# If All 3 trials are completed, set the MS full power flag
!set-gdt <Open_MasterSword_FullPower>[bool=true]
```

