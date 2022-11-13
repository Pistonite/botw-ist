import produce from "immer";
import { useCallback } from "react";
import { CommandItem, ColoredCodeBlocks, Section } from "ui/components";
import { ContextMenuState } from "ui/types";
import { CmdErr, Command } from "core/command";
import { useRuntime } from "core/runtime";
import { GetSetPair } from "data/util";

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

		<Section contentId="SimStepsPanel" titleText="Steps" style={{
			height: "100%"
		}}>
			<ol >
				{
					commandData.map((c,i)=>
						<CommandItem
							htmlId={displayIndex===i ? "SimStepSelectedItem":undefined}
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
							useListItem={!commands[i].shouldSkipWithKeyboard}
							isInvalid={commands[i].cmdErr !== CmdErr.None}
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
