import clsx from "clsx";
import React, {
    useEffect,
    useLayoutEffect,
    useMemo,
    useRef,
    useState,
} from "react";
import { ColoredCodeBlocks } from "ui/components";
import { CodeBlock } from "core/command";
import { GetSetPair } from "data/util";

type DivProps = React.DetailedHTMLProps<
    React.HTMLAttributes<HTMLDivElement>,
    HTMLDivElement
>;
type CommandTextAreaProps = {
    blocks: CodeBlock[][];
    large?: boolean;
    disabled?: boolean;
    onAutoResize?: (newHeight: number) => void;
    textAreaId?: string;
    stopPropagation?: boolean;
    removeLines?: boolean;
} & GetSetPair<"value", string[]>;

const MIN_HEIGHT = 30;
const MAX_HEIGHT = 300;

export const CommandTextArea: React.FC<CommandTextAreaProps & DivProps> = ({
    className,
    stopPropagation,
    removeLines,
    large,
    value,
    setValue,
    blocks,
    disabled,
    onAutoResize,
    textAreaId,
    ...restProps
}) => {
    const [cachedValue, setCachedValue] = useState<string>("");
    const textAreaRef = useRef<HTMLTextAreaElement>(null);
    const highlightAreaRef = useRef<HTMLDivElement>(null);
    const [updateHandle, setUpdateHandle] = useState<number | undefined>(
        undefined,
    );

    const splitedCachedValue = useMemo(() => {
        if (removeLines) {
            return [cachedValue];
        }
        return cachedValue.split("\n");
    }, [cachedValue, removeLines]);

    useEffect(() => {
        if (updateHandle === undefined) {
            if (removeLines) {
                setCachedValue(value[0] || "");
            } else {
                setCachedValue(value.join("\n"));
            }
        }
    }, [value, updateHandle]);

    useLayoutEffect(() => {
        if (onAutoResize && textAreaRef.current && highlightAreaRef.current) {
            // Reset height - important to shrink on delete
            textAreaRef.current.style.height = "inherit";
            // Set height
            const initialHeight = Math.max(
                textAreaRef.current.scrollHeight,
                MIN_HEIGHT,
            );
            const height = Math.min(MAX_HEIGHT, initialHeight);
            const scroll = initialHeight > MAX_HEIGHT ? "scroll" : "hidden";
            textAreaRef.current.style.height = `${height}px`;
            textAreaRef.current.style.overflowY = scroll;
            highlightAreaRef.current.style.height = `${height}px`;
            highlightAreaRef.current.style.overflowY = scroll;
            onAutoResize(height);
        }
    }, [cachedValue, onAutoResize]);

    return (
        <div
            className={clsx(className, "CommandInputRoot", large && "Large")}
            {...restProps}
        >
            <div
                ref={highlightAreaRef}
                aria-hidden={true}
                className={clsx("CommandTextArea", large && "Large")}
                style={{
                    zIndex: 0,
                }}
            >
                <ColoredCodeBlocks blocks={blocks} value={splitedCachedValue} />
            </div>
            <textarea
                id={textAreaId}
                ref={textAreaRef}
                disabled={disabled}
                className={clsx("CommandTextArea", large && "Large")}
                spellCheck={false}
                value={cachedValue}
                onChange={(e) => {
                    if (updateHandle) {
                        clearTimeout(updateHandle);
                    }
                    let value = e.target.value;
                    if (removeLines) {
                        value = value.replaceAll("\n", " ");
                    }
                    setCachedValue(value);
                    const newHandle = setTimeout(() => {
                        if (removeLines) {
                            setValue([value]);
                        } else {
                            setValue(e.target.value.split("\n"));
                        }

                        setUpdateHandle(undefined);
                    }, 50);
                    setUpdateHandle(newHandle as unknown as number);
                }}
                onScroll={() => {
                    if (textAreaRef.current && highlightAreaRef.current) {
                        highlightAreaRef.current.scrollTop =
                            textAreaRef.current.scrollTop;
                        highlightAreaRef.current.scrollLeft =
                            textAreaRef.current.scrollLeft;
                    }
                }}
                onKeyDown={(e) => {
                    if (stopPropagation) {
                        e.stopPropagation();
                    }
                }}
            />
        </div>
    );
};
