import { CommandItem } from "components/CommandItem";
import { Command } from "core/command";
import { SimulationState } from "core/SimulationState";
import { useRuntime } from "data/runtime";
import { GetSetPair } from "data/types";
import produce from "immer";
import { useCallback } from "react";
import { Section } from "ui/components";
import { ContextMenuState } from "ui/types";

type SimulationSidePanelProps = {
	simulationState: SimulationState | null,
	commands: Command[],
	showSaves: boolean,
}
& GetSetPair<"displayIndex", number>
& GetSetPair<"selectedSaveName", string>
& GetSetPair<"contextMenuState", ContextMenuState>;

export const SimulationSidePanel: React.FC<SimulationSidePanelProps> = ({
	simulationState,
	commands,
	selectedSaveName,
	setSelectedSaveName,
	displayIndex,
	setDisplayIndex,
	contextMenuState,
	setContextMenuState,
	showSaves
})=>{

	const {
		commandData,
		setPage,
		setCommandData,
		editing,
		setWarnReadOnly
	} = useRuntime();

	const createNewStep = useCallback(()=>{
		setCommandData(produce(commandData, newData=>{
			newData.push("");
		}));
	}, [commandData]);

	return (
		<>
			{ showSaves &&
				<Section titleText="Saves" style={{
					maxHeight: 220,
					height: "30vh",
					border: "1px solid black",
					boxSizing: "border-box",
					overflowY: "hidden",

				}}
				>

					{
						!!simulationState &&
							<ol>
								{
									!!simulationState.getManualSave() &&
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
									Object.entries(simulationState.getNamedSaves()).map(([name, _gamedata], i)=>
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

				</Section>
			}

			<Section titleText="Steps" style={{
				minHeight: "calc( 70vh - 40px )",
				height: showSaves ? "calc( 100vh - 40px - 220px )" : "calc( 100vh - 40px )",
				border: "1px solid black",
				boxSizing: "border-box",
				overflowY: "hidden"

			}}>
				<ol style={{
				}}>
					{
						commandData.map((c,i)=>
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
									if(editing){
										setContextMenuState({
											index: i,
											x,
											y
										});
									}else{
										setWarnReadOnly(true);
									}

								}}
								key={i}
								isSelected={displayIndex===i}
								isContextSelected={contextMenuState.index===i}
								isComment={c.startsWith("#")}
								useListItem={!c.startsWith("#")}
								isInvalid={commands[i].getError() !== undefined}
							>
								{c}
							</CommandItem>
						)
					}
					<CommandItem onClick={createNewStep} onContextMenu={createNewStep}>(new)</CommandItem>

				</ol>

			</Section>

		</>
	);
};
