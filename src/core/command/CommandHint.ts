import { isAddVerb, isRemoveVerb } from "./helper";
import { CommandImpl } from "./type";

export class CommandHint extends CommandImpl {
	private keyword: string;
	constructor(keyword: string){
		super();
		this.keyword = keyword;
	}
	public getError(): string {
		switch(this.keyword){
			case "initialize":
				return "Initialize X item1[meta] Y item2[meta] Z item3[meta] ...";
			case "save":
				return "Save (As <file name>)";
			case "reload":
				return "Reload (<file name>)";
			case "break":
				return "Break X Slots";
			case "d&p":
			case "dnp":
			case "dap":
				return "D&P X item1[meta] Y item2[meta] Z item3[meta] ...";
			case "equip":
				return "Equip item (In Slot X)";
			case "unequip":
				return "Unequip item (In Slot X)";
			case "shoot":
				return "Shoot X Arrow";
			case "close":
				return "\"Close Game\"?";
			case "exit":
				return "Exit Game|TOTS|Eventide";
			case "sync":
				return "\"Sync GameData\"?";
			case "enter":
				return "Enter TOTS|Eventide";
			case "sort":
				return "Sorting is currently not supported";
			case "init":
				return "Init GameData X item1[meta] Y item2[meta] Z item3[meta] ...";
		}
		if(isAddVerb(this.keyword)){
			return "Add item, Add X item1[meta] Y item2[meta] Z item3[meta] ...";
		}
		if(isRemoveVerb(this.keyword)){
			return "Remove item (From Slot X), Remove X item (From Slot Y), Remove X item1[meta] Y item2[meta] Z item3[meta] ...";
		}
		return "Unknown Command";
	}
}
