# botw-item-assets

React components for item images in BOTW.

**NOTE: The library is not usable out of the box, as this repository
does not contain the assets**

## Usage

See [ActorSprite.tsx](./src/ActorSprite.tsx) and [ModifierSprite.tsx](./src/ModifierSprite.tsx)

## Install (into a mono-dev repo)

1. Add this repo as submodule
2. Check `package.json` and add the packages to `pnpm-workspace.yaml` catalog.
3. Currently, only pulling assets from private Google Cloud is supported:
    ```bash
    task pull-build-priv
    task build-sprites
    ```

Add the dependency to another package:

```json
{
    "dependencies": {
        "botw-item-assets": "workspace:*"
    }
}
```

Import

```typescript
import { ActorSprite } from "botw-item-assets";
```
