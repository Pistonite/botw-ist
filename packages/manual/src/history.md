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
Meanwhile, speedrunners of the 100% category started to incorporate
it into the route. However, even the simplest form of this glitch required 
tracking the exact state of the player's inventory for every action executed, such as picking
up or dropping an item. The rules and patterns of this glitch then needs
to be applied at each of these steps. It was a painstaking process to manually
track all the information.

Luckily, tasks that are difficult for humans are often easy for computers.
On June 18, 2022, the Hundo Duplication Simulator was born. Users could
type actions such as <skyb>get 1 core</skyb> into the simulator. The simulator app
will display the inventory state and allow the user to quickly navigate between the steps.
This was a massive help for optimizing setups for speedruns because people can change one step and the app will re-calculate
the state for every step afterwards. This model is still used by the simulator today.

## V2 - IST Simulator
As more applications of IST are found, the glitch has found its use in categories
outside of the 100% category, for example in All Shrines and All Dungeons. One
of such application is [Direct Inventory Corruption](./ist/dic.md). However, the simulator
was built in one day and did not support any cases other than the very basic
rules that were initially discovered.

Now that this glitch is not specific to 100-percent, the project was renamed
to IST Simulator. V2 of the simulator added `GameData` Inventory and more items 
to simulate the inventory state closer to how the game handles it. Direct Inventory Corruption
support was added. People were happy.

## V3 - IST + Weapon Modifier Corruption
...Until more applications of IST are found again, and the simulator being a simulator,
did not support these new exploits because I couldn't predict the future.

Every time a new discovery is made, it almost certainly invalidates assumptions previously
made when coding the simulator, and the core has to be re-designed to incorporate the new
knowledge. In November 2022, I started re-designing the core by referencing
the [BOTW decompilation project](https://github.com/zeldaret/botw).
This massively improved the accuracy of the simulator and supported new applications
such as [Weapon Modifier Corruption](./ist/wmc.md). The UI was also revamped, adding syntax highlighting
to commands and displaying modifier information for items.


## V4 - Skybook
After the V3 update, a few small fixes were submitted, but no big re-architecture
was made for 2 years. 

While V3 was mostly accurate for setups viable for speedruns, it was still far from being perfect:
  - It didn't handle special cases for champion abilities or Travel Medallion.
  - It didn't handle cases where tabs aren't discovered
  - It couldn't reliably detect and warn user when an action is not possible in game,
      such as interacting with the inventory when `mCount` is `0`.
  - ...

It was clear that we can not chase these edge cases forever. Something needs to be remade *from the ground up*, again.
Sometime in 2022-2023, I have theorized to build a mini-emulator, to run the simulator from a small part of *the game itself*.
I submitted this idea as a proposal for senior project at my college, and got a student team who was interested in it.
I lead the team as the advisor to build a prototype that would later become **BlueFlame**, the new core for V4.

At the same time as BlueFlame was being developed by the students, I worked on everything else - a new command system,
new simulated systems like Overworld and Screens... And finally, in July 2025, Skybook was born.

For the first time, the IST simulator project is *ahead of the game*. The simulator is so accurate, it could
replicate setups that I didn't know would be possible (and thought they were bugs, but they are not).
