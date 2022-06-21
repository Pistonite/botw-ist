import clsx from "clsx";
import { ItemList, ItemListItemProps, ItemListProps } from "components/ItemList";
import { DoubleItemSlot } from "components/ItemSlot";
import { Command } from "core/Command";
import { ItemStack, itemToItemData } from "core/Item";
import { parseCommand } from "core/Parser";
import { Slots } from "core/Slots";

import React, { useEffect, useState } from "react";

type DisplayPaneProps = {
    command: string,
    displayIndex: number,
    slots: Slots,
	savedSlots: Slots,
    numBroken: number,
	overlaySave: boolean,
    editCommand: (c: Command)=>void
}

const stacksToItemListProps = (slots: Slots, numBroken: number, isSave: boolean): ItemListProps => {
	return {
		items: stacksToItemProps(slots.getSlotsRef()),
		numBroken,
		isSave,
	};
};

const stacksToItemProps = (stacks: ItemStack[]): ItemListItemProps[] => {
	return stacks.map(stackToItemProps);
};

const stackToItemProps = ({item, count, equipped}: ItemStack): ItemListItemProps => {
	const data = itemToItemData(item);
	return {image: data.image, count: data.stackable ? count : 0, isEquipped:equipped};
}

export const DisplayPane: React.FC<DisplayPaneProps> = ({command,editCommand,displayIndex, slots, savedSlots, numBroken, overlaySave})=>{
	const [commandString, setCommandString] = useState<string>("");
	const [hasError, setHasError] = useState<boolean>(false);
	const listProps = stacksToItemListProps(slots, numBroken, false);
	const listSaveProps = stacksToItemListProps(savedSlots, 0, true);
	useEffect(()=>{
		if(commandString!==command){
			setCommandString(command);
			setHasError(false);
		}
      
	}, [command, displayIndex]);

	return <div id="DisplayPane" style={{
		width: "calc( 100% - 300px - 5px )",
		float: "right",
		border: "1px solid black",
		boxSizing: "content-box"
	} }>
		<div style={{
			marginBottom: 2,
			boxSizing: "content-box",
			height: "50px"
		} }>
			<input className={clsx("Calamity", hasError && "InputError")} style={{
				marginTop: 2,
				width: "80%",
				height: "40px",
				fontSize: "20pt",
          
			}}value={commandString}
			placeholder="Type command here..."
			onChange={(e)=>{
				const cmdString = e.target.value;
				setCommandString(cmdString);
				const parsedCommand = parseCommand(cmdString);
				if(parsedCommand){
					editCommand(parsedCommand);
					setHasError(false);
				}else{
					setHasError(true);
				}
			}}></input>

		</div>
        {overlaySave ? 
			<div style={{
			borderTop: "1px solid black",
			boxSizing: "content-box",
			height: "calc( ( 99vh - 60px ))",
			overflowY: "auto"
		} }>
			<div>Save / Current</div>
			<div>
			{
				(()=>{
					const doubleSlots: JSX.Element[] = [];
					for(let i=0;i<savedSlots.length && i<slots.length;i++){
						doubleSlots.push(<DoubleItemSlot
							first={{...stackToItemProps(savedSlots.get(i)), isBroken:false, isSave:true}}
							second={{...stackToItemProps(slots.get(i)), isBroken:i>=slots.length-numBroken, isSave:false}}
						/>);
					}
					if(savedSlots.length>slots.length){
						for(let i=slots.length;i<savedSlots.length;i++){
							doubleSlots.push(<DoubleItemSlot
								first={{...stackToItemProps(savedSlots.get(i)), isBroken:false, isSave:true}}
							/>);
						}
					}else if(slots.length > savedSlots.length){
						for(let i=savedSlots.length;i<slots.length;i++){
							doubleSlots.push(<DoubleItemSlot
								second={{...stackToItemProps(slots.get(i)), isBroken:i>=slots.length-numBroken, isSave:false}}
							/>);
						}
					}
					return doubleSlots;
				})()
			}
			</div>
			
		</div>
		
		 :<>
		
			<div style={{
			borderTop: "1px solid black",
			borderBottom: "1px solid black",
			marginBottom: 2,
			boxSizing: "content-box",
			height: "calc( ( 99vh - 60px ) / 2)",
			overflowY: "auto"
		} }>
			<div>Inventory of (Hard) Save</div>
			<ItemList {...listSaveProps}/>
		</div>
		<div style={{
			borderTop: "1px solid black",
			boxSizing: "content-box",
			height: "calc( ( 99vh - 60px ) / 2)",
			overflowY: "auto"
		} }>
			<div>Current Inventory</div>
			<ItemList {...listProps}/>
		</div>
		</>}


	</div>;
};
