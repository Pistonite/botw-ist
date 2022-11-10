import produce from "immer";
import React, { useEffect, useMemo, useRef, useState } from "react";

import "./App.css";
import { CommandItem } from "ui/components";
import { createSimulationState, SimulationState } from "core/SimulationState";
import { useSearchItem } from "data/item";
import { useRuntime } from "data/runtime";
import { ContextMenuState } from "ui/types";

import { 
	ItemExplorerPanel,
	NavPanel,
	HelpPanel,
	ScriptOptionPanel,
	SettingPanel,
	SimulationMainPanel,
	SimulationSidePanel,
	ReferencePage
} from "ui/panels";

import { parseCommand } from "core/command/parsev2";
import { SavePanel } from "ui/panels/SavePanel";

export const App: React.FC =  () => {

	const { commandData, setCommandData, page, setting } = useRuntime();
	const searchItem = useSearchItem();

	// Layout Components
	// Core Logic States
	const [selectedSaveName, setSelectedSaveName] = useState<string>("");
	const [displayIndex, setDisplayIndex] = useState<number>(0);
	const [contextMenuState, setContextMenuState] = useState<ContextMenuState>({
		index: -1,
		x: 0,
		y: 0
	});

	const contextMenuRef = useRef<HTMLDivElement>(null);
	// compute props

	const commands = useMemo(()=>{
		return commandData.map(c=> parseCommand(c, searchItem));
	}, [commandData, searchItem]);

	const simulationStates = useMemo(()=>{
		const simulationStates: SimulationState[] = [];
		const state = createSimulationState();
		commands.forEach(c=>{
			state.executeCommand(c);
			simulationStates.push(state.deepClone());
		});
		return simulationStates;
	}, [commands]);
	const theSimulationState = displayIndex >=0 && displayIndex < simulationStates.length
		? simulationStates[displayIndex]
		: null;

	useEffect(()=>{
		window.onkeydown=(e)=>{
			if(e.code==="ArrowDown"){
				let nextCommandIndex = displayIndex+1;
				while(nextCommandIndex<commandData.length && commands[nextCommandIndex].shouldSkipWithKeyboard){
					nextCommandIndex++;
				}
				if(nextCommandIndex>=commandData.length-1){
					setCommandData(produce(commandData, newData=>{
						newData.push("");
					}))
					setDisplayIndex(commandData.length);
				}else{

					setDisplayIndex(Math.min(commandData.length-1, nextCommandIndex));
				}
			}else if(e.code==="ArrowUp"){
				let nextCommandIndex = displayIndex-1;
				while(nextCommandIndex>=0 && commands[nextCommandIndex].shouldSkipWithKeyboard){
					nextCommandIndex--;
				}
				setDisplayIndex(Math.max(0, nextCommandIndex));
			}
		};
	}, [commandData, displayIndex, commands, setCommandData]);

	useEffect(()=>{
		if(contextMenuState.index >= commandData.length){
			setContextMenuState({
				index: -1,
				x: 0,
				y: 0
			});
		}else if(contextMenuRef.current){
			const rect = contextMenuRef.current.getBoundingClientRect();
			if (rect.bottom > window.innerHeight){
				setContextMenuState(produce(contextMenuState, newState=>{
					newState.y = contextMenuState.y-rect.height;
				}));
			}
		}
	}, [contextMenuRef, contextMenuState, commandData]);

	const sideWidth = page === "#setting" ? 500 : 300;
	const showSavesSetting = setting("showSaves");
	let showSaves: boolean = false;
	if(page === "#simulation"){
		if(showSavesSetting === "auto"){
			if(theSimulationState){
				showSaves = theSimulationState.numberOfSaves() > 1;
			}else{
				showSaves = false;
			}
	
		}else{
			showSaves = showSavesSetting;
		}
	}
	
	

	return (
		<div className='Calamity'>

			<NavPanel />
			<div id="Main" style={{
				height: "calc( 100vh - 40px )",
				backgroundColor: "#262626",
			}}>
				<div style={{
					height: showSaves?"calc( 100vh - 40px - 220px )":"calc( 100vh - 40px )"
				}}>
					<div style={{
						display: "flex",
						height: "100%"
					}}>
						<div id="SidePane" style={{
							flexShrink: 0,
							width: sideWidth,
						}}>
							{
								page !== "#setting" &&
								<SimulationSidePanel
									commands={commands}
									displayIndex={displayIndex}
									setDisplayIndex={setDisplayIndex}
									selectedSaveName={selectedSaveName}
									setSelectedSaveName={setSelectedSaveName}
									contextMenuState={contextMenuState}
									setContextMenuState={setContextMenuState}
									simulationState={theSimulationState}
									showSaves={showSaves}
								/>
							}
							{
								page === "#setting" && <SettingPanel />
							}

						</div>
						<div style={{
							flexGrow: 1
						}}>
						{	(page === "#simulation" || page === "#setting") &&
								<SimulationMainPanel
									displayIndex={displayIndex}
									selectedSaveName={selectedSaveName}
									command={commands[displayIndex]}
									commandText={commandData[displayIndex]}
									simulationState={theSimulationState}
									showSaves={showSaves}
								/>
							}
							{
								page === "#reference" && <ReferencePage />
							}
							{
								page === "#items" && <ItemExplorerPanel />
							}
							{
								page === "#options" && <ScriptOptionPanel />
							}
							{
								page === "#help" && <HelpPanel />
							}
						</div>
						
							
						
					</div>
					
				</div>
				{
					showSaves && 
					<div style={{
						height: 220
					}}>
						<SavePanel 
							selectedSaveName={selectedSaveName}
							setSelectedSaveName={setSelectedSaveName}
							simulationState={theSimulationState}
							showSaves={showSaves}
						/>
					</div>
				}
			</div>

			

			{
				contextMenuState.index >= 0 && contextMenuState.index < commands.length && <div style={{
					position: "absolute",
					top: 0,
					left: 0,
					width: "100vw",
					height: "100vh",
					
					color: "white"
				}} onClick={()=>{
					setContextMenuState({
						index: -1,
						x: 0,
						y:0
					});
				}} onContextMenu={(e)=>{
					setContextMenuState({
						index: -1,
						x: 0,
						y:0
					});
					e.preventDefault();
				}}>
					<div ref={contextMenuRef} style={{
						position: "fixed",
						top: contextMenuState.y,
						left: contextMenuState.x,
						width: "200px",
						backgroundColor: "#262626",
						border: "1px solid white"
					}}>
						<ul style={{
							margin: 0,
							listStyleType: "none",
							paddingInlineStart: 0
						}}>

							<CommandItem onClick={()=>{
								setCommandData(produce(commandData, newData=>{
									newData.splice(contextMenuState.index, 0, "");
								}));
								setContextMenuState({
									index: -1, x: 0, y: 0
								});
							}}>Insert Above</CommandItem>
							<CommandItem onClick={()=>{
								if(contextMenuState.index > 0){
									setCommandData(produce(commandData, newData=>{
										newData[contextMenuState.index] = commandData[contextMenuState.index-1];
										newData[contextMenuState.index-1] = commandData[contextMenuState.index];
									}));
									setContextMenuState({
										index: -1, x: 0, y: 0
									});
								}

							}}>Move Up</CommandItem>
							<CommandItem onClick={()=>{
								if(confirm("Delete?")){
									setCommandData(produce(commandData, newData=>{
										newData.splice(contextMenuState.index, 1);
									}));
									setContextMenuState({
										index: -1, x: 0, y: 0
									});
								}
							}}>Delete</CommandItem>

						</ul>
					</div>
				</div>
			}
		</div>
	);
};
