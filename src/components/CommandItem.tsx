import clsx from "clsx";
import React, { PropsWithChildren, useCallback } from "react";

type CommandItemProps = PropsWithChildren<{
	useListItem?: boolean,
    isSelected?: boolean,
	isComment?: boolean,
	isInvalid?: boolean,
    isContextSelected?: boolean,
    onClick: (x: number, y: number)=>void,
    onContextMenu?: (x: number, y: number)=>void
}>;

export const CommandItem: React.FC<CommandItemProps> = ({
	useListItem,
	isSelected, 
	isContextSelected, 
	isComment, 
	isInvalid,
	onClick, 
	onContextMenu, 
	children
}) => {
	const className = clsx(
		"CommandItem", 
		isSelected && "CommandItemSelected", 
		isContextSelected && "CommandItemContextSelected",
		isComment && "CommandItemComment", 
		isInvalid && !isComment && "CommandItemInvalid",
	);

	const clickHandler = useCallback((e: React.MouseEvent)=>{
		onClick(e.clientX, e.clientY);
	}, [onClick]);
	const contextMenuHandler = useCallback((e: React.MouseEvent)=>{
		if(onContextMenu){
			onContextMenu(e.clientX,e.clientY);
			e.preventDefault();
		}
	}, [onContextMenu]);

	if(!useListItem){
		return (
			<div className={className} onClick={clickHandler} onContextMenu={contextMenuHandler}>
				{children}&nbsp;
			</div>
		);
	}
	return (
		<li className={className} onClick={clickHandler} onContextMenu={contextMenuHandler}>
			{children}&nbsp;
		</li>
	);
};
