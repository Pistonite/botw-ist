import { parseCommand } from "core/command";
import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";

import "./App.css";
import { CommandItem } from "./components/CommandItem";

import { DisplayPane } from "surfaces/DisplayPane";
import { ItemList } from "components/ItemList";
import { TitledList } from "components/TitledList";
import { createSimulationState, SimulationState } from "core/SimulationState";
import { ReferencePage } from "surfaces/ReferencePage";
import { OptionPage } from "surfaces/OptionPage";
import { useSearchItem } from "data/item";
import { GalleryPage } from "surfaces/GalleryPage";

const getDefaultCommandTexts = (): string[]=>{
	const encoded = localStorage.getItem("HDS.CurrentCommandsText");
	if(encoded){
		const lines = encoded.split("\n");
		return lines;
	}
	return [
		"Get 5 Diamond 1 Slate 1 Glider 4 SpiritOrb",
		"Save",
		"# Magically break 4 slots",
		"Break 4 Slots",
		"Reload",
		"Save",
		"Reload",
	];
};

type Setting = {
	interlaceInventory: boolean,
	isIconAnimated: boolean,

}
const getSetting = (): Setting=>{
	const settingString = localStorage.getItem("HDS.Setting");
	const defaultSetting = {
		interlaceInventory: false,
		isIconAnimated: true,
	};
	if(settingString){
		return {
			...defaultSetting,
			...JSON.parse(settingString)
		};
	}
	return defaultSetting;
};

const initialSetting = getSetting();

