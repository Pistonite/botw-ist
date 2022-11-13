import clsx from "clsx";
import { PropsWithChildren, useMemo } from "react";
import { CodeBlock, Colors, parseCommand } from "core/command";
import { useSearchItem } from "data/item";

type ColoredCodeBlocksProps = {
    blocks: CodeBlock[][],
    value: string[]
}

export const ParseCode: React.FC<PropsWithChildren> = ({children}) => {
    if (typeof children !== "string"){
        throw new Error("In ParseCode. Children must be string");
    }
    const search = useSearchItem();
    const text = children;
    const command = useMemo(()=>{
        return parseCommand(text, search);
    }, [text]);
    return (
        <ColoredCodeBlocks
            blocks={[command.codeBlocks]}
            value={[text]}
        />
    );
}

export const ColoredCodeBlocks: React.FC<ColoredCodeBlocksProps> = ({blocks, value}) => {
    return (
        <>
            {
                (()=>{
                    const result: (ColoredCodeBlockProps|null)[] = [];
                    for(let i=0;i<blocks.length&&i<value.length;i++){
                        if(!value[i]){
                            result.push(null);// indicate empty line
                        }
                        result.push({
                            blocks: blocks[i],
                            value: value[i]
                        });
                    }
                    return result.map((props, i)=>(
                        props === null
                            ? <br key={i}/>
                            : <ColoredCodeBlock {...props} key={i} />
                    ))
                })()
            }
        </>
    )
}

type ColoredCodeBlockProps = {
    blocks: CodeBlock[],
    value: string
};

const ColoredCodeBlock: React.FC<ColoredCodeBlockProps> = ({blocks, value})=>{
    const mappedProps  = useMemo(()=>{
        let last: ColoredSingleBlockProps[] = []; // blocks without whitespace
        const result: ColoredSingleBlockProps[][] = [last];

        let currentStart = 0;
        if(blocks.length === 0){
            blocks = [{
                color: "unknown",
                start: 0,
                end: value.length
            }];
        }

        if (blocks.length > 0 && blocks[blocks.length-1].end < value.length){
            blocks = [...blocks, {
                color: "unknown",
                start: blocks[blocks.length-1].end,
                end: value.length
            }];
        }

        blocks.forEach(({color, start, end})=>{
            const toAdd: string[] = [];
            if (start > currentStart){
                toAdd.push(value.substring(start, currentStart));

            }
            toAdd.push(value.substring(start, end));
            toAdd.forEach((sub, i)=>{
                const addColor = (i < toAdd.length - 1)
                    ? "unknown"
                    : color;
                // if sub has spaces, need to break it
                const parts = sub.split(" ");
                parts.forEach((part,i)=>{

                    if(part){
                        last.push({color: addColor, value: part});
                    }

                    if(i<parts.length-1){
                        last = [];
                        result.push([{color: "delimiter", value: " "}]);
                        result.push(last);
                    }

                })
                currentStart = end;
            })
        });
        return result;
    }, [blocks, value]);
    return (
        <pre className="CodeBlockLine">
            {
                mappedProps.map((propss, i)=>(
                    <span key={i}>
                        {
                            propss.map((props, i)=>(
                                <ColoredSingleBlock {...props} key={i}/>
                            ))
                        }
                    </span>

                ))
            }
        </pre>
    );
}

type ColoredSingleBlockProps = {
    color: CodeBlock["color"],
    value: string
}

const ColoredSingleBlock: React.FC<ColoredSingleBlockProps> = ({color, value})=>{
    return (

            <code className={clsx("CodeBlock", Colors[color])}>
                {value}
            </code>


    )
}
