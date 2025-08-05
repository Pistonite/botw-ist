import { serial } from "@pistonite/pure/sync";
import type { WxPromise } from "@pistonite/workex";

import {
    extLog,
    type FirstPartyExtension,
    FirstPartyExtensionAdapter,
    shallowEqual,
} from "self::util";

import { SaveViewer } from "./save_viewer.tsx";
import { createSaveViewerStore, type SaveViewerStore } from "./store.ts";

const UUID = "497ce310-293d-40a2-9a8d-fd4ce41c0d69";

export class SaveViewerExtension extends FirstPartyExtensionAdapter implements FirstPartyExtension {
    private store: SaveViewerStore;

    private component: React.FC;

    constructor(standalone: boolean) {
        super(standalone);
        const store = createSaveViewerStore();
        store.subscribe((curr, prev) => {
            if (curr.saveNames !== prev.saveNames || curr.displayedSave !== prev.displayedSave) {
                void this.update(curr.displayedSave);
            }
        });
        this.store = store;
        this.component = () => {
            return <SaveViewer useStore={this.store} />;
        };
    }

    public get Component() {
        return this.component;
    }

    public override async onIconSettingsChanged(
        highRes: boolean,
        animation: boolean,
    ): WxPromise<void> {
        this.store.getState().setItemDisplayProps(!highRes, !animation);
        return {};
    }

    public override async onScriptChanged(): WxPromise<void> {
        void this.update(this.store.getState().displayedSave);
        return {};
    }

    private update = serial({
        fn:
            (checkCancel) =>
            async (selectedSave: string | undefined): Promise<void> => {
                const saveNameForLogging = selectedSave || "<manual save>";
                extLog.info(`${saveNameForLogging}\nupdating save viewer display`);
                const app = this.app;
                if (!app) {
                    return;
                }
                const taskIds = await app.requestNewTaskIds(UUID, 2);
                checkCancel();
                if (taskIds.err) {
                    extLog.error(
                        `${saveNameForLogging}\nfailed to request task ids for save viewer`,
                    );
                    extLog.error(taskIds.err);
                    return;
                }
                const [idListSave, idGetSave] = taskIds.val;
                let PREFIX = `${idListSave} ${saveNameForLogging}`;

                // first request list of saves to make sure it's up to date
                const saves = await app.getSaveNames(idListSave, undefined, undefined);
                if (saves.err) {
                    extLog.error(`${PREFIX}\nfailed to get save names`);
                    extLog.error(saves.err);
                    return;
                }
                if (saves.val.type === "Aborted") {
                    return;
                }
                checkCancel();
                const { saveNames, setSaveNames } = this.store.getState();
                if (!shallowEqual(saveNames, saves.val.value)) {
                    extLog.info(`${PREFIX}\nsave names updated, retriggering the update`);
                    setSaveNames(saves.val.value);
                    // if the names updated, this function will retrigger
                    return;
                }
                PREFIX = `${idGetSave} ${saveNameForLogging}`;
                extLog.info(`${PREFIX}\nrequesting save data from app`);

                const result = await app.getSaveInventory(
                    idGetSave,
                    undefined,
                    undefined,
                    selectedSave,
                );
                checkCancel();
                if (result.err) {
                    extLog.error(`${PREFIX}\nfailed to get save data`);
                    extLog.error(result.err);
                    return;
                }
                if (result.val.type === "Aborted") {
                    return;
                }
                this.store.getState().setDisplayedData(result.val.value);
                extLog.info(`${PREFIX}\nsave data updated successfully`);
            },
    });
}
