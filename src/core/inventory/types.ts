export interface DisplayableInventory {
    getDisplayedSlots: (isIconAnimated: boolean)=>SlotDisplay[]
}

export interface SlotDisplay {
	// image to display
    readonly image: string,
	// count of stack, displayed as "xCCC" at the bottom left
    readonly count?: number,
	// durability of stack, displayed as floating window at the bottom left
	readonly durability?: string,
	// if the stack is equipped
    readonly isEquipped: boolean,
	// if the slot is broken (i.e in the count offset region)
    readonly isBrokenSlot: boolean,
	// Override the property string, displayed at the bottom right
	readonly propertyString?: string,
    readonly propertyClassName?: string,
	// Modifier image displayed at the top right
	readonly modifierImage?: string,
	// Modifier string, displayed at the top right next to modifier image
	readonly modifierText?: string,
	readonly modifierClassName?: string,
	// tooltip
	getTooltip: (translate: (s:string)=>string)=>[string, string][],
}

export type GameFlags = {
	weaponSlots: number,
	bowSlots: number,
	shieldSlots: number
}
