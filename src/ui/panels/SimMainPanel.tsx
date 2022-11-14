import InfoOutlined from "@mui/icons-material/InfoOutlined";
import clsx from "clsx";
import React, { useState } from "react";
import { Section, DoubleItemSlot, ItemList } from "ui/components";
import { CrashScreen, CommandTextArea, Tooltip } from "ui/surfaces";
import { SimulationState } from "core/SimulationState";
import { CmdErr, Command } from "core/command";
import { useRuntime } from "core/runtime";

type SimMainPanelProps = {
    commandText: string,
	command: Command,
	showGameData: boolean,
    simulationState: SimulationState,
    editCommand: (c: string)=>void
}

export const SimMainPanel: React.FC<SimMainPanelProps> = ({
	commandText,
	command,
	showGameData,
	editCommand,
	simulationState,
})=>{

	const { setting } = useRuntime();
	const isIconAnimated = setting("animatedIcon");
	const isGameDataInterlaced = setting("interlaceGameData");
	const showHint = setting("showCommandHint");

	const inventoryInfo = 
		<span style={{marginLeft: 8}}>
			<Tooltip title={simulationState.getInventoryInfo()}>
				<InfoOutlined />
			</Tooltip>
		</span>;

	let content: JSX.Element;
	if(simulationState.isCrashed()){
		content =
			<div style={{
				position: "relative",
				height: "100%"
			}}>

				<CrashScreen
					primaryText="The game has crashed"
					secondaryText="(This is NOT a simulator bug)"
					hideReloadButton
				/>

			</div>;

	}else if(isGameDataInterlaced && showGameData){
		content =
			<Section className="HatenoBackground" titleText={
				<div style={{display: "flex"}}>
					<span>Visible Inventory (Count={simulationState.inventoryMCount})</span>
					{inventoryInfo}
				</div>
			} style={{
				height: "100%",
				overflowY: "auto",
			} }>

				{
					(()=>{
						const doubleSlots: JSX.Element[] = [];
						const gameDataSlots = simulationState.displayableGameData.getDisplayedSlots(isIconAnimated);
						const inventorySlots = simulationState.displayablePouch.getDisplayedSlots(isIconAnimated);
						for(let i=0;i<gameDataSlots.length && i<inventorySlots.length;i++){
							doubleSlots.push(<DoubleItemSlot key={i}
								first={{slot: gameDataSlots[i]}}
								second={{slot: inventorySlots[i]}}
							/>);
						}
						if(gameDataSlots.length>inventorySlots.length){
							for(let i=inventorySlots.length;i<gameDataSlots.length;i++){
								doubleSlots.push(<DoubleItemSlot key={i+inventorySlots.length}
									first={{slot: gameDataSlots[i]}}
								/>);
							}
						}else if(inventorySlots.length > gameDataSlots.length){
							for(let i=gameDataSlots.length;i<inventorySlots.length;i++){
								doubleSlots.push(<DoubleItemSlot key={i + gameDataSlots.length}
									second={{slot: inventorySlots[i]}}
								/>);
							}
						}
						return doubleSlots;
					})()
				}

			</Section>
		;
	}else{
		content =
			<>
				{
					showGameData && <Section titleText="Game Data" className="SheikaBackground" style={{
						boxSizing: "border-box",
						height: "50%",
						overflowY: "auto",
						borderBottom: "1px solid black"
					} }>
						<ItemList slots={simulationState.displayableGameData.getDisplayedSlots(isIconAnimated)}/>
					</Section>
				}

				<Section className="HatenoBackground" titleText={
					<div style={{display: "flex"}}>
						<span>Visible Inventory (Count={simulationState.inventoryMCount})</span>
						{inventoryInfo}
					</div>
				} style={{
					boxSizing: "border-box",
					height: showGameData ? "50%" : "100%",
					overflowY: "auto",
				} }>
					<ItemList slots={simulationState.displayablePouch.getDisplayedSlots(isIconAnimated)}/>

				</Section>
			</>;

	}
	const [textAreaHeight, setTextAreaHeight] = useState<number>(40);

	return (
		<div id="DisplayPane" style={{
			height: "100%",
			width: "100%",
			overflowWrap: "break-word"
		} }>
			<div className="MainInput" style={{
				boxSizing: "content-box",
				height: textAreaHeight,
				paddingBottom: 0,
				paddingTop: 0,
				paddingLeft: 0,
				paddingRight: 0
			} }>
				<CommandTextArea
					textAreaId="SimulationCommandTextField"
					large
					onAutoResize={setTextAreaHeight}
					blocks={[command.codeBlocks]}
					value={[commandText]}
					setValue={(v)=>editCommand(v.join(" "))}
					removeLines
				/>
			</div>
			<div style={{
				position: "relative", // for command tooltip to anchor
				height: `calc( 100% - ${textAreaHeight}px)`
			}}>

				{content}
				{
					command.err.length > 0 && (showHint || command.cmdErr !== CmdErr.Guess) &&
					<div className={clsx(
						"TooltipWindow",
						command.cmdErr === CmdErr.Parse && "TooltipWarn",
						command.cmdErr === CmdErr.Execute && "TooltipError"
					)} style={{
						position: "absolute",
						top: 0,
						right: 0,
						left: 0
					}}>
						{
							command.err.map((error,i)=><p className={clsx("TooltipLine", i==0 && "TooltipHeader")} key={i}>{error}</p>)
						}
					</div>
				}
			</div>
		</div>
	);
};
