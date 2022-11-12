import { Item } from "data/item";
// import { SimulationState } from "../SimulationState";
// import { processWrappers } from "./helper";
// import { ItemStackArg } from "./ItemStackArg";


// export class CommandDaP extends CommandImpl  {
// 	private stacks: ItemStackArg[];

// 	constructor(stacks: ItemStackArg[]){
// 		super();
// 		this.stacks = stacks;
// 	}
// 	public execute(state: SimulationState): void {
// 		processWrappers(this.stacks).forEach(stack=>{
// 			state.remove(stack, 0);
// 			state.obtain(stack);
// 		});
// 	}
// }





// export class CommandShootArrow extends CommandImpl  {
// 	private count: number;
// 	constructor(count: number){
// 		super();
// 		this.count = count;
// 	}

// 	public execute(state: SimulationState): void {
// 		state.shootArrow(this.count);
// 	}
// 	public getDisplayString(): string {
// 		return `Shoot ${this.count} Arrow`;
// 	}
// }

// export class CommandCloseGame extends CommandImpl  {
// 	public execute(state: SimulationState): void {
// 		state.closeGame();
// 	}
// 	public getDisplayString(): string {
// 		return "Close Game";
// 	}
// }



// export class CommandEventide extends CommandImpl  {
// 	private enter: boolean;
// 	constructor(enter: boolean){
// 		super();
// 		this.enter = enter;
// 	}

// 	public execute(state: SimulationState): void {
// 		state.setEventide(this.enter);
// 	}
// 	public getDisplayString(): string {
// 		return `${this.enter? "Enter":"Exit"} Eventide`;
// 	}
// }
