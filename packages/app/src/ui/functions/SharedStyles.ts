import { makeStyles, mergeClasses } from "@fluentui/react-components";

// experimental, tailwind-like thing, but uses griffel
// the benefit with griffel is we don't get duplicate CSS rules
// from griffel and tailwind, and LTR/RTL works (which is one of the
// main reasons we use griffel in the first place)
//
// The idea is to have a set of utility classes that are
// defined globally (useSharedStyles hook), and make
// a merge function that uses template literal to merge
// shared and local styles
//
// // declare this somewhere
// const useStyleEngine = makeGale(makeStyles({ ... }));
//
// // in your components
// const m = useStyleEngine();
// const c = useStyles() // local styles;
//
//
// so: className={m("full-wh",c.custom,...)}
// equals to: className={mergeClasses(sharedStyles["full-wh"], c.custom, ...)}
const makeGale = <T extends string>(
    innerHook: () => Record<T, string>,
): (() => GaleFn) => {
    const classNamesObjCache = new Map<
        Record<string, string>,
        Map<string, string[]>
    >();

    return () => {
        // -- hook scope --
        const classnames = innerHook();
        let cached = classNamesObjCache.get(classnames);
        if (!cached) {
            cached = new Map();
            classNamesObjCache.set(classnames, cached);
        }
        const c = cached;

        return (sharedStylesString, values) => {
            let parsed = c.get(sharedStylesString);
            if (!parsed) {
                parsed = [];
                const parts = sharedStylesString.split(" ");
                for (const p of parts) {
                    const slotName = p.trim();
                    if (!slotName) {
                        continue;
                    }
                    if (!(slotName in classnames)) {
                        console.error(
                            `[gale] ${slotName} not found in shared styles`,
                        );
                        continue;
                    }
                    parsed.push(classnames[slotName as T]);
                }
                c.set(sharedStylesString, parsed);
            }

            if (Array.isArray(values)) {
                return mergeClasses(...parsed, ...values);
            }
            return mergeClasses(...parsed, values);
        };
    };
};

type GaleFn = (
    sharedStylesString: string,
    values?: string | false | undefined | (string | false | undefined)[],
) => string;

export const useStyleEngine = makeGale(
    makeStyles({
        "wh-100v": {
            width: "100vw",
            height: "100vh",
        },
        "wh-100": {
            width: "100%",
            height: "100%",
        },
        "h-100": {
            height: "100%",
        },
        "w-100": {
            width: "100%",
        },
        flex: {
            display: "flex",
        },
        "flex-col": {
            display: "flex",
            flexDirection: "column",
        },
        "flex-row": {
            display: "flex",
            flexDirection: "row",
        },
        "flex-1": {
            flex: 1,
        },
        "flex-grow": {
            flexGrow: 1,
        },
        "flex-noshrink": {
            flexShrink: 0,
        },
        "flex-centera": {
            alignItems: "center",
        },
        "flex-centerj": {
            justifyContent: "center",
        },
        "flex-center": {
            alignItems: "center",
            justifyContent: "center",
        },
        "flex-end": {
            justifyContent: "flex-end",
        },
        "flex-wrap": {
            flexWrap: "wrap",
        },
        "scrollbar-thin": {
            scrollbarWidth: "thin",
        },
        "overflow-y-auto": {
            overflowY: "auto",
        },
        "overflow-x-auto": {
            overflowX: "auto",
        },
        "overflow-auto": {
            overflow: "auto",
        },
        "overflow-x-hidden": {
            overflowX: "hidden",
        },
        "overflow-y-hidden": {
            overflowY: "hidden",
        },
        "overflow-hidden": {
            overflow: "hidden",
        },
        "overflow-visible": {
            overflow: "visible",
        },
        "pos-rel": {
            position: "relative",
        },
        "pos-abs": {
            position: "absolute",
        },
        "all-sides-0": {
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
        },
        "border-box": {
            boxSizing: "border-box",
        },
        "cursor-pointer": {
            cursor: "pointer",
        },
        "min-h-0": {
            minHeight: 0,
        },
        "min-w-0": {
            minWidth: 0,
        },
        "max-h-0": {
            maxHeight: 0,
        },
        "max-w-0": {
            maxWidth: 0,
        },
    }),
);
