import clsx from "clsx";
import { DoubleItemSlot } from "components/ItemSlot";
import { TitledList } from "components/TitledList";

import { SimulationState } from "core/SimulationState";
import Background from "assets/Background.png";
import InGameBackground from "assets/InGame.png";

import React, { useMemo } from "react";
import { ItemList } from "components/ItemList";
import { CrashScreen } from "components/CrashScreen";
import { Emphasized } from "components/Text";

type DisplayPaneProps = {
    command: string,
	commandError: string|undefined,
    simulationState: SimulationState,
    overlaySave: boolean,
    isIconAnimated: boolean,
    editCommand: (c: string)=>void
}

export const DisplayPane: React.FC<DisplayPaneProps> = ({
	command,
	commandError,
	editCommand,
	simulationState,
	overlaySave,
	isIconAnimated
})=>{
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
				<CrashScreen>
					The game has crashed (This is <Emphasized>not</Emphasized> a simulator bug)

				</CrashScreen>
			</div>;

	}else if(overlaySave){
		content =
			<div style={{
				borderTop: "1px solid black",
				boxSizing: "border-box",
				height: "100%",
				overflowY: "auto",
				background: `url(${InGameBackground})`,
				backgroundPosition: "center",
				backgroundSize: "auto 100%",
				color: "white",
			} }>

				<TitledList title={`Game Data / Visible Inventory (Count=${simulationState.inventoryMCount})`}>
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
				</TitledList>

			</div>
		;
	}else{
		content =
			<>

				<div style={{
					borderTop: "1px solid black",
					background: `url(${Background})`,
					color: "white",
					borderBottom: "1px solid black",
					boxSizing: "border-box",
					height: "50%",
					overflowY: "auto"
				} }>
					<TitledList title="Game Data">
						<ItemList slots={simulationState.displayableGameData.getDisplayedSlots(isIconAnimated)}/>
					</TitledList>

				</div>
				<div style={{
					borderTop: "1px solid black",
					background: `url(${InGameBackground})`,
					backgroundPosition: "center",
					backgroundSize: "100%",
					boxSizing: "border-box",
					height: "50%",
					overflowY: "auto",
					color: "white"
				} }>
					<TitledList title={`Visible Inventory (Count=${simulationState.inventoryMCount})`}>
						<ItemList slots={simulationState.displayablePouch.getDisplayedSlots(isIconAnimated)}/>
					</TitledList>

				</div>
			</>;

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
