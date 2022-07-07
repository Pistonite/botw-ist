import clsx from "clsx";
import { PropsWithChildren } from "react";

type CommandItemProps = PropsWithChildren<{
    isSelected?: boolean,
	comment?:boolean,
    isContextSelected?: boolean,
    onClick: (x: number, y: number)=>void,
    onContextMenu?: (x: number, y: number)=>void
}>;

export const CommandItem: React.FC<CommandItemProps> = ({isSelected, isContextSelected, comment,children, onClick, onContextMenu}) => {
	if(comment){
		return <div className={clsx("CommandItem", isSelected && "CommandItemSelected", isContextSelected&& "CommandItemContextSelected",comment && "CommandItemComment")}>{children}</div>
	}
	return <li 
		className={clsx("CommandItem", isSelected && "CommandItemSelected", isContextSelected&& "CommandItemContextSelected",comment && "CommandItemComment")}
		onClick={(e)=>{
			onClick(e.clientX, e.clientY);
		}}
		onContextMenu={(e)=>{
			if(onContextMenu){
				onContextMenu(e.clientX,e.clientY);
				e.preventDefault();
			}
            
		}}
	>{children}&nbsp;</li>;
};
