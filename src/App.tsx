import { Command, CommandBreakSlots, CommandInitialize, CommandNothing, CommandReload, CommandSave, CommandSortKey } from "core/Command";
import { Inventory } from "core/Inventory";
import React, { useEffect, useMemo, useRef, useState } from "react";

import "./App.css";
import { CommandItem } from "./components/CommandItem";

import { DisplayPane } from "surfaces/DisplayPane";
import { Item } from "core/Item";
import { saveAs } from "data/FileSaver";
import { parseCommand } from "core/Parser";

const getDefaultCommands = (): Command[]=>{
	const encoded = localStorage.getItem("HDS.CurrentCommandsText");
	if(encoded){
		const lines = encoded.split("\n");
		return lines.map(l=>parseCommand(l)).filter(c=>c) as Command[];
	}
	return [
		new CommandInitialize([
			{
				item: Item.Diamond,
				count: 5,
				equipped:false,
			},
			{
				item: Item.Slate,
				count: 1,
				equipped:false,
			},
			{
				item: Item.Glider,
				count: 1,
				equipped:false,
			},
			{
				item: Item.SpiritOrb,
				count: 4,
				equipped:false,
			}
		]),
		new CommandBreakSlots(4),
		new CommandReload(),
		new CommandSortKey(),
		new CommandSave(),
		new CommandReload()
	];
};

