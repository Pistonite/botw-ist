import { CommandItem } from "ui/components/basic/ListItem";
import { Command } from "core/command/command";
import { SimulationState } from "core/SimulationState";
import { useRuntime } from "core/runtime";
import { GetSetPair } from "data/util";
import produce from "immer";
import { useCallback } from "react";
import { ColoredCodeBlocks, Section } from "ui/components";
import { ContextMenuState } from "ui/types";

type SimStepsPanelProps = {
	commands: Command[],
}
& GetSetPair<"displayIndex", number>

& GetSetPair<"contextMenuState", ContextMenuState>;

export const SimStepsPanel: React.FC<SimStepsPanelProps> = ({
	commands,
	displayIndex,
	setDisplayIndex,
	contextMenuState,
	setContextMenuState,
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
	}, [commandData, setCommandData]);

	return (
		
			<Section titleText="Steps" style={{
				height: "100%"
			}}>
				<ol >
					{
						commandData.map((c,i)=>
							<CommandItem
								onClick={()=>{
									setDisplayIndex(i);
									setPage("#simulation");
									const inputField = document.getElementById("SimulationCommandTextField");
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
								small={false}
								useListItem={true}
								isInvalid={false}
							>
								<ColoredCodeBlocks blocks={[commands[i].codeBlocks]} value={[c]} />
							</CommandItem>
						)
					}
					<CommandItem onClick={createNewStep} onContextMenu={createNewStep}>(new)</CommandItem>

				</ol>

			</Section>
		
	);
};
