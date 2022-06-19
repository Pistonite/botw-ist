import { Command, CommandAddMaterial, CommandBreakSlots, CommandInitialize, CommandNothing, CommandReload, CommandRemoveMaterial, CommandRemoveUnstackableMaterial, CommandSave, CommandSortKey, CommandSortMaterial } from "./Command";
import { Item } from "./Item";

const Buffer = require("buffer/").Buffer; /* eslint-disable-line  @typescript-eslint/no-var-requires*/

export const serializeCommands = (commands: Command[]): string => {
	const sizeBuf: Buffer = Buffer.alloc(4);
	sizeBuf.writeInt32LE(commands.length);

	const commandBuffers = commands.map(c=>c.toBuffer());
	const allBufs = Buffer.concat([sizeBuf, ...commandBuffers]);
	
	return allBufs.toString("base64");
};

export const deserialzeCommands = (base64str: string): Command[] => {
	const buf: Buffer = Buffer.from(base64str, "base64");
	const size = buf.readInt32LE();
	let off = 4;
	const commands: Command[] = [];
	for(let i=0;i<size;i++){
		const op = buf.readUInt8(off);
		off++;
		let command: Command | undefined = undefined;
		switch(op){
			case CommandNothing.Op:
				command = new CommandNothing();
				break;
			case CommandInitialize.Op:
				command = new CommandInitialize([]);
				break;
			case CommandBreakSlots.Op:
				command = new CommandBreakSlots(0);
				break;
			case CommandSave.Op:
				command = new CommandSave();
				break;
			case CommandReload.Op:
				command = new CommandReload();
				break;
			case CommandSortKey.Op:
				command = new CommandSortKey();
				break;
			case CommandSortMaterial.Op:
				command = new CommandSortMaterial();
				break;
			case CommandRemoveMaterial.Op:
				command = new CommandRemoveMaterial("",0,Item.Slate,0);
				break;            
			case CommandRemoveUnstackableMaterial.Op:
				command = new CommandRemoveUnstackableMaterial("", Item.Slate, 0);
				break;            
			case CommandAddMaterial.Op:
				command = new CommandAddMaterial("",0,Item.Slate);
				break;
		}
		if(command){
			off += command.fromBuffer(buf.slice(off));
			commands.push(command);
		}else{
			console.error("invalid opcode: "+op);
		}
	}
	return commands;
};
