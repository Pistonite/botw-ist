# Extensions

The simulator supports extensions. Each extension is an embedded web page
that communicates with the app through the messaging API.

## Builtin extensions

## Custom extensions
The simulator supports loading your own or 3rd-party extensions.

Custom extensions have some restrictions:
- They can only be opened as popups

## Creating custom extension
Custom extension can be built with any technology as long as
it's served as a HTML webpage in the end. 
The `@pistonite/create-skybook-extension` script
will create a `vite` project with `React + TypeScript`

You need to following tools:
- NodeJS
- `pnpm`

Run
```
pnpm exec @pistonite/create-skybook-extension@latest
```

Custom extensions uses the `@pistonite/skybook-api` library
to communicate with the host app.

```typescript
import { 
    bindExtensionHost, 
    type ExtensionApp,
} from "@pistonite/skybook-api/sides/extension";
import { createExtensionAppClient } from "@pistonite/skybook-api/extension";
import { type Delegate, hostFromDelegate } from "@pistonite/workex";

// client to call the app

const app = createExtensionAppClient();

<!-- const properties = getAppProperties(); -->
<!--     new ExtensionAppClient({ -->
<!--     worker: withTargetOrigin(self, properties.targetOrigin), -->
<!-- }); -->

// your extension. these are functions that the app will call you
const delegate = {
    onDarkModeChanged: async (dark) => {
        console.log("User changed the dark mode to: " + dark)
    },
    onLocaleChanged: async (locale) => {
        console.log("User changed the language to: " + locale)
    },
    onScriptChanged: async (script) => {
        console.log("User changed the script to: " + script)
    },
    onStepChanged: async (script, step) => {
    },
    onViewChanged: async (script, step) => {
    },

    // please raise an issue on GitHub if your use case requires
    // more 

} satisfies Delegate<ExtensionApp>;

// initiate communication
const handshake = bindExtensionHost(hostFromDelegate(delegate), { worker: self });
await handshake.initiate();

// on handshake complete, your extension will be connected to the app!
// you will receive a seriers of "change" events

// you can use app's functionality to get properties:
const result = await app.resolveItem("apple", false, 1);
// { val: { val: ["Item_Fruit_A"] } }


```

Note that it's your responsibility to make sure your extension
keeps up with the latest version of `@pistonite/skybook-api`, there will
be announcement in advance if breaking changes will happen.
