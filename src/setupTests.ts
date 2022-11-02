/* eslint-disable @typescript-eslint/no-explicit-any */
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

// input should already be different
const diffObjects = (obj1: any, obj2: any, path: string, out: string[]) => {
	if(Array.isArray(obj1) && Array.isArray(obj2)){
		if(obj1.length !== obj2.length){
			out.push("Array length mismatch");
			out.push("path    : "+path);
			out.push("expected: "+JSON.stringify(obj1));
			out.push("actual  : "+JSON.stringify(obj2));
			out.push("");
		}else{
			for (let i=0;i<obj1.length;i++){
				const obj1Str = JSON.stringify(obj1[i]);
				const obj2Str = JSON.stringify(obj2[i]);
				if(obj1Str !== obj2Str){
					diffObjects(obj1[i], obj2[i], path+`[${i}]`, out);
				}
			}
		}
		return;
	}
	if(typeof obj1 === "object" && typeof obj2 === "object") {
		// find out if any sub-thing mismatch
		const mismatches = new Set<string>();
		for(const key in obj1) {
			if (key in obj2){
				const obj1Str = JSON.stringify(obj1[key]);
				const obj2Str = JSON.stringify(obj2[key]);
				if(obj1Str !== obj2Str){
					mismatches.add(key);
				}
			}else{
				// keys are not the same, output entire diff
				out.push("Object key set mismatch");
				out.push("path    : "+path);
				out.push("expected: "+JSON.stringify(obj1));
				out.push("actual  : "+JSON.stringify(obj2));
				out.push("");
				return;
			}
		}
		for(const key in obj2) {
			// every key in obj2 either is also in obj1 (already checked), or not
			if(mismatches.has(key)){
				continue;
			}
			if (!(key in obj1)){
				out.push("Object key set mismatch");
				out.push("path    : "+path);
				out.push("expected: "+JSON.stringify(obj1));
				out.push("actual  : "+JSON.stringify(obj2));
				out.push("");
				return;
			}
		}
		// output each mismatch
		mismatches.forEach(key=>{
			diffObjects(obj1[key], obj2[key], path+"."+key, out);
		});
		return;
	}
	out.push("Value mismatch");
	out.push("path    : "+path);
	out.push("expected: "+JSON.stringify(obj1));
	out.push("actual  : "+JSON.stringify(obj2));
	out.push("");
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
		if(expectedString !== resultString){
			const out: string[] = [];
			diffObjects(expectedState, result, "", out);
			fs.writeFileSync(
				"src/__tests__/debug.mismatches.log",
				out.join("\n"),
				"utf-8");
		}

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
