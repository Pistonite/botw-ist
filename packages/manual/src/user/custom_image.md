# Custom Image

The simulator does not actually run the game (for obvious reasons).
When running into code outside the normal inventory logic using glitches
like [Item Stack Underflow](../ist/isu.md), the simulator crashes the game
because it doesn't know what to do.

In these situations, you have to provide a custom BlueFlame image (with `.blfm` extension).
You can create an image from the game files.
```admonish info
[BlueFlame](https://github.com/Pistonite/botw-ist/tree/main/packages/blueflame) 
is the core of the simulator that actually runs the simulation
```

## Create Image
You can create an image with the [`uking-relocate`](https://github.com/Pistonight/symbotw/tree/main/packages/uking-relocate)
tool. Follow the instruction on the tool's README.
If you don't have Rust installed, see [here](https://mono.pistonite.dev/standard_tools.html#rust-toolchain)

## The `env` Block
To tell the simulator to use a custom image instead of the default image,
put an `env` block literal at the beginning of the script.
The `env` block must be at the beginning, before any lines and comments.
Empty lines are allowed before the block.

The `env` block should contain one `<key> = <value>` per line. Here is an example:
```skybook
'''env
image = custom-ver1.5
dlc   = champions-ballad
program-start  = 0x0000001234500000
stack-start    = 0x0000005678900000
stack-size     = 0x40000
heap-free-size = 0x40000
pmdm-addr      = 0x0000003456789abc
'''
```
```admonish note
The numeric values must be hexadecimal, the leading `0x` is optional
```
```admonish note
If a value is invalid, it's equivalent to that value being not specified.
At the same time, you will see an error in the script editor.
```

The `image` key specifies the version of the game, either `custom-ver1.5` or `custom-ver1.6`.
You can also use `1.5` or `1.6` as shorthands. An invalid `image` will fallback to using the default
image.

The rest of the keys are optional. If not specified, the simulator will use the internal default values.

| Key | Value | Description |
|-|-|-|
|`dlc`| a DLC specifier (see below) | Specify the DLC version to simulate |
|`program-start`| Region Address | The physical memory address of the start of the program region |
|`stack-start` | Region Address | The physical memory address of the start of the stack |
|`stack-size` | Size | The size of the stack, must be aligned to 4KB |
|`heap-free-size` | Size | Size of the free region of the heap for the simulator to allocate memory |
|`pmdm-addr` | Physical Address | The address of the `PauseMenuDataMgr` (in other words, the value of `PauseMenuDataMgr*`). |

DLC specifier can be any string that contains `0`, `1`, `2`, or `3`, which correspond
to no DLC installed, `DLC ver1.0 (Day 1)`, `DLC ver2.0 (Master Trials)` and `DLC ver3.0 (Champions' Ballad)`.
One of the following shorthand is recommended:
| DLC Version | Possible Specifiers |
|-|-|
| No DLC | `nodlc`, `none`, `uninstalled` |
| ver 1.0| `dlc-1`, `ver1.0`, `day-1`  |
| ver 2.0| `dlc-2`, `ver2.0`, `master-trials`  |
| ver 3.0| `dlc-3`, `ver3.0`, `champions-ballad` |

Invalid DLC version specifier defaults to `ver3.0`

A Region Address must be a hexadecimal string aligned to `0x100000`,
the most significant 6 hex-digits must be all `0`.

A Size must be a 32-bit positive integer, aligned to `0x1000`.
A size of `0` is the same as unspecified, and the internal defaults will be used.

`pmdm-addr` must be aligned to `0x8`.

Furthermore, the program, stack, and heap regions must not overlap.

## Upload the Custom Image
Once `image` is specified in the `env` block, the simulator should pick that
up and warn you that the current runtime parameters don't satisfy the parameters
in the `env` block. You need to restart the application by refreshing the page.

Once the application restarts, you should see a prompt that asks if you want to upload
a custom image. Select `Yes` and follow the on-screen instructions.

```admonish note
If the custom image fails to load, you can always select `Use Default Image` 
in the prompt to start the application normally and fix your script.
```

## Use Custom Image By Default
You can opt-in to always use your custom image for you own script, even when
the `env` block doesn't specify a custom image. This includes loading
scripts saved locally in your browser, or loading a Gist that you own.

To enable this, check the box that says `Use Custom Image by default for my scripts`
when uploading the custom image.

If you have already uploaded a custom image and want to change this setting, open the
menu next to the app logo on the top-left, and select `Reset Image`.
This will clear any uploaded custom image and any settings related.
Then you can restart the application and upload an image again with the new setting.

```admonish note
Note that this option will only appear if you have a custom image uploaded
```
