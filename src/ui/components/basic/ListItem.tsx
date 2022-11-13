import clsx from "clsx";
import React, { PropsWithChildren, useCallback } from "react";

type CommandItemProps = PropsWithChildren<{
	htmlId?: string,
	useListItem?: boolean,
    isSelected?: boolean,
	small?: boolean,
	isInvalid?: boolean,
    isContextSelected?: boolean,
    onClick: (x: number, y: number)=>void,
    onContextMenu?: (x: number, y: number)=>void
}>;

export const CommandItem: React.FC<CommandItemProps> = ({
	htmlId,
	useListItem,
	isSelected,
	isContextSelected,
	small,
	isInvalid,
	onClick,
	onContextMenu,
	children
}) => {
	const className = clsx(
		"ListItem",
		isInvalid && "ListItemInvalid",
		isSelected && "ListItemSelected",
		isContextSelected && "ListItemContextSelected",
		small && "Small",
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
			<div id={htmlId} className={className} onClick={clickHandler} onContextMenu={contextMenuHandler}>
				{children}
			</div>
		);
	}
	return (
		<li id={htmlId} className={className} onClick={clickHandler} onContextMenu={contextMenuHandler}>
			{children}
		</li>
	);
};
