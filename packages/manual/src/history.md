# History

## Discovery
Inventory Slot Transfer (IST) was discovered in June 2022 by zxrobbin (
[Video 1](https://www.bilibili.com/video/BV1AS4y1v7wN),
[Video 2](https://www.bilibili.com/video/BV16T411576U)
). It initially manifested as an innocent glitch
to quickly duplicate items in Breath of the Wild. Little did we know
that IST will become the most complicated glitch to ever exist in BOTW.

Around June 17, 2022, as the news spread, glitch hunters from the BOTW speedrunning community
started to investigate and quickly found some patterns to the glitch.


## Hundo Duplication Simulator
Meanwhile, speedrunners of the 100-percent category started to incorporate
it into the route. However, even the simplest form of this glitch required 
tracking the exact state of the player's inventory for every action executed, such as picking
up or dropping an item. The rules and patterns of this glitch then needs
to be applied at even of these steps. It was a painstaking process to manually
track all the information.

Luckily, tasks that are difficult for humans are often easy for computers.
On June 18, 2022, the Hundo Duplication Simulator was born. Users could
type in actions such as <skyb>get 1 core</skyb>, and the app will display
the inventory state after each step. This was a massive help for optimizing
setups for speedruns because people can change one step and the app will re-calculate
the state for every step afterwards. This model is still used by the simulator today.

## The IST Simulator - V2
As more applications of IST are found, the glitch has found its use in categories
outside of 100-percent, for example in All Shrines and All Dungeons. One
of such application is Direct Inventory Corruption. However, the simulator
was built in one day and did not support any cases other than the very basic
rules that were initially discovered.

Now that this glitch is not specific to 100-percent. The project was renamed
to IST Simulator. V2 of the simulator added GameData Inventory and more items 
to simulate the inventory state closer to how the game handles it. Direct Inventory Corruption
support was added. People were happy.

## Weapon Modifier Corruption - V3
...Until more applications of IST are found again, and the simulator being a simulator,
did not support these applications because I couldn't predict the future.

Every time a new discovery is made, it almost certainly invalidates assumptions previously
made when coding the simulator, and the core has to be re-designed to incorporate the new
knowledge. In November 2022, I started re-designing the core by referencing
the [BOTW decompilation project](https://github.com/zeldaret/botw).
This massively improved the accuracy of the simulator and supported new applications
such as Weapon Modifier Corruption. The UI was also revamped, adding syntax highlighting
to commands and displaying modifier information for items.

## Skybook - V4
After the V3 update, a few small fixes were submitted, but no big re-architecture
was made for 2 years. During this time, some minor inconsistencies were found, mostly
related to GameData flags (such as champion ability and discovered tabs).

Even though the remaining edge cases are rarely hit, it was clear that we could
not keep chasing these cases. If the simulator was to be improved again, it needs
to fix all these cases entirely. During this time, I had theorized a new version of the simulator that uses the game
itself to execute the actions. This effort started in Summer 2024.

TO BE CONTINUED

