import { Command, CommandNop } from "core/Command";
import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";

import "./App.css";
import { CommandItem } from "./components/CommandItem";

import { DisplayPane } from "surfaces/DisplayPane";
import { parseCommand } from "core/Parser";
import { ItemList } from "components/ItemList";
import { TitledList } from "components/TitledList";
import { createSimulationState, SimulationState } from "core/SimulationState";
import { ReferencePage } from "surfaces/ReferencePage";
import { OptionPage } from "surfaces/OptionPage";

const getDefaultCommands = (): Command[]=>{
	const encoded = localStorage.getItem("HDS.CurrentCommandsText");
	if(encoded){
		const lines = encoded.split("\n");
		return lines.map(parseCommand);
	}
	return [
		parseCommand("Get 5 Diamond 1 Slate 1 Glider 4 SpiritOrb"),
		parseCommand("Save"),
		parseCommand("# Magically break 4 slots"),
		parseCommand("Break 4 Slots"),
		parseCommand("Reload"),
		parseCommand("Save"),
		parseCommand("Reload"),
	]  as Command[];
};

export const App: React.FC =  () => {
	const [page, setPageInState] = useState<string>("#simulation");
	// Option States
	const [interlaceInventory, setInterlaceInventory] = useState<boolean>(false);

	const [commands, setCommands] = useState<Command[]>(getDefaultCommands());
	const [selectedSaveName, setSelectedSaveName] = useState<string>("");
	const [displayIndex, setDisplayIndex] = useState<number>(0);
	const [contextMenuX, setContextMenuX] = useState<number>(0);
	const [contextMenuY, setContextMenuY] = useState<number>(0);
	const [contextIndex, setContextIndex] = useState<number>(-1);

	const contextMenuRef = useRef<HTMLDivElement>(null);
	// compute props
	const simulationStates = useMemo(()=>{
		const simulationStates: SimulationState[] = [];
		const state = createSimulationState();
		commands.forEach(c=>{
			c.execute(state);
			simulationStates.push(state.deepClone());
		});
		return simulationStates;
	}, [commands]);
	const commandText = useMemo(()=>{
		return commands.map(c=>c.getDisplayString()).join("\n");
	}, [commands]);

	const setPage = useCallback((hash: string)=>{
		window.location.hash = hash;
		setPageInState(hash);
	}, []);
	
	useEffect(()=>{
		setPage(window.location.hash || "#simulation");
	}, [window.location.hash]);

	useEffect(()=>{
		window.onkeydown=(e)=>{
			if(e.code==="ArrowDown"){
				let nextCommandIndex = displayIndex+1;
				while(nextCommandIndex<commands.length && !commands[nextCommandIndex].isValid()){
					nextCommandIndex++;
				}
				if(nextCommandIndex===commands.length-1){
					const arrCopy = [...commands];
					arrCopy.push(new CommandNop(""));
					setCommands(arrCopy);
					setDisplayIndex(arrCopy.length-1);
				}else{
					
					setDisplayIndex(Math.min(commands.length-1, nextCommandIndex));
				}
			}else if(e.code==="ArrowUp"){
				let nextCommandIndex = displayIndex-1;
				while(nextCommandIndex>=0 && !commands[nextCommandIndex].isValid()){
					nextCommandIndex--;
				}
				setDisplayIndex(Math.max(0, nextCommandIndex));
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
		if(contextIndex < 0 || contextIndex >= commands.length){
			setContextIndex(-1);
		}else if(contextMenuRef.current){
			const rect = contextMenuRef.current.getBoundingClientRect();
			if (rect.bottom > window.innerHeight){
				setContextMenuY(contextMenuY-rect.height);
			}
		}
	}, [contextMenuRef, contextIndex, commands]);
  
	return (
		<div className='Calamity'>
			
			<div id="NavBar" style={{
				backgroundColor: "#262626",
				height: 40
			}}>
				<button className="MainButton" onClick={()=>{
					setPage("#simulation");
				}}>Simulation</button>
				<button className="MainButton" onClick={()=>{
					setPage("#reference");
				}}>Reference</button>
				<button className="MainButton" onClick={()=>{
					setPage("#options");
				}}>Options</button>
			</div>

			<div id="SidePane" style={{
				width: 300,
				float: "left"
			}}>
				<div style={{
					maxHeight: 220,
					height: "30vh",
					border: "1px solid black",
					boxSizing: "border-box",
					overflowY: "hidden",
					
				}}>
					<TitledList title="Saves">
						{
							displayIndex >=0 && displayIndex < simulationStates.length &&
							<ol>
								{
									!!simulationStates[displayIndex].getManualSave() &&
									<CommandItem 
										onClick={()=>{
											setSelectedSaveName("");
											setPage("#simulation");
										}}  
										useListItem
										isSelected={selectedSaveName===""}

									>
								Manual Save
									</CommandItem>
								}
							
								{
									Object.entries(simulationStates[displayIndex].getNamedSaves()).map(([name, _gamedata])=>
										<CommandItem 
											onClick={()=>{
												setSelectedSaveName(name);
												setPage("#simulation");
											}}  
											isSelected={selectedSaveName===name}
											useListItem
										>
											{name}
										</CommandItem>
									)
								}
							</ol>
						}
					
					</TitledList>
				</div>
				<div style={{
					minHeight: "calc( 70vh - 40px )",
					height: "calc( 100vh - 40px - 220px )",
					border: "1px solid black",
					boxSizing: "border-box",
					overflowY: "hidden"
					
				}}>
					<TitledList title="Instructions">
						<ol style={{
						}}>
							{
								commands.map((c,i)=>
									<CommandItem 
										onClick={()=>{
											setDisplayIndex(i);
											setPage("#simulation");
											const inputField = document.getElementById("CommandInputField");
											if(inputField){
												inputField.focus();
											}
										}} 
										onContextMenu={(x,y)=>{
											setContextIndex(i);
											setContextMenuX(x);
											setContextMenuY(y);
										}}
										key={i} 
										isSelected={displayIndex===i}
										isContextSelected={contextIndex===i}
										isComment={c.getDisplayString().startsWith("#")}
										useListItem={!c.getDisplayString().startsWith("#")}
										isInvalid={!c.isValid()}
									>
										{c.getDisplayString()}
									</CommandItem>
								)
							}
							<CommandItem onClick={()=>{
								const arrCopy = [...commands];
								arrCopy.push(new CommandNop(""));
								setCommands(arrCopy);
							}} onContextMenu={()=>{
								const arrCopy = [...commands];
								arrCopy.push(new CommandNop(""));
								setCommands(arrCopy);
							}}>(new)</CommandItem>

						</ol>
					</TitledList>
					
				</div>
				
			</div>
			<div id="MainPane" style={{
				position: "absolute",
				top: 40,
				right: 0,
				bottom: 0,
				left: 300,
				backgroundColor: "#262626"
			}}>
				{
					page === "#simulation" && <>
						<div style={{
							maxHeight: 220,
							height: "30vh",
							overflowY: "hidden",
							color: "white",
							backgroundColor: "#262626"
						} }>
							{
								displayIndex >= 0 && displayIndex < commands.length ? 
									<TitledList title="Save Data">
										{
											(()=>{
												if (selectedSaveName === ""){
													const manualSave = simulationStates[displayIndex].getManualSave();
													if(manualSave){
														return <ItemList slots={manualSave.getDisplayedSlots()}/>;
													}
												}else if(selectedSaveName){
													const namedSaves = simulationStates[displayIndex].getNamedSaves();
													if(selectedSaveName in namedSaves){
														const save = namedSaves[selectedSaveName];
														return <ItemList slots={save.getDisplayedSlots()}/>;
													}
												}
												return null;
											})()
										}
									</TitledList>
									:
									<TitledList title="Select an instruction on the left to view it">
						
									</TitledList>
							}
						
						</div>
						<div style={{
							minHeight: "calc( 70vh - 40px )",
							height: "calc( 100vh - 40px - 220px )",
							border: "1px solid black",
							boxSizing: "border-box",
							overflowY: "hidden"
						}}>
							{displayIndex >= 0 && displayIndex < commands.length &&
					<DisplayPane 
						overlaySave={interlaceInventory}
						displayIndex={displayIndex}
						command={commands[displayIndex].getDisplayString()}
						simulationState={simulationStates[displayIndex]}
						editCommand={(c)=>{
							const arrCopy = [...commands];
							arrCopy[displayIndex] = c;
							setCommands(arrCopy);
						}}
					/> 
					
							}
						</div>
					</>
				}
				{
					page === "#reference" && <ReferencePage />
				}
				{
					page === "#options" && 
					<OptionPage 
						interlaceInventory={interlaceInventory}
						setInterlaceInventory={setInterlaceInventory}
						commandText={commandText}
						setCommandText={(value)=>{
							if(value !== commandText){
								const commands = value.split("\n").map(parseCommand);
								setCommands(commands);
							}
						}}
					/>
				}
			</div>

			{
				contextIndex >= 0 && contextIndex < commands.length && <div style={{
					position: "absolute",
					top: 0,
					left: 0,
					width: "100vw",
					height: "100vh",
				}} onClick={()=>{
					setContextIndex(-1);
				}} onContextMenu={(e)=>{
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
						
							<CommandItem onClick={()=>{
								const arrCopy = [...commands];
								arrCopy.splice(contextIndex, 0, new CommandNop(""));
								setCommands(arrCopy);
								setContextIndex(-1);
							}}>Insert Above</CommandItem>
							<CommandItem onClick={()=>{
								if(contextIndex > 0){
									const arrCopy = [...commands];
									const temp = arrCopy[contextIndex];
									arrCopy[contextIndex] = arrCopy[contextIndex-1];
									arrCopy[contextIndex-1] = temp;
									setCommands(arrCopy);
									setContextIndex(-1);
								}
								
							}}>Move Up</CommandItem>
							<CommandItem onClick={()=>{
								if(confirm("Delete?")){
									setCommands(commands.filter((_,i)=>i!==contextIndex));
									if(displayIndex >= commands.length){
										setDisplayIndex(commands.length-1);
									}
									setContextIndex(-1);
								}
							}}>Delete</CommandItem>
							
						</ul>
					</div>
				</div>
			}
		</div>
	);
};
