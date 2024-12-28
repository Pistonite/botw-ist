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

import { Button, makeStyles } from "@fluentui/react-components"
import { useRef, useState } from "react";
import { create } from "zustand"
import { SideToolbar } from "./SideToolbar";
import { setPrimary, useExtensionStore } from "./extensions/extensionStore";
import { ExtensionWindow } from "./extensions/ExtensionWindow";
import { RuntimeApiClient } from "./runtime/interfaces/RuntimeApi.send";

import RuntimeWorker from "./worker.ts?worker";

let runtime: RuntimeApiClient | undefined = undefined;

async function initWorker() {
    const worker = new RuntimeWorker();
    const _runtime = new RuntimeApiClient({worker});
    await _runtime.handshake().established();
    runtime = _runtime;
}
initWorker();

type RootLayoutState = {
    /** Percentage of widget area in horizontal mode */
    horizontalWidget: number;
    /** Percentage of widget area in vertical mode */
    verticalWidget: number;
    /** Percentage of primary widget in horizontal mode */
    primaryWidget: number;

    resizeMainLayoutHorizontal: (percent: number) => void;
    resizeMainLayoutVertical: (percent: number) => void;
}

const useRootLayout = create<RootLayoutState>((set) => ({
    horizontalWidget: 40,
    verticalWidget: 40,
    primaryWidget: 50,

    resizeMainLayoutHorizontal: (percent: number) => {
            set({ horizontalWidget: percent })
    }
    ,
    resizeMainLayoutVertical: (percent: number) => {
        set({ verticalWidget: percent })
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
    widget: {
        position: "relative",
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
    }
})

function App() {
    const styles = useRootStyles();
    const [resizing, setResizing] = useState<number[] | undefined>(undefined);
    const widgetRef = useRef<HTMLDivElement>(null);
    const horizontalWidget = useRootLayout((state) => state.horizontalWidget)
    const verticalWidget = useRootLayout((state) => state.verticalWidget)
    const resizeMainLayoutHorizontal = useRootLayout((state) => state.resizeMainLayoutHorizontal)
    const resizeMainLayoutVertical = useRootLayout((state) => state.resizeMainLayoutVertical)

    const primaryIds = useExtensionStore((state) => state.primaryIds)
    const currentPrimary = useExtensionStore((state) => state.currentPrimary)
    

  return (
    <div className={styles.root}


                    onMouseUp={(e) => {
                        setResizing(undefined)
                    }}
                    onMouseMove={(e) => {
                        if (!resizing || !widgetRef.current) {
                            return
                        }
                        const [startX, startY, startWidth, startHeight] = resizing;
                        const deltaX = e.clientX - startX;
                        const deltaY = e.clientY - startY;
                        if (window.innerWidth < 800) {
                            const newHeight = startHeight + deltaY;
                            resizeMainLayoutVertical((newHeight / window.innerHeight) * 100)
                        } else {
                            const newWidth = startWidth + deltaX;
                            resizeMainLayoutHorizontal((newWidth / window.innerWidth) * 100)
                        }
                    }}

        >
      <div ref={widgetRef} className={styles.widget} style={{
                minWidth: `${horizontalWidget}%`,
                minHeight: `${verticalWidget}%`,
            }}>
                <SideToolbar />
                <Button onClick={async () => {
                    const result = await runtime?.parseScript("init 1 apple 2 orange[foo=bar] 3 <Hello>");
                    console.log(result);
                }} />
                <ExtensionWindow 
                    currentId={currentPrimary} 
                    ids={primaryIds} 
                    onSelect={(id) => setPrimary(id)} 
                    primary 
                />
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
      </div>
      <main className={styles.main} style={{background: "green"}}>
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
                main
      </main>
    </div>
  )
}

export default App
