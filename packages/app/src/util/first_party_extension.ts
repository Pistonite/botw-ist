import { setDark, setLocale } from "@pistonite/pure/pref";
import { cell, type Cell } from "@pistonite/pure/memory";
import type { WxPromise } from "@pistonite/workex";

import type { ExtensionApp, ItemDragData, SessionMode } from "@pistonite/skybook-api";
import type { ExtensionModule } from "@pistonite/skybook-api/client";

/**
 * Adapter for the first-party extensions with common
 * functionality for running both in standalone and embedded mode.
 *
 * This serves as the base class for first party extensions to reduce
 * boilerplate. yes it's inheritance, it's fine
 */
export class FirstPartyExtensionAdapter implements ExtensionModule {
    protected app: ExtensionApp | undefined;
    private isPopout: boolean;
    private itemDragData: Cell<ItemDragData | undefined>;

    constructor(isPopout: boolean) {
        this.app = undefined;
        this.isPopout = isPopout;
        this.itemDragData = cell({ initial: undefined });
    }

    public async onDarkModeChanged(dark: boolean): WxPromise<void> {
        if (this.isPopout) {
            setDark(dark);
        }
        return {};
    }
    public async onLocaleChanged(locale: string): WxPromise<void> {
        if (this.isPopout) {
            setLocale(locale);
        }
        return {};
    }
    public async onAppModeChanged(_mode: SessionMode): WxPromise<void> {
        return {};
    }
    public async onScriptChanged(_script: string, _charPos: number): WxPromise<void> {
        return {};
    }
    public async onIconSettingsChanged(_highRes: boolean, _animation: boolean): WxPromise<void> {
        return {};
    }
    public async onItemDragChanged(item: ItemDragData | undefined): WxPromise<void> {
        if (this.isPopout) {
            this.itemDragData.set(item);
        }
        return {};
    }
    public getItemDragData() {
        return this.itemDragData;
    }
    public onAppConnectionEstablished(app: ExtensionApp): void {
        this.app = app;
    }
}

export type FirstPartyExtension = ExtensionModule & {
    get Component(): React.FC;
    getItemDragData(): Cell<ItemDragData | undefined>;
};
