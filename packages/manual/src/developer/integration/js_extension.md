# TypeScript Extension

## Installing the Packages

Skybook has 2 TypeScript packages:
- `@pistonite/skybook-api` contains the type/API definitions
  and messaging implementation. This is required for all extensions.
- `@pistonite/skybook-itemsys` is the system that handles displaying
  the item slots and other related functionalities, such as tooltip
  and Drag-and-Drop. This is optional and only needed if your extension
  wants to display item slots, or to support dropping an item dragged from
  somewhere else onto your extension.

You can install only `@pistonite/skybook-api`, or both (using `pnpm` as the package
manager for example):

```bash
pnpm i @pistonite/skybook-api
pnpm i @pistonite/skybook-itemsys
```

**Important**: The packages are **TypeScript-only**. Meaning it's required
that your project use a transpiler or bundler. Additionally,
`@pistonite/skybook-itemsys` also requires using the `React` framework for the parts
that work with the item system.

If you are not familiar with bundler configuration, it's recommended
that you use the same stack as the IST Simulator itself: `pnpm` + `Vite` + `mono-dev`.
TODO: example or setup script or something.

If you don't use `mono-dev` or `vite`, ensure your bundler has the config/plugin
setup to bundle `React` code and `*.yaml` imports.

If you don't use `mono-dev`, ensure you manually dedupe these packages
in `vite.config.ts`:

```typescript
// vite.config.ts
import { defineConfig } from "vite";

export default defineConfig({
    resolve: {
        dedupe: [
            "@pistonite/pure",
            "@pistonite/workex",
            "@pistonite/skybook-itemsys", // if installed
        ]
    }
});
```

When using `mono-dev`, only `@pistonite/skybook-itemsys` needs to be specified,
if installed.
```typescript
// vite.config.ts
import { defineConfig } from "vite";
import monodev from "mono-dev/vite";

const monodevConfig = monodev({});

export default defineConfig(
    monodevConfig({
        resolve: {
            dedupe: [
                "@pistonite/skybook-itemsys", // if installed
            ]
        }
    }),
);
```
