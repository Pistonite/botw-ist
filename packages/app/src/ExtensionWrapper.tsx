import { Extension } from "@pistonite/skybook-extension-api"

export type ExtensionComponentProps = {
    extClient: Extension
}

export type ExtensionComponent = React.ComponentType<ExtensionComponentProps>
