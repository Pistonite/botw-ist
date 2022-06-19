import clsx from "clsx";
import { PropsWithChildren } from "react";

type CommandItemProps = PropsWithChildren<{
    isSelected?: boolean,
    isContextSelected?: boolean,
    error?: boolean,
    onClick: ()=>void,
    onContextMenu?: (x: number, y: number)=>void
}>;

export const CommandItem: React.FC<CommandItemProps> = ({isSelected, isContextSelected,error, children, onClick, onContextMenu}) => {
    return <li 
        className={clsx("CommandItem", isSelected && "CommandItemSelected", isContextSelected&& "CommandItemContextSelected",error && "CommandItemError")}
        onClick={onClick}
        onContextMenu={(e)=>{
            if(onContextMenu){
                console.log(e);
                onContextMenu(e.clientX,e.clientY);
                e.preventDefault();
            }
            
        }}
    >{children}&nbsp;</li>
}

