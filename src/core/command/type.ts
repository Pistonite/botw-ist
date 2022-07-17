import { SimulationState } from "core/SimulationState";

export interface Command {
	getError(): string|undefined,
	execute(state: SimulationState): void,
	//getDefaultString(): string,
}

export class CommandImpl implements Command{
	getError(): string|undefined {
		return undefined;
	}
	execute(_state: SimulationState): void {
		// nothing
	}
}
