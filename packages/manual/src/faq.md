# FAQ

## What is IST? What is this app?
Inventory Slot Transfer, or IST, is a glitch in BOTW that desyncs the number of 
items you have in the inventory and number of items the game *thinks you have*.
For more details, check out [history of the app](./history.md) and an 
[overview of the glitch](./ist/index.md)

## I am new to IST, How do I use this tool?
You definitely don't need to be a master of IST to find this tool useful.
If you are looking at a speedrun setup made by someone else. 
You can view the setup in the tool as a step-by-step guide for how to perform
the glitch. If you are a glitch hunter or is interested in investigating
the glitch in more details, the [user manual](./user/index.md) has everything
you need to unlock the full potential of the simulator.

In any case, it might be helpful to understand [the basic concepts](./ist/basics.md)
of IST to get started.

## I can't understand IST, but I still want to speedrun
Don't worry. IST is very complicated. Most people (including WR holders!)
don't fully understand the glitch. This is exactly why this tool exists.

If your goal is to do the setup in a speedrun, what most people do is 
simply following each action in the setup *exactly*, either from memory,
or by looking at the steps while they do it.
Many categories also have tutorials made for the IST section. For example,
[here](https://www.youtube.com/watch?v=NZBmu9hEZY0) is one for All Dungeons
made by Player 5.

## I just want to play with IST as a casual player
Be cautious to use IST with your casual file, as effect of IST can persist
in saves and may cause the saves to be corrupted or not-loadable.

There are generalized guides for how to achieve certain things with IST
in a casual file (for example, <skyb>999 korok seeds</skyb>). You can follow
these steps. A good place to look for those steps is the `#general-help`
channel of the [speedrunning discord](./welcome.md#discord).
There are also tutorials online on YouTube (or Bilibili if you are from China)
for using IST in a casual file.

## How is the simulator made?
V1 to V3 of the simulator was developed by understanding the outcome
and patterns of IST, and by referencing the [decompilation project](https://github.com/zeldaret/botw).
It was a white-box approach, similar to a person that understood everything ever discovered about IST.
V4 took the black-box approach, where sections of the real code of the game (not decompiled code)
is executed in a sandbox orchestrated by the simulator app. This means the simulator
might even be possible to support use cases that are not discovered yet.

## Can I contribute?
Certainly! If you see some bugs and want to take a shot on fixing them,
feel free to open a PR on GitHub. If you want to add features, please discuss
with me first. A decent level of programming knowledge is needed.

Most of this project is open-source and use publicly available data of the game.
However, some parts require you own a copy of the game to develop.

Please refer to the [contributing guide](./developer/contributing.md) for more information.

```admonish note
If you goal is to add extra functionality, you might be able to do that through
an extension, See [TODO add link here]
```
```admonish note
If you are not familiar with programming, you can still contribute to the test suite
by providing your (complicated) scripts as test cases. These test cases help ensure
future updates to the simulator don't introduce bugs. See [TODO add link here]
```





