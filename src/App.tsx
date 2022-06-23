import { Command, CommandBreakSlots, CommandInitialize, CommandNothing, CommandReload, CommandSave, CommandSortKey } from "core/Command";
import { Inventory } from "core/Inventory";
import React, { useEffect, useMemo, useRef, useState } from "react";

import "./App.css";
import { CommandItem } from "./components/CommandItem";

import { DisplayPane, stacksToItemListProps } from "surfaces/DisplayPane";
import { Item } from "core/Item";
import { saveAs } from "data/FileSaver";
import { parseCommand } from "core/Parser";
import { ItemList } from "components/ItemList";
import { TitledList } from "components/TitledList";

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

const tempSaveInventory = new Inventory();
tempSaveInventory.rawInit([
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
	{item: Item.Acorn, count: 55, equipped: false},
])
const listProps = stacksToItemListProps(tempSaveInventory.getSlots(), 0, true);


export const App: React.FC =  () => {
	const [overlaySave, setOverlaySave] = useState<boolean>(false);
	const [commands, setCommands] = useState<Command[]>(getDefaultCommands());
	const [displayIndex, setDisplayIndex] = useState<number>(0);
	const [contextMenuX, setContextMenuX] = useState<number>(0);
	const [contextMenuY, setContextMenuY] = useState<number>(0);
	const [contextMenuShowing, setContextMenuShowing] = useState<boolean>(false);
	const [contextIndex, setContextIndex] = useState<number>(-1);

	const uploadRef = useRef<HTMLInputElement>(null);
	const contextMenuRef = useRef<HTMLDivElement>(null);
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

	useEffect(()=>{
		if(contextMenuRef.current && contextMenuShowing){
			const rect = contextMenuRef.current.getBoundingClientRect();
			if (rect.bottom > window.innerHeight){
				setContextMenuY(contextMenuY-rect.height);
			}
		}
	}, [contextMenuRef, contextMenuShowing]);
  
	return (
		<div className='Calamity'>
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
			<div id="NavBar" style={{
				height: 40
			}}>
				<button>Simulation</button>
				<button>Reference</button>
				<button>Options</button>
			</div>

			<div id="SidePane" style={{
				width: 300,
				float: "left"
			}}>
				<div style={{
					maxHeight: 220,
					height: "30vh",
					border: "1px solid black",
					boxSizing: "content-box",
					overflowY: "hidden"
					
				}}>
					<TitledList title="Saves">
					<ol>
						<CommandItem 
								onClick={()=>{}}  
								comment={false}
							>
								Manual Save
							</CommandItem>
							<CommandItem 
								onClick={()=>{}}  
								comment={false}
							>
								Auto Save 1
							</CommandItem>
							<CommandItem 
								onClick={()=>{}}  
								comment={false}
							>
								Auto Save 2
							</CommandItem>
							<CommandItem 
								onClick={()=>{}}  
								comment={false}
							>
								Auto Save 3
							</CommandItem>
							<CommandItem 
								onClick={()=>{}}  
								comment={false}
							>
								Auto Save 4
							</CommandItem>
							<CommandItem 
								onClick={()=>{}}  
								comment={false}
							>
								Auto Save 5
							</CommandItem>
					</ol>
					</TitledList>
				</div>
				<div style={{
					minHeight: "calc( 70vh - 45px )",
					height: "calc( 100vh - 45px - 220px )",
					border: "1px solid black",
					boxSizing: "content-box",
					overflowY: "hidden"
					
				}}>
					<TitledList title="Instructions">
					<ol style={{
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
								comment={c.getDisplayString().startsWith("#")}
							>
								{c.getDisplayString()}
							</CommandItem>
						)
					}
					<CommandItem onClick={()=>{
						const arrCopy = [...commands];
						arrCopy.push(new CommandNothing());
						setCommands(arrCopy);
					}} onContextMenu={()=>{
						const arrCopy = [...commands];
						arrCopy.push(new CommandNothing());
						setCommands(arrCopy);
					}}>(new)</CommandItem>
					<CommandItem onClick={(x,y)=>{
						setContextIndex(-1);
						setContextMenuX(x);
						setContextMenuY(y);
						setContextMenuShowing(true);
					}} onContextMenu={(x,y)=>{
						setContextIndex(-1);
						setContextMenuX(x);
						setContextMenuY(y);
						setContextMenuShowing(true);
					}}>(options)</CommandItem>

				</ol>
					</TitledList>
					
				</div>
				
			</div>
			<div id="MainPane" style={{
				width: "calc ( 100% - 300px )"
			}}>
				<div style={{
					maxHeight: 220,
					height: "30vh",
					border: "1px solid black",
					boxSizing: "content-box",
					overflowY: "hidden"
				} }>
					<TitledList title="Save Data">
						<ItemList {...listProps}/>
					</TitledList>
					
					
				</div>
				<div >
			{displayIndex >= 0 && displayIndex < commands.length &&
				<DisplayPane 
					overlaySave={overlaySave}
					displayIndex={displayIndex}
					command={commands[displayIndex].getDisplayString()}
					orbs={inventories[displayIndex].getTurnedInOrbs()}  
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
			</div>
			</div>




			{/* <div id="SavePane" style={{
				height: "200px"
			}}>
				
				
			</div>
			<div id="InstructionPane" style={{

			}}> */}
			{/* <div id="CommandList" style={{
				width: "300px",
				height: "calc( 60vh - 5px )",
				overflowY: "auto",
		
				
				border: "1px solid black",
				boxSizing: "content-box"
			} }>
				
				
				
			</div> */}
			
			
			{/* </div> */}
			

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
					<div ref={contextMenuRef} style={{
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
								<CommandItem onClick={()=>{
									if(contextIndex > 0){
										const arrCopy = [...commands];
										const temp = arrCopy[contextIndex];
										arrCopy[contextIndex] = arrCopy[contextIndex-1];
										arrCopy[contextIndex-1] = temp;
										setCommands(arrCopy);
										setContextMenuShowing(false);
										setContextIndex(-1);
									}
								
								}}>Move Up</CommandItem>
								<CommandItem onClick={()=>{
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
