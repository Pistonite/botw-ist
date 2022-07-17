// jest-dom adds custom jest matchers for asserting on DOM nodes.
// allows you to do things like:
// expect(element).toHaveTextContent(/react/i)
// learn more: https://github.com/testing-library/jest-dom
import "@testing-library/jest-dom";
import { Command, parseCommand } from "core/command";
import { createSimulationState, SimulationState } from "core/SimulationState";
import { ItemStack, loadItemData, searchItemMemoized } from "data/item";
import fs from "fs";
import YAML from "yaml";
const ItemDataString = fs.readFileSync("src/data/item/all.items.yaml", "utf-8");
const ItemData = YAML.parse(ItemDataString);
expect.extend({
	toEqualItemStacks: (received, expected, equals?)=>{
		if (!Array.isArray(received)) {
			return {
				message: () =>
					`expected ${received} to be an ItemStack array`,
				pass: false,
			};
		}
		if (received.length !== expected.length){
			return {
				message: () =>
					`expected ${received} to equal ${expected}, but their length do not equal (${received.length} !== ${expected.length})`,
				pass: false,
			};
		}
		for(let i = 0; i<received.length;i++){
			const receivedStack = received[i];
			const expectStack = expected[i];
			if(equals){
				if(!equals(expectStack, receivedStack)){
					return {
						message: () =>
							`Differ at index ${i}, expected ${JSON.stringify(expectStack, null, 2)}, got ${JSON.stringify(receivedStack, null, 2)}`,
						pass: false,
					};
				}
			}else if(!expectStack.equals || !expectStack.equals(receivedStack)){
				return {
					message: () =>
						`Differ at index ${i}, expected ${JSON.stringify(expectStack, null, 2)}, got ${JSON.stringify(receivedStack, null, 2)}`,
					pass: false,
				};
			}
		}
		return {
			message: () =>
				"ItemStack arrays are the same",
			pass: true,
		};
	}
});

const [IdMap, SearchMap] = loadItemData(ItemData);
const SearchResultMemo = {};
const searchFunc = (word: string): ItemStack | undefined => {
	return searchItemMemoized(word, IdMap, SearchMap, SearchResultMemo);
};
const getCommandsFromString = (str: string): Command[] => {
	const lines = str.split("\n");
	return lines.map(l=>parseCommand(l, searchFunc));
};
const runE2ESimulation = (str: string): SimulationState => {
	const state = createSimulationState();
	const commands = getCommandsFromString(str);
	commands.forEach(c=>c.execute(state));
	return state;
};

const runE2ETest = (name: string, debug: boolean): [string, string]=>{
	const script = fs.readFileSync(`src/__tests__/${name}.in.txt`, "utf-8");
	const result = runE2ESimulation(script);
	const resultString = JSON.stringify(result, null, 2);
	const expected = fs.readFileSync(`src/__tests__/${name}.out.txt`, "utf-8");

	const expectedState = runE2ESimulation(expected);
	const expectedString = JSON.stringify(expectedState, null, 2);
	if(debug){
		fs.writeFileSync("src/__tests__/debug.actual.log", resultString, "utf-8");
		fs.writeFileSync("src/__tests__/debug.expected.log", expectedString, "utf-8");
	}
	return [resultString, expectedString];
};

expect.extend({
	toPassE2ESimulation: (receivedName: string, expectDebug?: boolean) => {
		const [resultString, expectedString] = runE2ETest(receivedName, !!expectDebug);
		if (resultString !== expectedString) {
			return {
				message: () =>
					"E2E simulation failed. Pass true to toPassE2ESimulation() to emit debug logs",
				pass: false,
			};
		}
		return {
			message: ()=>"E2E simulation passed.",
			pass: true
		};
	}
});
