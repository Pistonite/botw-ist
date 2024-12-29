import { makeStub } from "./Stub1.tsx";
import type { ExtensionComponentProps } from "./types.ts";

export const ExtensionIds = ["editor", "stub1", "stub2"] as const;

export const getExtensionComponent = async (id: string): Promise<React.ComponentType<ExtensionComponentProps> | undefined> => {
    switch (id) {
        case "editor":
            return (await import("./editor/EditorComponent.tsx")).EditorComponent;
        case "stub1":
            return makeStub(1);
        case "stub2":
            return makeStub(2);

    }

    return undefined;
}
