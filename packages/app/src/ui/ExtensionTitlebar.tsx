import { Button, Divider, Dropdown, makeStyles, Tooltip } from "@fluentui/react-components";
import { useExtensionStore } from "application/extensionStore";
import { ExtensionToolbar } from "./components/ExtensionToolbar";
import { openExtensionPopup } from "application/extensionManager";
import { WindowDevTools20Regular } from "@fluentui/react-icons";
import { useUITranslation } from "skybook-localization";
import { ExtensionOpenButton } from "./ExtensionOpenButton";
import { useEffect, useRef, useState } from "react";

const useStyles = makeStyles({
    bar: {
        display: "flex",
        flexDirection: "row",
        gap: "4px",
    },
    divider: {
        maxWidth: "2px",
    }
});

export const ExtensionTitlebar: React.FC = () => {
    const t = useUITranslation();
    const styles = useStyles();

    const currentPrimaryId = useExtensionStore(state => state.currentPrimary);
    const primaryIds = useExtensionStore(state => state.primaryIds);
    const currentSecondaryId = useExtensionStore(state => state.currentSecondary);
    const secondaryIds = useExtensionStore(state => state.secondaryIds);
    const setPrimary = useExtensionStore(state => state.setPrimary);
    const closePrimary = useExtensionStore(state => state.closePrimary);
    const setSecondary = useExtensionStore(state => state.setSecondary);
    const closeSecondary = useExtensionStore(state => state.closeSecondary);

    const [barRef, setBarRef] = useState<HTMLDivElement | null>(null);
    const [isNarrowLayout, setIsNarrowLayout] = useState(false);

    useEffect(() => {
        if (!barRef) {
            return;
        }
        const observer = new ResizeObserver(() => {
            const {width} = barRef.getBoundingClientRect();
            setIsNarrowLayout(width<660);
        });
        observer.observe(barRef);
        return () => observer.disconnect();
    }, [barRef]);

    const primaryToolbar =
                currentPrimaryId && (
                    <ExtensionToolbar 
                        id={currentPrimaryId}
                        allIds={primaryIds}
                        onClickPopout={() => {
                            openExtensionPopup(currentPrimaryId);
                            closePrimary();
                        }}
                        onSelect={setPrimary}
                    />
                );

    const secondaryToolbar =
                currentSecondaryId && (
                    <ExtensionToolbar 
                        id={currentSecondaryId}
                        allIds={secondaryIds}
                        onClickPopout={() => {
                            openExtensionPopup(currentSecondaryId);
                            closeSecondary();
                        }}
                        onSelect={setSecondary}
                        onClickClose={closeSecondary}
                    />);

    const twoLine = isNarrowLayout && currentPrimaryId && currentSecondaryId;
    if (twoLine) {
        return (
        <div ref={setBarRef}>
    <div className={styles.bar}>
                    {primaryToolbar}
            <ExtensionOpenButton />
    </div>
                {secondaryToolbar}
        </div>
        );
    }

    return (
    <div ref={setBarRef} className={styles.bar}>
            {primaryToolbar}
            {
                currentPrimaryId && currentSecondaryId && (
                    <Divider vertical className={styles.divider}/>
                )
            }
            {secondaryToolbar}
            <Divider vertical className={styles.divider}/>
            <ExtensionOpenButton />
    </div>
    );

};
