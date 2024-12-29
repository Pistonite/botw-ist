// import './App.css'
// import { CodeEditor } from '@pistonite/intwc'
//
// const CODE=`
// function test() {
//     let notconst = 1;
//     const a = 1;
//     const f = console.log;
//     console.log('Hello, World!');
//
//     type Foo = {
//         readonly a: number;
//     };
//
//     const foo: Foo = { a: 1, bar: f };
//
//     self.aIsReadonly = foo.a;
// }
// `

import { Button, makeStyles, mergeClasses } from "@fluentui/react-components"
import { useEffect, useRef, useState } from "react";
import { create } from "zustand"
import { SideToolbar } from "./SideToolbar";
import { useExtensionStore } from "./application/extensionStore";
import { ExtensionWindow } from "./extensions/ExtensionWindow";
import { ExtensionPanel } from "ui/ExtensionPanel";

type RootLayoutState = {
    /** Percentage of widget area in horizontal mode */
    sideAreaPercentage: number;
    /** Percentage of primary widget in horizontal mode */
    primaryWidget: number;

    resizeMainLayout: (percent: number) => void;
}

const useRootLayout = create<RootLayoutState>((set) => ({
    sideAreaPercentage: 40,
    primaryWidget: 50,

    resizeMainLayout: (percent: number) => {
            set({ sideAreaPercentage: percent })
    }
}))

const useRootStyles = makeStyles({
    root: {
        width: "100vw",
        height: "100vh",
        display: "flex",
        "@media screen and (max-width: 800px)": {
            flexDirection: "column",
        },
    },
    rootNoExtensionPanel: {
        flexDirection: "column",
    },
    widget: {
        position: "relative",
        minWidth: "320px",
        // minWidth: "40%",
        // minHeight: "40%",
        
    },
    main: {
        position: "relative",
        flex: 1,
    },
    drag: {
        position: "absolute",
        bottom: 0,
        right: 0,
        height: "100%",
        width: "3px",
        "@media screen and (max-width: 800px)": {
            width: "100%",
            height: "3px",
            cursor: "ns-resize",
        },
        cursor: "ew-resize",
        // zIndex: 10000,
    },
    drag2: {
        position: "absolute",
        top: 0,
        left: 0,
        height: "100%",
        width: "3px",
        "@media screen and (max-width: 800px)": {
            width: "100%",
            height: "3px",
            cursor: "ns-resize",
        },
        cursor: "ew-resize",
        // zIndex: 10000,
    }
})

function App() {
    const styles = useRootStyles();
    const [resizing, setResizing] = useState<number[] | undefined>(undefined);
    const widgetRef = useRef<HTMLDivElement>(null);
    const sideAreaPercentage = useRootLayout((state) => state.sideAreaPercentage)
    const resizeMainLayout = useRootLayout((state) => state.resizeMainLayout)

    // const primaryIds = useExtensionStore((state) => state.primaryIds)
    // const currentPrimary = useExtensionStore((state) => state.currentPrimary)

    const currentPrimaryId = useExtensionStore(state => state.currentPrimary);
    const currentSecondaryId = useExtensionStore(state => state.currentSecondary);
    const showExtensionPanel = currentPrimaryId || currentSecondaryId;

    useEffect(() => {
        if (!widgetRef.current) {
            return;
        }
        if (!showExtensionPanel) {
            widgetRef.current.style.width = "100%";
            widgetRef.current.style.height = "";
            return;
        }
        const listener = () => {
            if (!widgetRef.current) {
                return;
            }
            const widget = widgetRef.current;
            if (window.innerWidth < 800) {
                widget.style.height = `${sideAreaPercentage}%`;
                widget.style.width = "100%";
            } else {
                widget.style.width = `${sideAreaPercentage}%`;
                widget.style.height = "100%";
            }
        };
        listener();
        window.addEventListener("resize", listener);
        return () => {
            window.removeEventListener("resize", listener);
        }
    }, [sideAreaPercentage, showExtensionPanel])
    

  return (
    <div className={mergeClasses(styles.root, !showExtensionPanel && styles.rootNoExtensionPanel)}

            onMouseLeave={(e) => {
                setResizing(undefined)
            }}

                    onMouseUp={(e) => {
                        setResizing(undefined)
                    }}
                    onMouseMove={(e) => {
                        if (!showExtensionPanel || !resizing || !widgetRef.current) {
                            return
                        }
                        const [startX, startY, startWidth, startHeight] = resizing;
                        const deltaX = e.clientX - startX;
                        const deltaY = e.clientY - startY;
                        if (window.innerWidth < 800) {
                            const newHeight = startHeight + deltaY;
                            resizeMainLayout((newHeight / window.innerHeight) * 100)
                        } else {
                            const newWidth = startWidth + deltaX;
                            resizeMainLayout((newWidth / window.innerWidth) * 100)
                        }
                    }}

        >
            {
            }
      <div ref={widgetRef} className={styles.widget}>
                    <SideToolbar />
                { showExtensionPanel && <>
                    <ExtensionPanel />
                {/*
                    <ExtensionWindow 
                        currentId={currentPrimary} 
                        ids={primaryIds} 
                        onSelect={(id) => setPrimary(id)} 
                        primary 
                    />*/}
                <div 
                    className={styles.drag} 
                    onMouseDown={(e) => {
                        if (!widgetRef.current) {
                            return
                        }
                        e.preventDefault()
                        e.stopPropagation()
                        const widget = widgetRef.current.getBoundingClientRect();
                        setResizing([e.clientX, e.clientY, widget.width, widget.height])
                    }}
                ></div>
                    </>
                }
      </div>
      <main className={styles.main} style={{background: "green"}}>
                {
                    showExtensionPanel && 
                        <div 
                            className={styles.drag2} 
                            onMouseDown={(e) => {
                                if (!widgetRef.current) {
                                    return
                                }
                                e.preventDefault()
                                e.stopPropagation()
                                const widget = widgetRef.current.getBoundingClientRect();
                                setResizing([e.clientX, e.clientY, widget.width, widget.height])
                            }}
                        ></div>
                }
                main
      </main>
    </div>
  )
}

export default App