export const App: React.FC =  () => {
	
	const searchItem = useSearchItem();
	const [page, setPageInState] = useState<string>("#simulation");
	// Option States
	const [interlaceInventory, setInterlaceInventory] = useState<boolean>(initialSetting.interlaceInventory);
	const [isIconAnimated, setIsIconAnimated] = useState<boolean>(initialSetting.isIconAnimated);

	// save settings
	useEffect(()=>{
		localStorage.setItem("HDS.Setting", JSON.stringify({
			interlaceInventory,
			isIconAnimated
		}));
	},[interlaceInventory, isIconAnimated]);
	// Core Logic States
	const [commandTexts, setCommandTexts] = useState<string[]>(getDefaultCommandTexts());
	const [selectedSaveName, setSelectedSaveName] = useState<string>("");
	const [displayIndex, setDisplayIndex] = useState<number>(0);
	const [contextMenuX, setContextMenuX] = useState<number>(0);
	const [contextMenuY, setContextMenuY] = useState<number>(0);
	const [contextIndex, setContextIndex] = useState<number>(-1);

	const contextMenuRef = useRef<HTMLDivElement>(null);
	// compute props

	const parseCommandWithSearch = useCallback((cmd: string)=>{
		return parseCommand(cmd, searchItem);
	}, [searchItem]);

	const commands = useMemo(()=>{
		return commandTexts.map(c=>parseCommandWithSearch(c));
	}, [commandTexts, parseCommandWithSearch]);

	const simulationStates = useMemo(()=>{
		const simulationStates: SimulationState[] = [];
		const state = createSimulationState();
		commands.forEach(c=>{
			state.executeCommand(c);
			simulationStates.push(state.deepClone());
		});
		return simulationStates;
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
				while(nextCommandIndex<commandTexts.length && commands[nextCommandIndex].getError() !== undefined){
					nextCommandIndex++;
				}
				if(nextCommandIndex===commandTexts.length-1){
					const arrCopy = [...commandTexts];
					arrCopy.push("");
					setCommandTexts(arrCopy);
					setDisplayIndex(arrCopy.length-1);
				}else{
					
					setDisplayIndex(Math.min(commandTexts.length-1, nextCommandIndex));
				}
			}else if(e.code==="ArrowUp"){
				let nextCommandIndex = displayIndex-1;
				while(nextCommandIndex>=0 && commands[nextCommandIndex].getError() !== undefined){
					nextCommandIndex--;
				}
				setDisplayIndex(Math.max(0, nextCommandIndex));
			}
		};
	}, [commandTexts, displayIndex, commands]);

	useEffect(()=>{
		const text = commandTexts.join("\n");
		localStorage.setItem("HDS.CurrentCommandsText", text);

	}, [commandTexts]);

	useEffect(()=>{
		if(contextIndex < 0 || contextIndex >= commandTexts.length){
			setContextIndex(-1);
		}else if(contextMenuRef.current){
			const rect = contextMenuRef.current.getBoundingClientRect();
			if (rect.bottom > window.innerHeight){
				setContextMenuY(contextMenuY-rect.height);
			}
		}
	}, [contextMenuRef, contextIndex, commandTexts]);
  
	return (
		<div className='Calamity'>
			
			<div id="NavBar" style={{
				backgroundColor: "#262626",
				color: "#ffffff",
				height: 40
			}}>
				<button className="MainButton" onClick={()=>{
					setPage("#simulation");
				}}>Simulation</button>
				<button className="MainButton" onClick={()=>{
					setPage("#reference");
				}}>Commands</button>
				<button className="MainButton" onClick={()=>{
					setPage("#items");
				}}>Items</button>
				<button className="MainButton" onClick={()=>{
					setPage("#options");
				}}>Options</button>
				Helpful reading for understanding IST: <a href="https://restite.org/reload/#">https://restite.org/reload</a> by savage13
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
									Object.entries(simulationStates[displayIndex].getNamedSaves()).map(([name, _gamedata], i)=>
										<CommandItem 
											key={i}
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
								commandTexts.map((c,i)=>
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
										isComment={c.startsWith("#")}
										useListItem={!c.startsWith("#")}
										isInvalid={commands[i].getError() !== undefined}
									>
										{c}
									</CommandItem>
								)
							}
							<CommandItem onClick={()=>{
								const arrCopy = [...commandTexts];
								arrCopy.push("");
								setCommandTexts(arrCopy);
							}} onContextMenu={()=>{
								const arrCopy = [...commandTexts];
								arrCopy.push("");
								setCommandTexts(arrCopy);
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
														return <ItemList slots={manualSave.getDisplayedSlots(isIconAnimated)}/>;
													}
												}else if(selectedSaveName){
													const namedSaves = simulationStates[displayIndex].getNamedSaves();
													if(selectedSaveName in namedSaves){
														const save = namedSaves[selectedSaveName];
														return <ItemList slots={save.getDisplayedSlots(isIconAnimated)}/>;
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
									isIconAnimated={isIconAnimated}
									command={commandTexts[displayIndex]}
									commandError={commands[displayIndex].getError()}
									simulationState={simulationStates[displayIndex]}
									editCommand={(c)=>{
										const arrCopy = [...commandTexts];
										arrCopy[displayIndex] = c;
										setCommandTexts(arrCopy);
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
					page === "#items" && <GalleryPage isIconAnimated={isIconAnimated}/>
				}
				{
					page === "#options" && 
					<OptionPage 
						interlaceInventory={interlaceInventory}
						setInterlaceInventory={setInterlaceInventory}
						isIconAnimated={isIconAnimated}
						setIsIconAnimated={setIsIconAnimated}
						commandText={commandTexts.join("\n")}
						setCommandText={(value)=>{
							setCommandTexts(value.split("\n"));
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
								const arrCopy = [...commandTexts];
								arrCopy.splice(contextIndex, 0, "");
								setCommandTexts(arrCopy);
								setContextIndex(-1);
							}}>Insert Above</CommandItem>
							<CommandItem onClick={()=>{
								if(contextIndex > 0){
									const arrCopy = [...commandTexts];
									const temp = arrCopy[contextIndex];
									arrCopy[contextIndex] = arrCopy[contextIndex-1];
									arrCopy[contextIndex-1] = temp;
									setCommandTexts(arrCopy);
									setContextIndex(-1);
								}
								
							}}>Move Up</CommandItem>
							<CommandItem onClick={()=>{
								if(confirm("Delete?")){
									setCommandTexts(commandTexts.filter((_,i)=>i!==contextIndex));
									if(displayIndex >= commandTexts.length){
										setDisplayIndex(commandTexts.length-1);
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
