import {
    FileEditor,
    type SingleFileEditorState,
    EditorEventType,
    StatusItemPreset,
} from "@pistonite/intwc";
import type { WxPromise } from "@pistonite/workex";

import type { ExtensionApp, ItemDragData, SessionMode } from "@pistonite/skybook-api";
import { CookEffectNames, ItemDropZone, translateActorOrAsIs } from "@pistonite/skybook-itemsys";
import { useUITranslation } from "skybook-localization";

import { FirstPartyExtensionAdapter, type FirstPartyExtension } from "#util";

import { init, setApp, updateScriptInApp } from "./init.ts";

void init();

export class EditorExtension extends FirstPartyExtensionAdapter implements FirstPartyExtension {
    private editor: SingleFileEditorState | undefined;
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
                    <FileEditor
                        onCreated={(editor) => {
                            void this.attachEditor(editor);
                            return undefined;
                        }}
                        language="skybook"
                        filename="script.skyb"
                        statusLeft={[
                            StatusItemPreset.DiagnosticErrors,
                            StatusItemPreset.DiagnosticWarnings,
                        ]}
                        statusRight={[StatusItemPreset.Position, StatusItemPreset.WordWrap]}
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
        this.editor?.overrideOptions({ readOnly: isReadonly });
        return {};
    }

    /**
     * Attach the extension instance to an editor.
     * Automatically detaches other previously attached editor
     */
    public async attachEditor(editor: SingleFileEditorState): Promise<() => void> {
        editor.overrideOptions({ readOnly: this.isReadonly });
        if (this.editor === editor) {
            return this.detachEditor || (() => {});
        }
        const detachEditor = this.detachEditor;
        this.detachEditor = undefined;
        detachEditor?.();

        this.editor = editor;

        const updateHandler = () => {
            updateScriptInApp(editor.getContent(), editor.getCursorCharOffset());
        };

        const unsubscribeContentChange = editor.subscribe(
            EditorEventType.ContentChanged,
            updateHandler,
        );
        const unsubscribeCursorPositionChange = editor.subscribe(
            EditorEventType.CursorPositionChanged,
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
        this.editor.setContent(script);
        return {};
    }

    private onDropItem(data: ItemDragData) {
        const editor = this.editor;
        if (!editor) {
            return;
        }

        const script = editor.getContent();
        const cursorOffset = editor.getCursorCharOffset();
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
        editor.setContent(before + itemScript + after);
        editor.setCursorCharOffset(newOffset);
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
            if (data.keepLocation) {
                position = data.position;
            }
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
