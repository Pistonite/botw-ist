# 自定义镜像

```admonish todo
Custom Image functionality is WIP. Please reach out to me if you want to play with it.
```

When running into code outside the normal inventory logic using glitches
like [Item Stack Underflow](../ist/isu.md), the simulator will probably crash
because it does not contain the full game to be able to execute setups
that involves code outside of the normal inventory code.

BUT, the simulator is *capable* of executing the whole game's code
if it is given access. This is referred to as the **Full Mode** or **Custom Image Mode**.
To do this, you need to create a BlueFlame image (a `.bfi` file) from the 
game files.

## Create Image
To create the image, you need the following things:
- Dump of the game (only some files are needed, not all of them)
- A Nightly Rust toolchain
  - If you don't have rust installed, see [here](https://mono.pistonite.dev/standard_tools.html#rust-toolchain)

For detailed instructions on what is needed and steps to create the image, see
the [`uking-relocate`](https://github.com/Pistonight/symbotw/tree/main/packages/uking-relocate)
tool. Follow the instruction on the tool's README.

## The `env` Block
To tell the simulator to use a custom image instead of the default image,
put an `env` block literal at the beginning of the script.
The `env` block must be at the beginning, before any lines and comments.
Empty lines are allowed before the block.

The `env` block should contain one `<key> = <value>` per line. Here is an example:

```skybook
'''env
image = 1.5
dlc   = champions-ballad
program-start  = 0x0000001234500000
stack-start    = 0x0000005678900000
stack-size     = 0x40000
heap-free-size = 0x40000
pmdm-addr      = 0x0000003456789ab0
'''
```
```admonish note
The numeric values must be hexadecimal; the leading `0x` is optional.
```
```admonish note
If a value is invalid, it's equivalent to that value being not specified.
At the same time, you will see an error in the script editor.
```

The `image` key specifies the version of the game.
Allowed values are `1.5` and `1.6`.

```admonish warning
Currently, only `1.5` is supported. `1.6` is recognized but
not supported. Newer versions won't be recognized by either
the simulator or `uking-relocate`.
```

The rest of the keys are optional. If not specified, the simulator will use the internal default values.

| Key | Value | Description |
|-|-|-|
|`dlc`| a DLC specifier (see below) | Specify the DLC version to simulate |
|`program-start`| Region Address | The physical memory address of the start of the program region. This is checked against the `program-start` of the image file. The simulator cannot start if this doesn't match. If this is not specified, any `program-start` will work. |
|`stack-start` | Region Address | The physical memory address of the start of the stack |
|`stack-size` | Size | The size of the stack, must be aligned to 4KB |
|`heap-free-size` | Size | Size of the free region of the heap for the simulator to allocate memory |
|`pmdm-addr` | Physical Address | The address of the `PauseMenuDataMgr` (in other words, the value of `PauseMenuDataMgr*`). This is used to calculate heap start |

```admonish danger
Large stack/heap size can slow down simulator start-up. It is recommended to only change these
if the default does not work for you.
```

DLC specifier can be any string that contains `0`, `1`, `2`, or `3`, which correspond
to no DLC installed, `DLC ver1.0 (Day 1)`, `DLC ver2.0 (Master Trials)` and `DLC ver3.0 (Champions' Ballad)`.
One of the following shorthand is recommended:
| DLC Version | Possible Specifiers |
|-|-|
| No DLC | `nodlc`, `none`, `uninstalled` |
| ver 1.0| `dlc-1`, `ver1.0`, `day-1`  |
| ver 2.0| `dlc-2`, `ver2.0`, `master-trials`  |
| ver 3.0| `dlc-3`, `ver3.0`, `champions-ballad` |

Invalid DLC version specifier defaults to `ver3.0`.

A Region Address must be a hexadecimal string aligned to `0x100000`,
the most significant 6 hex-digits must be all `0`.

A Size must be a 32-bit positive integer, aligned to `0x1000`.
A size of `0` is the same as unspecified, and the internal defaults will be used.

`pmdm-addr` must be aligned to `0x8`.

Furthermore, the program, stack, and heap regions must not overlap.

## Upload the Custom Image
Once `image` is specified in the `env` block, refresh the page.
You should see a prompt that asks if you want to upload
a custom image. Select `Setup` and follow the on-screen instructions
to upload the `.bfi` file you created.

```admonish note
If the custom image fails to load, you can always select `Use Default Image` 
in the prompt to start the application normally and fix your script.

The uploaded image is stored in your local browser.
```

## Use Custom Image By Default
You can opt-in to always use your custom image for you own script, even when
the `env` block doesn't specify a custom image.

To enable this, check the box that says `Use Custom Image by default for my scripts`
when uploading the custom image.

## Delete Uploaded Custom Image
You can open the
3-dot menu on the top of the app and select `Delete Custom Image`.
This clears the custom image file that is stored in your local browser.
