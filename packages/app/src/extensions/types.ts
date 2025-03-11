import type { Extension } from "@pistonite/skybook-api";

export type ExtensionComponentProps = {
    /**
     * If the extension is loaded as part of the app,
     * or in its standalone window
     */
    standalone: boolean;

    /**
     * Callback to connect the extension to the app, once it's ready
     */
    connect: (ext: Extension) => () => void;
};

export type ExtensionComponent = React.ComponentType<ExtensionComponentProps>;
