import clsx from "clsx";
import React, { useLayoutEffect, useRef, useState } from "react";
import { ColoredCodeBlocks } from "ui/components";
import { CodeBlock } from "core/command";
import { GetSetPair } from "data/util";

type DivProps = React.DetailedHTMLProps<React.HTMLAttributes<HTMLDivElement>, HTMLDivElement>;
type CommandTextAreaProps = {
    blocks: CodeBlock[][],
    large?: boolean,
    disabled?: boolean,
    onAutoResize?: (newHeight: number)=>void,
    textAreaId?: string,
    stopPropagation?:boolean,
} & GetSetPair<"value", string[]>;

const MIN_HEIGHT = 30;

export const CommandTextArea: React.FC<CommandTextAreaProps & DivProps> = ({
    className, stopPropagation,
    large, value, setValue, blocks, disabled, onAutoResize, textAreaId, ...restProps
}) => {
    const textAreaRef = useRef<HTMLTextAreaElement>(null);
    const highlightAreaRef = useRef<HTMLDivElement>(null);

    const joinedValue = value.join("\n");
    useLayoutEffect(()=>{
        if(onAutoResize && textAreaRef.current && highlightAreaRef.current){
            // Reset height - important to shrink on delete
            textAreaRef.current.style.height = "inherit";
            // Set height
            const height = Math.max(
                textAreaRef.current.scrollHeight,
                MIN_HEIGHT
            );
            textAreaRef.current.style.height = `${height}px`;
            highlightAreaRef.current.style.height = `${height}px`;
            onAutoResize(height);
        }
        
        
    }, [value, onAutoResize]);
    //const rootStyle = scrollBehavior === "expand" ? {height: "unset"} : {};
    
    return (
        <div className={clsx(className, "CommandInputRoot", large && "Large")} {...restProps}>
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
                id={textAreaId}
                ref={textAreaRef}
                disabled={disabled}
                className={clsx("CommandTextArea", large && "Large")}
                spellCheck={false}
                value={joinedValue}
                onChange={(e)=>{
                    setValue(e.target.value.split("\n"));
                    // console.log(e.target.rows);
                    // if(onHeightChange){
                    //     onHeightChange(e.target.scrollHeight);
                    // }
                }}
                onScroll={()=>{
                    if(textAreaRef.current && highlightAreaRef.current){
                        highlightAreaRef.current.scrollTop = textAreaRef.current.scrollTop;
                        highlightAreaRef.current.scrollLeft = textAreaRef.current.scrollLeft;
                    }
                }}
                onKeyDown={(e)=>{
                    if(stopPropagation){
                        e.stopPropagation();
                    }
                }}
            />
            
            
        </div>
    );
}
