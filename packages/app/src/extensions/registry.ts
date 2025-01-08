import { makeStub } from "./Stub1.tsx";
import type { ExtensionComponentProps } from "./types.ts";


export const getExtensionComponent = async (id: string): Promise<React.ComponentType<ExtensionComponentProps> | undefined> => {
    switch (id) {
        case "editor":
            return (await import("./editor/Component.tsx")).Component;
        case "item-explorer":
            return (await import("./item-explorer/Component.tsx")).Component;
        case "stub1":
            return makeStub(1);

    }

    return undefined;
}
