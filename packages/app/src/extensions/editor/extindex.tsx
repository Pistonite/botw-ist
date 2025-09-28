import { getNormalizedPath, CodeEditor, type CodeEditorApi } from "@pistonite/intwc";
import type { WxPromise } from "@pistonite/workex";

import type { ExtensionApp, ItemDragData, SessionMode } from "@pistonite/skybook-api";
import { CookEffectNames, ItemDropZone, translateActorOrAsIs } from "@pistonite/skybook-itemsys";
import { useUITranslation } from "skybook-localization";

import { FirstPartyExtensionAdapter, type FirstPartyExtension } from "self::util";

import { init, setApp, updateScriptInApp } from "./init.ts";

void init();

const FILE = getNormalizedPath("script.skyb");

export class EditorExtension extends FirstPartyExtensionAdapter implements FirstPartyExtension {
    private editor: CodeEditorApi | undefined;
    private scriptChangedBeforeEditorReady: string | undefined;
    private detachEditor: (() => void) | undefined;
    private isReadonly = false;

    private component: React.FC;

    constructor(standalone: boolean) {
        super(standalone);

        const C = () => {
            const t = useUITranslation();
            return (
                <ItemDropZone
                    getHint={() => t("drop_target.editor")}
                    onDropItem={(item) => this.onDropItem(item)}
                    style={{ height: "100%" }}
                >
                    <CodeEditor
                        onCreated={(editor) => {
                            void this.attachEditor(editor);
                            return undefined;
                        }}
                    />
                </ItemDropZone>
            );
        };
        this.component = C;
    }

    public get Component() {
        return this.component;
    }

    public override onAppConnectionEstablished(app: ExtensionApp): void {
        super.onAppConnectionEstablished(app);
        setApp(app);
    }

    public override async onAppModeChanged(mode: SessionMode): WxPromise<void> {
        const isReadonly = mode === "read-only";
        this.isReadonly = isReadonly;
        this.editor?.setReadonly(isReadonly);
        return {};
    }

    /**
     * Attach the extension instance to an editor.
     * Automatically detaches other previously attached editor
     */
    public async attachEditor(editor: CodeEditorApi): Promise<() => void> {
        editor.setReadonly(this.isReadonly);
        if (this.editor === editor) {
            return this.detachEditor || (() => {});
        }
        const detachEditor = this.detachEditor;
        this.detachEditor = undefined;
        detachEditor?.();

        this.editor = editor;

        const updateHandler = (filename: string) => {
            if (filename !== FILE) {
                return;
            }
            updateScriptInApp(editor.getFileContent(FILE), editor.getCursorOffset() || 0);
        };

        const unsubscribeContentChange = editor.subscribe("content-changed", updateHandler);
        const unsubscribeCursorPositionChange = editor.subscribe(
            "cursor-position-changed",
            updateHandler,
        );

        this.detachEditor = () => {
            this.detachEditor = undefined;
            unsubscribeContentChange();
            unsubscribeCursorPositionChange();
            this.editor = undefined;
        };
        if (this.scriptChangedBeforeEditorReady !== undefined) {
            const script = this.scriptChangedBeforeEditorReady;
            this.scriptChangedBeforeEditorReady = undefined;
            await this.onScriptChanged(script);
        } else if (this.app) {
            const script = await this.app.getScript();
            if (script.val && this.editor) {
                await this.onScriptChanged(script.val);
            }
        }
        return this.detachEditor || (() => {});
    }

    public override async onScriptChanged(script: string): WxPromise<void> {
        if (!this.editor) {
            this.scriptChangedBeforeEditorReady = script;
            return {};
        }
        if (!this.editor.getFiles().includes(FILE)) {
            this.editor.openFile(FILE, script, "skybook");
        } else {
            this.editor.setFileContent(FILE, script);
        }
        return {};
    }

    private onDropItem(data: ItemDragData) {
        const editor = this.editor;
        if (!editor) {
            return;
        }
        if (editor.getCurrentFile() !== FILE) {
            return;
        }

        const script = editor.getFileContent(FILE);
        const cursorOffset = editor.getCursorOffset() || script.length;
        let before = script.substring(0, cursorOffset);
        let after = script.substring(cursorOffset);
        const itemScript = getScriptFromDragData(data);
        let newOffset = cursorOffset + itemScript.length;
        // ensure the item has spaces around it
        if (before && !before.match(/\s$/)) {
            before += " ";
            newOffset += 1;
        }
        if (after && !after.match(/^\s/)) {
            after = " " + after;
        }
        editor.setFileContent(FILE, before + itemScript + after);
        editor.setCursorOffset(newOffset);
        updateScriptInApp(before + itemScript + after, newOffset);
    }
}

const getScriptFromDragData = (data: ItemDragData) => {
    // we only extract actor, amount and effect id from the drag data
    let amount = 1;
    let actorName = "";
    let effectId = 0;
    // if the item has a position, then we don't use effect Id
    let position: [number, number] | undefined = undefined;
    switch (data.type) {
        case "search": {
            actorName = data.payload.actor;
            effectId = data.payload.cookEffect;
            break;
        }
        case "pouch": {
            actorName = data.payload.common.actorName;
            effectId = data.payload.data.effectId;
            position = data.position;
            const itemType = data.payload.itemType;
            if (
                itemType === 2 ||
                itemType === 7 ||
                (itemType === 8 && !actorName.startsWith("Item_Cook_")) ||
                actorName === "Obj_KorokNuts" ||
                actorName === "Obj_DungeonClearSeal"
            ) {
                amount = data.payload.common.value;
            }
            break;
        }
        case "gdt": {
            actorName = data.payload.common.actorName;
            const gdtData = data.payload.data;
            switch (gdtData.type) {
                case "food": {
                    effectId = gdtData.info.effectId;
                    if (!actorName.startsWith("Item_Cook_")) {
                        amount = data.payload.common.value;
                    }
                    break;
                }
                case "none": {
                    if (!actorName.startsWith("Armor_")) {
                        amount = data.payload.common.value;
                    }
                }
            }
            break;
        }
        case "overworld": {
            actorName = data.payload.actor;
            break;
        }
    }

    let amountString = "";
    if (amount > 1) {
        amountString = `${amount} `;
    }

    let itemName: string;
    // if the item is an armor, since the upgraded armor
    // all has the same name, we don't use localized name for accuracy
    if (actorName.startsWith("Armor_")) {
        itemName = `<${actorName}>`;
    } else {
        const name = translateActorOrAsIs(actorName);
        if (name === actorName) {
            itemName = `<${actorName}>`;
        } else {
            itemName = `"${name}"`;
        }
    }

    const itemMeta: string[] = [];
    if (position) {
        const [tab, slot] = position;
        itemMeta.push(`tab=${tab}`);
        itemMeta.push(`slot=${slot}`);
    } else {
        if (effectId) {
            const effectName = CookEffectNames[effectId];
            if (effectName && effectName !== "LifeRecover") {
                itemMeta.push(`effect=${effectName.toLowerCase()}`);
            }
        }
    }

    if (itemMeta.length) {
        return `${amountString}${itemName}[${itemMeta.join(", ")}]`;
    }

    return amountString + itemName;
};
