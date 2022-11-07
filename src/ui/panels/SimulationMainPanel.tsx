import { ItemList } from "ui/components/item/ItemList";
import { Command } from "core/command/command";
import { SimulationState } from "core/SimulationState";
import { useRuntime } from "data/runtime";
import produce from "immer";
import { DisplayPane } from "ui/surfaces/DisplayPane";
import { Section } from "ui/components";

type SimulationMainPanelProps = {
	displayIndex: number,
	selectedSaveName: string,
	simulationState: SimulationState | null,
	commandText: string,
	command: Command,
	showSaves: boolean,
}
export const SimulationMainPanel: React.FC<SimulationMainPanelProps> = ({
	displayIndex,
	selectedSaveName,
	simulationState,
	commandText,
	command,
	showSaves
}) => {
	const { setting, commandData, setCommandData} = useRuntime();
	const isIconAnimated = setting("animatedIcon");
	const showGameDataSetting = setting("showGameData");
	let showGameData: boolean;
	if(showGameDataSetting === "auto"){
		if(simulationState){
			showGameData = !simulationState.isGameDataSyncedWithPouch();
		}else{
			showGameData = false;
		}
	}else{
		showGameData = showGameDataSetting;
	}

	return (

		<>
			{ showSaves &&
					<div style={{
						maxHeight: 220,
						height: "30vh",
						overflowY: "hidden",
						color: "white",
						backgroundColor: "#262626"
					} }>
						{
							simulationState ?
								<Section titleText="Save Data">
									{
										(()=>{
											if (selectedSaveName === ""){
												const manualSave = simulationState.getManualSave();
												if(manualSave){
													return <ItemList slots={manualSave.getDisplayedSlots(isIconAnimated)}/>;
												}
											}else if(selectedSaveName){
												const namedSaves = simulationState.getNamedSaves();
												if(selectedSaveName in namedSaves){
													const save = namedSaves[selectedSaveName];
													return <ItemList slots={save.getDisplayedSlots(isIconAnimated)}/>;
												}
											}
											return null;
										})()
									}
								</Section>
								:
								<Section titleText="Select an instruction on the left to view it">

								</Section>
						}

					</div>
			}

			<div style={{
				minHeight: "calc( 70vh - 40px )",
				height: showSaves ? "calc( 100vh - 40px - 220px )" : "calc( 100vh - 40px )",
				border: "1px solid black",
				boxSizing: "border-box",
				overflowY: "hidden"
			}}>
				{!!simulationState && command!==undefined &&

						<DisplayPane
							commandText={commandText}
							command={command}
							showGameData={showGameData}
							simulationState={simulationState}
							editCommand={(c)=>{
								setCommandData(produce(commandData, newData=>{
									newData[displayIndex] = c;
								}));
							}}
						/>

				}
			</div>
		</>

	);
};
