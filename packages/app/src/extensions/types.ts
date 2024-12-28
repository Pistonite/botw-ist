import { Application, Extension } from "@pistonite/skybook-extension-api"

export type ExtensionComponentProps = {
    app: Application
}

export type ExtensionComponent = React.ComponentType<ExtensionComponentProps>

export type ExtensionMetadata = {
    //type: "builtin";
    id: string;

    render: React.ComponentType;
}
