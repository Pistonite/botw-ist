# Building the Runtime

While you can make changes and build the runtime packages locally,
to run it, you must have a BlueFlame image.

Building the BlueFlame image requires you to dump some parts of the game.
See [uking-relocate](https://github.com/Pistonight/symbotw/tree/main/packages/uking-relocate)
for more info.

Once you have the game files prepared, put them in `/packages/runtime-tests/data/botw150`:

The directory structure should be like
```
packages/
  runtime-tests/
    data/
      botw150/
        romfs/
        main.elf
        rtld.elf
        sdk.elf
        subsdk0.elf
```

Then run `task exec -- runtime-tests:build-mini` to setup the mini image
used for running the runtime locally.

Alternatively, you can put the built BlueFlame image at `/packages/runtime-tests/data/program-full.bfi`, then run:

```
cd packages/runtime-tests
task update-trace-hash
python scripts/relocate.py
```

This will also produce `program-mini.bfi`.

Once you have the mini image, you can now make changes to the runtime packages,
and rebuild the WASM package:
```
cd packages/runtime-wasm
task build
```

After building the WASM package, run the web app (or reload if already running)
for it to pick up the newly built changes.
