import {
    mergeClasses,
    Button,
    Caption2,
    makeStyles,
    Menu,
    MenuGroup,
    MenuGroupHeader,
    MenuItemRadio,
    MenuList,
    MenuPopover,
    MenuTrigger,
    Tooltip,
} from "@fluentui/react-components";
import {
    BeakerEdit20Filled,
    BeakerEdit20Regular,
    EditProhibited20Filled,
    EditProhibited20Regular,
    SaveEdit20Regular,
} from "@fluentui/react-icons";
import { memo } from "react";
import { useDark } from "@pistonite/pure-react";

import type { SessionMode } from "@pistonite/skybook-api";
import { useUITranslation } from "skybook-localization";

import { useDebouncedHasUnsavedChanges, usePersistStore, useSessionStore } from "self::application";
import { useStyleEngine } from "self::util";

const useStyles = makeStyles({
    warningDark: {
        color: "orange",
    },
    warningLight: {
        color: "darkred",
    },
    readonlyDark: {
        backgroundColor: "orange",
        color: "black",
        "&:hover": {
            backgroundColor: "#FFB867",
            color: "black",
        },
        "&:hover:active": {
            backgroundColor: "#DE8600",
            color: "black",
        },
    },
    readonlyLight: {
        backgroundColor: "#ff6666",
        color: "black",
        "&:hover": {
            backgroundColor: "red",
            color: "black",
        },
        "&:hover:active": {
            backgroundColor: "darkred",
            color: "black",
        },
    },
    editonlyDark: {
        color: "lightgreen",
    },
    editonlyLight: {
        color: "green",
    },
    editonlyUnsaved: {
        top: "-4px",
        right: "-4px",
    },
});

const ModeSwitcherImpl: React.FC = () => {
    const t = useUITranslation();
    const c = useStyles();
    const dark = useDark();
    const mode = useSessionStore((state) => state.mode);
    const setModeToLocal = useSessionStore((state) => state.setModeToLocal);
    const setModeToEditOnly = useSessionStore((state) => state.setModeToEditOnly);
    const setModeToReadOnly = useSessionStore((state) => state.setModeToReadOnly);
    return (
        <Menu
            checkedValues={{ mode: [mode] }}
            onCheckedValueChange={(_, { name, checkedItems }) => {
                if (name !== "mode") {
                    return;
                }
                const newMode = checkedItems[0] || "";
                if (newMode === mode) {
                    return;
                }
                if (newMode === "edit-only") {
                    const { savedScript } = usePersistStore.getState();
                    setModeToEditOnly(savedScript);
                    return;
                }
                if (newMode === "read-only") {
                    setModeToReadOnly(undefined);
                    return;
                }
                setModeToLocal();
            }}
        >
            <MenuTrigger>
                <Tooltip
                    relationship="label"
                    content={t("menu.header.mode_current", {
                        mode: t(`menu.mode.${ModeMap[mode]}.title`),
                    })}
                >
                    <Button
                        className={mergeClasses(
                            mode === "read-only" && (dark ? c.readonlyDark : c.readonlyLight),
                            mode === "edit-only" && (dark ? c.editonlyDark : c.editonlyLight),
                        )}
                        icon={<ModeIcon mode={mode} isHeader />}
                        appearance={mode !== "read-only" ? "subtle" : "outline"}
                    />
                </Tooltip>
            </MenuTrigger>
            <MenuPopover>
                <MenuList>
                    <MenuGroup>
                        <MenuGroupHeader>{t("menu.header.mode")}</MenuGroupHeader>
                        {(["local", "edit-only", "read-only"] as const).map((m) => (
                            <MenuItemRadio
                                key={m}
                                subText={<ModeDesc mode={m} />}
                                icon={<ModeIcon mode={m} />}
                                name="mode"
                                value={m}
                            >
                                {t(`menu.mode.${ModeMap[m]}.title`)}
                            </MenuItemRadio>
                        ))}
                    </MenuGroup>
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};
export const ModeSwitcher = memo(ModeSwitcherImpl);

const ModeDesc: React.FC<{ mode: SessionMode }> = ({ mode }) => {
    const t = useUITranslation();
    const c = useStyles();
    const dark = useDark();
    const currentMode = useSessionStore((state) => state.mode);
    const activeScript = useSessionStore((state) => state.activeScript);
    const savedScript = usePersistStore((state) => state.savedScript);
    if (mode === "local" && currentMode !== "local" && activeScript !== savedScript) {
        return (
            <>
                <Caption2 block>{t(`menu.mode.${ModeMap[mode]}.desc`)}</Caption2>
                <Caption2 block className={dark ? c.warningDark : c.warningLight}>
                    {t(`menu.mode.${ModeMap[mode]}.warning`)}
                </Caption2>
            </>
        );
    }
    return <Caption2>{t(`menu.mode.${ModeMap[mode]}.desc`)}</Caption2>;
};

const ModeMap = {
    local: "auto_saved",
    "edit-only": "not_saving",
    "read-only": "view_only",
} as const;
const ModeIcon: React.FC<{ mode: SessionMode; isHeader?: boolean }> = ({ mode, isHeader }) => {
    const hasUnsavedChanges = useDebouncedHasUnsavedChanges();
    const m = useStyleEngine();
    const c = useStyles();
    if (mode === "local") {
        return <SaveEdit20Regular />;
    }
    if (mode === "read-only") {
        return isHeader ? <EditProhibited20Filled /> : <EditProhibited20Regular />;
    }
    if (!isHeader) {
        return <BeakerEdit20Regular />;
    }
    return (
        <span className={m("pos-rel")}>
            <BeakerEdit20Filled />
            {hasUnsavedChanges && <span className={m("pos-abs", c.editonlyUnsaved)}>*</span>}
        </span>
    );
};
