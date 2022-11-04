import clsx from "clsx";
import { DoubleItemSlot } from "components/ItemSlot";
import { Section } from "ui/components";

import { SimulationState } from "core/SimulationState";
import Background from "assets/Background.png";
import InGameBackground from "assets/InGame.png";

import React, { useMemo } from "react";
import { ItemList } from "components/ItemList";
import { CrashScreen } from "ui/surfaces/CrashScreen";
import { useRuntime } from "data/runtime";

type DisplayPaneProps = {
    command: string,
	commandError: string|undefined,
	showGameData: boolean,
    simulationState: SimulationState,
    editCommand: (c: string)=>void
}

export const DisplayPane: React.FC<DisplayPaneProps> = ({
	command,
	commandError,
	showGameData,
	editCommand,
	simulationState,
})=>{
	const { setting } = useRuntime();
	const isIconAnimated = setting("animatedIcon");
	const isGameDataInterlaced = setting("interlaceGameData");
	//const searchItem = useSearchItem();
	// const
	const error = useMemo(()=>{
		if(command === ""){
			return "";
		}else if (command.startsWith("#")){
			return "";
		}
		return commandError;
	}, [command, commandError]);

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
				/>

			</div>;

	}else if(isGameDataInterlaced && showGameData){
		content =
			<Section titleText={`Game Data / Visible Inventory (Count=${simulationState.inventoryMCount})`} style={{
				borderTop: "1px solid black",
				boxSizing: "border-box",
				height: "100%",
				overflowY: "auto",
				background: `url(${InGameBackground})`,
				backgroundPosition: "center",
				backgroundSize: "auto 100%",
				color: "white",
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
			<div style={{
				display: "flex",
				flexDirection: "column",
				minHeight: "100%"
			}}>
				{
					showGameData && <Section titleText="Game Data" style={{
						borderTop: "1px solid black",
						background: `url(${Background})`,
						color: "white",
						borderBottom: "1px solid black",
						boxSizing: "border-box",
						flex: 1,
						overflowY: "auto"
					} }>
						<ItemList slots={simulationState.displayableGameData.getDisplayedSlots(isIconAnimated)}/>
					</Section>
				}

				<Section titleText={`Visible Inventory (Count=${simulationState.inventoryMCount})`} style={{
					borderTop: "1px solid black",
					backgroundImage: `url(${InGameBackground})`,
					backgroundPosition: "center",
					backgroundRepeat: "no-repeat",
					backgroundSize: "cover",
					boxSizing: "border-box",
					flex: 1,
					overflowY: "auto",
					color: "white"
				} }>
					<ItemList slots={simulationState.displayablePouch.getDisplayedSlots(isIconAnimated)}/>

				</Section>
			</div>;

	}

	return <div id="DisplayPane" style={{
		height: "100%",
	} }>
		<div style={{
			boxSizing: "border-box",
			height: "40px",
			position: "relative",
		} }>
			<input id="CommandInputField" className={clsx("Calamity", "CommandInput", error && "InputError")} style={{
				background: `url(${Background})`,
				width: "100%",
				height: "40px",
				paddingLeft: 10,
				margin: 0,
				boxSizing: "border-box",
				fontSize: "16pt",
				outline: "none",
			}}value={command}
			placeholder="Type command here..."
			spellCheck={false}
			onChange={(e)=>{
				const cmdString = e.target.value;
				editCommand(cmdString);
			}}></input>
			{
				error && <div style={{
					boxSizing: "border-box",
					border: "2px solid #ee0000",
					borderTop: "none",
					backgroundColor: "#333333bb",
					padding: 3,
					color: "#eeeeee",
				}}>{error}</div>
			}

		</div>
		<div style={{
			height: "calc( 100% - 40px )"
		}}>
			{content}
		</div>

	</div>;
};
