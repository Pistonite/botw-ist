import { Divider, makeStyles } from "@fluentui/react-components";
import { useAllNonPopoutExtensionIds, useCurrentPrimaryExtensionId, useCurrentSecondaryExtensionId, useExtensionStore, usePrimaryExtensionIds, useSecondaryExtensionIds } from "application/extensionStore";
import { ExtensionToolbar } from "./components/ExtensionToolbar";
import { openExtensionPopup } from "application/extensionManager";
import { ExtensionOpenButton } from "./ExtensionOpenButton";
import { memo, useEffect, useState } from "react";
import { isLessProductive } from "./platform";

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

const ExtensionTitlebarConnected: React.FC = () => {
    const styles = useStyles();

    const currentPrimaryId = useCurrentPrimaryExtensionId();
    const currentSecondaryId = useCurrentSecondaryExtensionId();
    const primaryIds = usePrimaryExtensionIds();
    const secondaryIds = useSecondaryExtensionIds();
    const openExtension = useExtensionStore(state => state.open);
    const closePrimary = useExtensionStore(state => state.closePrimary);
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
                onClickPopout={isLessProductive ? undefined : () => {
                    openExtensionPopup(currentPrimaryId);
                    closePrimary();
                }}
                onSelect={(id) => {
                    openExtension(id, "primary");
                }}
            />
        );

    const secondaryToolbar =
                currentSecondaryId && (
                    <ExtensionToolbar 
                        id={currentSecondaryId}
                        allIds={secondaryIds}
                        onClickPopout={isLessProductive ? undefined : () => {
                            openExtensionPopup(currentSecondaryId);
                            closeSecondary();
                        }}
                onSelect={(id) => {
                    openExtension(id, "secondary");
                }}
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
            {
                currentPrimaryId && currentSecondaryId && (
                    <Divider vertical className={styles.divider}/>
                )
            }
            <ExtensionOpenButton />
    </div>
    );

};

export const ExtensionTitlebar = memo(ExtensionTitlebarConnected);

/** Titlebar for mobile platform with simplified controls */
const ExtensionTitlebarMobileConnected: React.FC = () => {
    const styles = useStyles();
    
    const allIds = useAllNonPopoutExtensionIds();
    const currentId = useCurrentPrimaryExtensionId();
    const openExtension = useExtensionStore(state => state.open);


    return (
        <div className={styles.bar}>
                    <ExtensionToolbar 
                        id={currentId}
                        allIds={allIds}
                        onSelect={(id) => {
                    openExtension(id, "primary");
                }}
                fullWidth
                    />
        </div>
    );
};

export const ExtensionTitlebarMobile = memo(ExtensionTitlebarMobileConnected);
