import clsx from "clsx";
import { useRef } from "react";
import { ColoredCodeBlocks } from "ui/components";
import { CodeBlock } from "core/command";
import { GetSetPair } from "data/util";

type DivProps = React.DetailedHTMLProps<React.HTMLAttributes<HTMLDivElement>, HTMLDivElement>;
type CommandTextAreaProps = {
    blocks: CodeBlock[][],
    large?: boolean,
    disabled?: boolean,
    scrollBehavior: "scroll" | "expand"
} & GetSetPair<"value", string[]>;

export const CommandTextArea: React.FC<CommandTextAreaProps & DivProps> = ({
    className, large, value, setValue, blocks, disabled, scrollBehavior, ...restProps
}) => {
    const textAreaRef = useRef<HTMLTextAreaElement>(null);
    const highlightAreaRef = useRef<HTMLDivElement>(null);

    const joinedValue = value.join("\n");
    const rootStyle = scrollBehavior === "expand" ? {height: "unset"} : {};
    
    return (
        <div className={clsx(className, "CommandInputRoot", large && "Large")} {...restProps} style={rootStyle}>
            <div className={clsx(large && "Large")} style= {{
                pointerEvents: "none",
                opacity: 0,
                maxHeight: 290,
            }}>
                <ColoredCodeBlocks blocks={blocks} value={value} />
            </div>
            <div 
                ref={highlightAreaRef}
                aria-hidden={true} 
                className={clsx("CommandTextArea", large && "Large")}
                style={{
                    zIndex: 0
                }}
            > 
                <ColoredCodeBlocks blocks={blocks} value={value} />
            </div>
            <textarea
                ref={textAreaRef}
                disabled={disabled}
                className={clsx("CommandTextArea", large && "Large")}
                style={{
                    zIndex: 1
                }}
                spellCheck={false}
                value={joinedValue}
                onChange={(e)=>{
                    setValue(e.target.value.split("\n"));
                }}
                onScroll={()=>{
                    if(textAreaRef.current && highlightAreaRef.current){
                        highlightAreaRef.current.scrollTop = textAreaRef.current.scrollTop;
                        highlightAreaRef.current.scrollLeft = textAreaRef.current.scrollLeft;
                    }
                }}
            />
            
            
        </div>
    );
}
