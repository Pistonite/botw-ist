import type { WorkexPromise } from "@pistonite/workex";

import type { FirstPartyExtension } from "./FirstParty.ts";

export class Stub1Extension implements FirstPartyExtension {
    onAppConnectionEstablished(): void {
        // do nothing
    }
    private component = () => {
        return <div>Stub 1</div>;
    };
    async onDarkModeChanged(): WorkexPromise<void> {
        return {};
    }
    async onLocaleChanged(): WorkexPromise<void> {
        return {};
    }
    async onScriptChanged(): WorkexPromise<void> {
        return {};
    }
    public get Component() {
        return this.component;
    }
}
