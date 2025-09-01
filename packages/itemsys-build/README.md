## Building animated icons

The animated icons are built from a video recording of the game.

Python dependencies are at `scripts/videoprep/requirements.txt`

- Step 1: Get the video, see `scripts/videoprep/splice.ps1`
- Step 2: (`vdecode` task) Decode the video into frames, crop the item out and 
  use algorithms to automatically fix the image (for example delete the orb count).
  See `scripts/decode.py`
- Step 3: Encode the frames into `webp`, and use an algorithm to add alpha.
  See `src/encode.rs`
