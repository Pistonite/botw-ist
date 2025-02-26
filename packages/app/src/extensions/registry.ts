import { makeStub } from "./Stub1.tsx";
import type { ExtensionComponentProps } from "./types.ts";

import { ItemExplorer } from "./item-explorer/ItemExplorer.tsx";

export const getExtensionComponent = async (
    id: string,
): Promise<React.ComponentType<ExtensionComponentProps> | undefined> => {
    switch (id) {
        case "editor":
            return (await import("./editor/Editor.tsx")).Editor;
        case "item-explorer":
            return ItemExplorer;
        case "stub1":
            return makeStub(1);
    }

    return undefined;
};
