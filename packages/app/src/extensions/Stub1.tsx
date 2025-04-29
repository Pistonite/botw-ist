import type { WxPromise } from "@pistonite/workex";

import type { FirstPartyExtension } from "./FirstParty.ts";

export class Stub1Extension implements FirstPartyExtension {
    onAppConnectionEstablished(): void {
        // do nothing
    }
    private component = () => {
        return <div>Stub 1</div>;
    };
    async onDarkModeChanged(): WxPromise<void> {
        return {};
    }
    async onLocaleChanged(): WxPromise<void> {
        return {};
    }
    async onScriptChanged(): WxPromise<void> {
        return {};
    }
    async onIconSettingsChanged(): WxPromise<void> {
        return {};
    }
    public get Component() {
        return this.component;
    }
}
