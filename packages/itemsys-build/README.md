# itemsys-build

Build scripts/files/assets for itemsys

## Creating animated icons

This step is not needed for development. The relevant tools are in `tools/video`.

The animated icons are built from a video recording of the game.
The videos are stored in GCP and see `tools/video/prep/splice.ps1` to see
the (re-)encoding used to generate the spliced videos. The videos are spliced
to exactly one animation cycle.

To build the animated icons:
- `task vpull-deps` to download the videos (requires GCP access).
- `task vdecode` to decode the videos into frames and clean the frames.
- `task vencode` to encode the frames into webp.

To push the encoded animated icons: `task vpush-art`.

## Creating sprites

This step is not needed for development. The relevant tools are in `tools/sprite`.

The sprite tool re-encode item icons into spritesheets so they can be smaller and faster to load.

To build the spritesheets:
- Either build the animated icons or pull them `task vpull-art`.
- `task spull-deps` to pull the other icons.
- `task schunk` to build the chunk metadata.