export const App: React.FC =  () => {
	const [overlaySave, setOverlaySave] = useState<boolean>(false);
	const [commands, setCommands] = useState<Command[]>(getDefaultCommands());
	const [displayIndex, setDisplayIndex] = useState<number>(0);
	const [contextMenuX, setContextMenuX] = useState<number>(0);
	const [contextMenuY, setContextMenuY] = useState<number>(0);
	const [contextMenuShowing, setContextMenuShowing] = useState<boolean>(false);
	const [contextIndex, setContextIndex] = useState<number>(-1);
	// compute props
	const inventories = useMemo(()=>{
		const inventories: Inventory[] = [];
		const inv = new Inventory();
		commands.forEach(c=>{
			c.execute(inv);
			inventories.push(inv.deepClone());
		});
		return inventories;
	}, [commands]);

	useEffect(()=>{
		window.onkeydown=(e)=>{
			if(e.code==="ArrowDown"){
				setDisplayIndex(Math.min(commands.length-1, displayIndex+1));
			}else if(e.code==="ArrowUp"){
				setDisplayIndex(Math.max(0, displayIndex-1));
			}
		};
	}, [commands, displayIndex]);

	useEffect(()=>{
		// const encoded = serializeCommands(commands);
		// localStorage.setItem("HDS.CurrentCommands", encoded);
		const lines = commands.map(c=>c.getDisplayString());
		const text = lines.join("\n");
		localStorage.setItem("HDS.CurrentCommandsText", text);

	}, [commands]);

	const uploadRef = useRef<HTMLInputElement>(null);
  
	return (
		<div className='Calamity'
		>
			<input ref={uploadRef} id="Upload" type="File" hidden onChange={(e)=>{
				const files = e.target.files;
				if(files?.length && files[0]){
					const file = files[0];
					file.text().then(text=>{
						const lines = text.split("\n");
						const parsedCommands: Command[] = lines.map(l=>parseCommand(l)).filter(c=>c) as Command[];
						setDisplayIndex(0);
						setContextIndex(-1);
						setContextMenuShowing(false);
						setCommands(parsedCommands);
					});
				}
			}}/>
			<div id="CommandList" style={{
				width: "300px",
				height: "calc( 100vh - 5px )",
				overflowY: "auto",
		
				float: "left",
				border: "1px solid black",
				boxSizing: "content-box"
			} }>
				<ul style={{
					listStyleType: "none",
					paddingInlineStart: 0
				}}>
					{
						commands.map((c,i)=>
							<CommandItem 
								onClick={()=>setDisplayIndex(i)} 
								onContextMenu={(x,y)=>{
									setContextIndex(i);
									setContextMenuX(x);
									setContextMenuY(y);
									setContextMenuShowing(true);
								}}
								key={i} 
								isSelected={displayIndex===i}
								isContextSelected={contextIndex===i}
								error={inventories[i].isInaccurate()}
							>
								{c.getDisplayString()}
							</CommandItem>
						)
					}
					<CommandItem onClick={()=>{
						const arrCopy = [...commands];
						arrCopy.push(new CommandNothing());
						setCommands(arrCopy);
					}}>(new)</CommandItem>
					<CommandItem onClick={(x,y)=>{
						setContextIndex(-1);
						setContextMenuX(x);
						setContextMenuY(y);
						setContextMenuShowing(true);
					}}>(options)</CommandItem>

				</ul>
			</div>
			{displayIndex >= 0 && displayIndex < commands.length &&
				<DisplayPane 
				overlaySave={overlaySave}
					displayIndex={displayIndex}
					command={commands[displayIndex].getDisplayString()} 
					slots={inventories[displayIndex].getSlots()} 
					savedSlots={inventories[displayIndex].getSavedSlots()}
					numBroken={inventories[displayIndex].getNumBroken()} 
					editCommand={(c)=>{
						const arrCopy = [...commands];
						arrCopy[displayIndex] = c;
						setCommands(arrCopy);
					}}
				/> 
			}

			{
				contextMenuShowing && <div style={{
					position: "absolute",
					top: 0,
					left: 0,
					width: "100vw",
					height: "100vh",
				}} onClick={()=>{
					setContextMenuShowing(false);
					setContextIndex(-1);
				}} onContextMenu={(e)=>{
					setContextMenuShowing(false);
					setContextIndex(-1);
					e.preventDefault();
				}}>
					<div style={{
						position: "absolute",
						top: contextMenuY,
						left: contextMenuX,
						width: "200px",
						backgroundColor: "white",
						border: "1px solid black"
					}}>
						<ul style={{
							margin: 0,
							listStyleType: "none",
							paddingInlineStart: 0
						}}>
							{contextIndex >= 0 ? <>
							<CommandItem onClick={()=>{
								const arrCopy = [...commands];
								arrCopy.splice(contextIndex, 0, new CommandNothing());
								setCommands(arrCopy);
								setContextMenuShowing(false);
								setContextIndex(-1);
							}}>Insert Above</CommandItem>
							<CommandItem error onClick={()=>{
								if(confirm("Delete?")){
									setCommands(commands.filter((_,i)=>i!==contextIndex));
									if(displayIndex >= commands.length){
										setDisplayIndex(commands.length-1);
									}
									setContextMenuShowing(false);
									setContextIndex(-1);
								}
							}}>Delete</CommandItem></> :
							<>
							<CommandItem onClick={()=>{
						setOverlaySave(!overlaySave);
					}}>Toggle Save Overlay</CommandItem>
												<CommandItem onClick={()=>{
						if(uploadRef.current){
							uploadRef.current.click();
						}
					}}>Import</CommandItem>
					<CommandItem onClick={()=>{
						const lines = commands.map(c=>c.getDisplayString());
						const text = lines.join("\n");
						saveAs(text, "dupe.txt");
					}}>Export</CommandItem>
					<CommandItem onClick={()=>{
										alert(`Available Commands:
Initialize X Item1 Y Item2 Z Item3 ...
Break X Slots - add X broken slots
Save
Reload
Sort Key/Material - sort key items or material
Get/Add/Cook/Pickup X ITEM
Remove/Drop/Sell X ITEM From Slot Y
Remove/Sell/Eat MEAL From Slot X

Limitations:
Inventory corruption is not implemented yet

`);
	alert(`Available Items:
Slate
Glider
SpiritOrb
SpeedFood
Lotus
SilentPrincess
Honey
Acorn
FaroshScale
FaroshClaw
FaroshHorn
HeartyBass
Beetle
Opal
Diamond
Tail
Spring
Shaft
Core
Wood
Weapon
		`);
					}}>Reference</CommandItem>

							</>
							
							}
							
						</ul>
					</div>
				</div>
			}
		</div>
	);
};
