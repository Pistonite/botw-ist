import { Command } from "./command";
import { parseCommand } from "./parsev2";
import { ItemSearchFunction } from "./type";

// When debugging, it's useful to turn memoized parser off to make sure commands are reparsed when the parser changes
const EnableMemoizedParser = true;
// When command data updates, usually most of the commands are the same
// So we could memoize the result from last iteration, and use that in the next iteration
export class MemoizedParser {
    private lastSearchItem: ItemSearchFunction | undefined = undefined;
    private memo: { [command: string]: Command } = {};
    public parseCommands(
        commandData: string[],
        searchItem: ItemSearchFunction,
    ): Command[] {
        //const t0 = performance.now();
        if (this.lastSearchItem !== searchItem) {
            this.memo = {};
        }
        const newMemo: { [command: string]: Command } = {};
        const commands = commandData.map((commandString) => {
            let result = this.memo[commandString];
            if (!result) {
                result = parseCommand(commandString, searchItem);
            }
            newMemo[commandString] = result;
            return result;
        });

        // const t1 = performance.now();
        // console.log(`Command parsing took ${t1 - t0} ms for ${commandData.length} commands.`);
        if (EnableMemoizedParser) {
            this.memo = newMemo;
        }
        return commands;
    }
}
