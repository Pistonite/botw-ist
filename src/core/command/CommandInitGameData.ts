import { SimulationState } from "core/SimulationState";
import { joinItemStackString, processWrappers } from "./helper";
import { ItemStackCommandWrapper } from "./ItemStackCommandWrapper";
import { CommandImpl } from "./type";

export class CommandInitGameData extends CommandImpl {
	private stacks: ItemStackCommandWrapper[];
	constructor(stacks: ItemStackCommandWrapper[]){
		super();
		this.stacks = stacks;
	}

	public execute(state: SimulationState): void {
		state.setGameData(processWrappers(this.stacks));
	}
	public getDisplayString(): string {
		return joinItemStackString("Init GameData", this.stacks);
	}
}
