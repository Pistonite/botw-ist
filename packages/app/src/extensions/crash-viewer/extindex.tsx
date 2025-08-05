import { useSyncExternalStore } from "react";
import { serial } from "@pistonite/pure/sync";
import { cell, type Cell } from "@pistonite/pure/memory";
import type { WxPromise } from "@pistonite/workex";

import { type FirstPartyExtension, FirstPartyExtensionAdapter } from "self::util";

import { CrashViewer } from "./crash_viewer.tsx";

const UUID = "630c655a-2547-4e40-9b95-5f07a048ed38";

export class CrashViewerExtension
    extends FirstPartyExtensionAdapter
    implements FirstPartyExtension
{
    private crashInfo: Cell<string>;
    private component: React.FC;

    constructor(standalone: boolean) {
        super(standalone);
        this.crashInfo = cell({
            initial: "",
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

    private updateCrashInfo = serial({
        fn: (checkCancel) => async (): Promise<void> => {
            const app = this.app;
            if (!app) {
                return;
            }
            const taskId = await app.requestNewTaskIds(UUID, 1);
            checkCancel();
            if (taskId.err) {
                return;
            }
            const result = await app.getCrashInfo(taskId.val[0], undefined, undefined);
            checkCancel();
            if (result.err) {
                return;
            }
            if (result.val.type === "Aborted") {
                return;
            }
            this.crashInfo.set(result.val.value);
        },
    });
}
