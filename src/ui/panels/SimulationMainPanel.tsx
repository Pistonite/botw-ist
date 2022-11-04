import { ItemList } from "components/ItemList";
import { SimulationState } from "core/SimulationState";
import { useRuntime } from "data/runtime";
import produce from "immer";
import { DisplayPane } from "surfaces/DisplayPane";
import { Section } from "ui/components";

type SimulationMainPanelProps = {
	displayIndex: number,
	selectedSaveName: string,
	simulationState: SimulationState | null,
	command?: string,
	commandError?: string,
	showSaves: boolean,
}
export const SimulationMainPanel: React.FC<SimulationMainPanelProps> = ({
	displayIndex,
	selectedSaveName,
	simulationState,
	command,
	commandError,
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
				{!!simulationState && !!command &&

						<DisplayPane
							command={command}
							commandError={commandError}
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
