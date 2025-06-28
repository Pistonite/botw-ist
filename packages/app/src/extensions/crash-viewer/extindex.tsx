import { useSyncExternalStore } from "react";
import { debounce } from "@pistonite/pure/sync";
import type { WxPromise } from "@pistonite/workex";
import { cell, type Cell } from "@pistonite/pure/memory";

import {
    type FirstPartyExtension,
    FirstPartyExtensionAdapter,
} from "../FirstParty.ts";

import { CrashViewer } from "./CrashViewer.tsx";

const CRASH_VIEWER_UUID = "630c655a-2547-4e40-9b95-5f07a048ed38";

export class CrashViewerExtension
    extends FirstPartyExtensionAdapter
    implements FirstPartyExtension
{
    public updateCrashInfo: () => Promise<void>;
    private crashInfo: Cell<string>;
    private component: React.FC;

    constructor(standalone: boolean) {
        super(standalone);
        this.crashInfo = cell({
            initial: "",
        });
        this.updateCrashInfo = debounce({
            fn: async (): Promise<void> => {
                const app = this.app;
                if (!app) {
                    return;
                }
                const taskId = await app.requestNewTaskId(CRASH_VIEWER_UUID);
                if (taskId.err) {
                    return;
                }
                const result = await app.getCrashInfo(
                    taskId.val,
                    undefined,
                    undefined,
                );
                if (result.err) {
                    return;
                }
                if (result.val.type === "Aborted") {
                    return;
                }
                this.crashInfo.set(result.val.value);
            },
            interval: 100,
        });
        const subscribe = (cb: (x: string) => void) => {
            return this.crashInfo.subscribe(cb);
        };
        this.component = () => {
            // eslint-disable-next-line react-hooks/rules-of-hooks
            const crashInfo = useSyncExternalStore(subscribe, () => {
                return this.crashInfo.get();
            });
            return <CrashViewer crashInfo={crashInfo} />;
        };
    }

    public get Component() {
        return this.component;
    }

    public override async onScriptChanged(): WxPromise<void> {
        void this.updateCrashInfo();
        return {};
    }
}
