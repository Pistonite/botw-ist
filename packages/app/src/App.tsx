
import { makeStyles } from "@fluentui/react-components"
import { SideToolbar } from "./SideToolbar";
import { useIsShowingExtensionPanel } from "./application/extensionStore";
import { ExtensionPanel } from "ui/ExtensionPanel";
import { useNarrow } from "pure-contrib/narrow";
import { ResizeLayout } from "ui/components/ResizeLayout";
import { useUIStore } from "ui/store";
import { isLessProductive } from "ui/platform";
import { useEffect } from "react";

// const testOnce = () => {
// }

// const doFoo = () => {
//     console.log('foo3')
// }
//
// setInterval(doFoo, 1000) 2

const useStyles = makeStyles({
    root: {
        width: "100vw",
        height: "100vh",
    },
    side: {
        width: "100%",
        height: "100%",
        display: "flex",
        flexDirection: "column",
    }
})

function App() {
    const narrow = useNarrow();
    const styles = useStyles();

    const showExtensionPanel = useIsShowingExtensionPanel();

    const extensionPanelPercentage = useUIStore(state => state.extensionPanelPercentage);
    const setExtensionPanelPercentage = useUIStore(state => state.setExtensionPanelPercentage);

    useEffect(() => {
        console.log('App mounted')
        return () => {
            console.log('App unmounted')
        }
    }, [])

    return (
    <ResizeLayout 
            className={styles.root}
            vertical={narrow || !showExtensionPanel}
            disabled={!showExtensionPanel}
            naturalSize={!showExtensionPanel}
            valuePercent={extensionPanelPercentage}
            setValuePercent={setExtensionPanelPercentage}
            minWidth={330}
            minHeight={45}
            touch={isLessProductive}
        >
            <div className={styles.side}>
                <SideToolbar />
                { showExtensionPanel && <ExtensionPanel />}
            </div>
            <main style={{background: "green", height: "100%"}}>
                main
            </main>
    </ResizeLayout>
    );
    
}

export default App
