import produce from "immer";
import React, { useEffect, useMemo, useRef, useState } from "react";

import { CommandItem } from "ui/components";
import { createSimulationState, SimulationState } from "core/SimulationState";
import { useSearchItem } from "data/item";
import { useRuntime } from "core/runtime";
import { ContextMenuState } from "ui/types";

import { 
	ItemExplorerPanel,
	NavPanel,
	HelpPanel,
	ScriptOptionPanel,
	SettingPanel,
	SimMainPanel,
	SimStepsPanel,
	ReferencePage
} from "ui/panels";

import { SavePanel } from "ui/panels/SavePanel";
import { Command, ExecErrorDecorator, MemoizedParser } from "core/command";
const parser = new MemoizedParser();

export const App: React.FC =  () => {

	const { commandData, setCommandData, page, setting } = useRuntime();
	const searchItem = useSearchItem();
	const rawCommands = useMemo(()=>{
		return parser.parseCommands(commandData, searchItem);
	}, [commandData, searchItem]);

	// Layout Components
	// Core Logic States
	const [selectedSaveName, setSelectedSaveName] = useState<string>("");
	const [_displayIndex, setDisplayIndex] = useState<number>(0);
	const [contextMenuState, setContextMenuState] = useState<ContextMenuState>({
		index: -1,
		x: 0,
		y: 0
	});

	const contextMenuRef = useRef<HTMLDivElement>(null);
	// compute props
	const [commands, simulationStates] = useMemo(()=>{
		const simulationStates: SimulationState[] = [];
		const state = createSimulationState();
		const commands: Command[] = [];
		rawCommands.forEach(c=>{
			state.executeCommand(c);
			simulationStates.push(state.deepClone());
			if(state.errors.length === 0){
				commands.push(c);
			}else{
				commands.push(new ExecErrorDecorator(c, state.errors));
			}
		});
		return [commands, simulationStates];
	}, [rawCommands]);

	const displayIndex = (_displayIndex < 0 || _displayIndex >= simulationStates.length)
		? 0
		: _displayIndex;
	const theSimulationState = simulationStates[displayIndex];

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
			showSaves = theSimulationState.numberOfSaves() > 1;
		}else{
			showSaves = showSavesSetting;
		}
	}
	const showGameDataSetting = setting("showGameData");
	let showGameData: boolean;
	if(showGameDataSetting === "auto"){
		showGameData = !theSimulationState.isGameDataSyncedWithPouch();
	}else{
		showGameData = showGameDataSetting;
	}
	
	const saveHeight = 220;
	const fullMainHeight = "calc( 100vh - 40px )";
	const middleHeight = showSaves?`calc( 100vh - 40px - ${saveHeight}px )`:fullMainHeight;


	return (
		<div className='Calamity'>

			<NavPanel />
			<div id="Main" style={{
				position: "absolute",
				top: 40,
				height: fullMainHeight,
				width: "100vw",
				backgroundColor: "#262626",
			}}>
				<div style={{
					height: middleHeight,
					width: "100vw",
					display: "flex" // so they show up side by side
				}}>
					
					<div id="SidePane" style={{
						width: sideWidth,
						height: middleHeight
					}}>
						{
							page !== "#setting" &&
							<SimStepsPanel
								commands={commands}
								displayIndex={displayIndex}
								setDisplayIndex={setDisplayIndex}
								contextMenuState={contextMenuState}
								setContextMenuState={setContextMenuState}
							/>
						}
						{
							page === "#setting" && <SettingPanel />
						}

					</div>
					<div style={{
						position: "absolute",
						height: middleHeight,
						width: `calc( 100vw - ${sideWidth}px)`,
						left: sideWidth
					}}>
					{	(page === "#simulation" || page === "#setting") &&
							
							<SimMainPanel
								commandText={commandData[displayIndex]}
								command={commands[displayIndex]}
								showGameData={showGameData}
								simulationState={theSimulationState}
								editCommand={(c)=>{
									setCommandData(produce(commandData, newData=>{
										newData[displayIndex] = c;
									}));
								}}
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
