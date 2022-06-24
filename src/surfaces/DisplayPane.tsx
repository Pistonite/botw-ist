import clsx from "clsx";
import { DoubleItemSlot } from "components/ItemSlot";
import { TitledList } from "components/TitledList";
import { Command } from "core/Command";
import { parseCommand } from "core/Parser";
import { SimulationState } from "core/SimulationState";
import Background from "assets/Background.png";
import InGameBackground from "assets/InGame.png";

import React, { useEffect, useState } from "react";
import { ItemList } from "components/ItemList";

type DisplayPaneProps = {
    command: string,
    displayIndex: number,
    simulationState: SimulationState,
	overlaySave: boolean,
    editCommand: (c: Command)=>void
}

export const DisplayPane: React.FC<DisplayPaneProps> = ({command,editCommand,displayIndex,simulationState,  overlaySave})=>{
	const [commandString, setCommandString] = useState<string>("");
	const [hasError, setHasError] = useState<boolean>(false);
	//const listProps = stacksToItemListProps(slots, numBroken, false);
	//const listSaveProps = stacksToItemListProps(savedSlots, 0, true);
	useEffect(()=>{
		if(commandString!==command){
			setCommandString(command);
			setHasError(false);
		}
      
	}, [command, displayIndex]);

	return <div id="DisplayPane" style={{
		height: "100%",
		// width: "calc( 100% - 300px - 5px )",
		// float: "right",
		// border: "1px solid black",
		// boxSizing: "content-box"
	} }>
		<div style={{
			boxSizing: "border-box",
			height: "40px"
		} }>
			<input id="CommandInputField" className={clsx("Calamity", "CommandInput", hasError && "InputError")} style={{
				background: `url(${Background})`,
				width: "100%",
				height: "40px",
				paddingLeft: 10,
				margin: 0,
				boxSizing: "border-box",
				fontSize: "16pt",
				outline: "none",
			}}value={commandString}
			placeholder="Type command here..."
			spellCheck={false}
			onChange={(e)=>{
				const cmdString = e.target.value;
				setCommandString(cmdString);
				const parsedCommand = parseCommand(cmdString);
				if(parsedCommand){
					editCommand(parsedCommand);
					setHasError(false);
				}else{
					setHasError(true);
				}
			}}></input>

		</div>
		<div style={{
			height: "calc( 100% - 40px )"
		}}>
			{overlaySave ? 
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
								const gameDataSlots = simulationState.displayableGameData.getDisplayedSlots();
								const inventorySlots = simulationState.displayablePouch.getDisplayedSlots();
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
		
				:<>
		
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
							<ItemList slots={simulationState.displayableGameData.getDisplayedSlots()}/>
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
							<ItemList slots={simulationState.displayablePouch.getDisplayedSlots()}/>
						</TitledList>
					
					</div>
				</>}
		</div>

	</div>;
};
