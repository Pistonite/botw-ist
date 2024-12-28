import type { ExtensionComponentProps } from "./types.ts";

export const getExtensionComponent = async (id: string): Promise<React.ComponentType<ExtensionComponentProps> | undefined> => {
    switch (id) {
        case "editor":
            return (await import("./editor/EditorComponent.tsx")).EditorComponent;

    }

    return undefined;
}
