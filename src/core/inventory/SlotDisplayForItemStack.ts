import clsx from "clsx";
import { CookEffect, getWeaponModifierName, ItemStack, ItemType, WeaponModifier } from "data/item";
import { SlotDisplay } from "./types";

export class SlotDisplayForItemStack implements SlotDisplay {
	private stack: ItemStack;
	isBrokenSlot = false;
	isIconAnimated = false;
	propertyString = "";
	propertyClassName = "";
	constructor(stack: ItemStack){
		this.stack = stack;
	}

	public init(
		isBrokenSlot: boolean,
		isIconAnimated: boolean
	): SlotDisplay {
		this.isBrokenSlot = isBrokenSlot;
		this.isIconAnimated = isIconAnimated;
		return this;
	}

	get image(): string {
		return this.isIconAnimated ? this.stack.item.animatedImage : this.stack.item.image;
	}

	get count(): number | undefined {
		if(this.isEquipment()){
			return undefined;
		}
		// not 1: always display
		// 1: display if stackable
		if(this.stack.item.stackable || this.stack.count !==  1){
			return this.stack.count;
		}
		return undefined;
	}

	get durability(): string | undefined {
		if (!this.isEquipment()){
			return undefined;
		}
		const durability = this.stack.durability;
		return Number.isInteger(durability) ? durability + "" : durability.toPrecision(4);
	}

	get isEquipped(): boolean {
		return this.stack.equipped;
	}

	get modifierImage(): string | undefined{
		const root = "assets/img/Modifiers/";
		if(!this.isEquipment()){
			// try food effect
			if(this.stack.foodEffect === CookEffect.None){
				return undefined;
			}
			return `${root}Cook${CookEffect[this.stack.foodEffect]}.png`;
		}

		const selectedModifier = selectModifier(this.stack);
		if(!selectedModifier){
			if(this.stack.item.bowMultishot > 0){
				// Default multishot
				return `${root}Multishot3.png`;
			}
			return undefined;
		}

		const isYellow = (this.stack.weaponModifier & WeaponModifier.Yellow) !== WeaponModifier.None;
		const yellowString = isYellow ? "Yellow" : "";

		if(selectedModifier === WeaponModifier.AttackUp){
			if (this.stack.item.type === ItemType.Bow){
				return `${root}BowAttackUp${yellowString}.png`;
			}else{
				return `${root}WeaponAttackUp${yellowString}.png`;
			}
		}
		if(selectedModifier === WeaponModifier.MultiShot){
			if(this.stack.weaponValue <= 3){
				return `${root}Multishot3.png`;
			}else if(this.stack.weaponValue === 5){
				return `${root}Multishot5.png`;
			}
			return `${root}MultishotX.png`;
		}

		return `${root}${getWeaponModifierName(selectedModifier)}${yellowString}.png`;

	}

	get modifierText(): string{
		if(!this.isEquipment()){
			if(this.stack.foodSellPrice){
				return `$${this.stack.foodSellPrice}`;
			}
			return "";
		}
		const selectedModifier = selectModifier(this.stack);

		if(selectedModifier){
			// currently we only display attack up and guard up numbers
			if((selectedModifier & WeaponModifier.AttackUp) !== WeaponModifier.None){
				return `+${this.stack.weaponValue}`;
			}
			if((selectedModifier & WeaponModifier.GuardUp) !== WeaponModifier.None){
				let value = this.stack.weaponValue;
				if((this.stack.weaponModifier & WeaponModifier.AttackUp) !== WeaponModifier.None){
					// if attack up is also present, guard up value is doubled
					value += this.stack.weaponValue;
				}
				return `+${value}`;
			}
		}
		return "";
	}

	get modifierClassName(): string {
		if(!this.modifierText){
			return "";
		}
		if(!this.isEquipment()){
			return "ItemModifierStringValue";
		}
		return clsx("ItemModifierStringValue", (this.stack.weaponModifier & WeaponModifier.Yellow) !== WeaponModifier.None && "ItemModifierStringValueYellow");
	}

	public getTooltip(translate: (s: string) => string): [string, string][] {
		const {
			item,
			durability,
			count,
			equipped,
			foodEffect,
			foodHpRecover,
			foodSellPrice,
			weaponModifier,
			weaponValue
		} = this.stack;
		const isFood = item.type===ItemType.Food;
		const isEquipment = this.isEquipment();
		const isBow = item.type ===ItemType.Bow;
		return [
			[translate(item.localizationKey), ""],
			[isEquipment
				? `Durability: ${durability}`
				: `Stack size: ${count}`, ""],

			isEquipment && equipped && [translate("state.Equipped"), ""],

			item.bowZoom && ["Zoom Bow", ""],
			item.bowRapidfire && [`Initial Rapidfire x${item.bowRapidfire}`, ""],
			item.bowMultishot && [`Initial Multishot x${item.bowMultishot}`, ""],

			isFood && [`Food Effect: ${CookEffect[foodEffect]}`, "ItemTooltipFoodEffect"],
			isFood && [`Recover ${foodHpRecover/4} Hearts`, "ItemTooltipFoodEffect"],
			isFood && [`Sell Price: ${foodSellPrice}`, "ItemTooltipFoodEffect"],

			isEquipment && (weaponModifier & WeaponModifier.AttackUp) !== 0
			&& [`Attack +${weaponValue}`, "ItemTooltipWeaponModifier"],
			isEquipment && (weaponModifier & WeaponModifier.DurabilityUp) !== 0
			&& ["Durablity Up", "ItemTooltipWeaponModifier"],
			isEquipment && (weaponModifier & WeaponModifier.CriticalHit) !== 0
			&& ["Critical Hit", item.type === ItemType.Weapon ? "ItemTooltipWeaponModifier" : "ItemTooltipWeaponModifierInactive"],
			isEquipment && (weaponModifier & WeaponModifier.LongThrow) !== 0
			&& [`Throw Speed ${getFixedPointReductionString(weaponValue)}`, item.type === ItemType.Weapon ? "ItemTooltipWeaponModifier" : "ItemTooltipWeaponModifierInactive"],
			isEquipment && (weaponModifier & WeaponModifier.MultiShot) !== 0
			&& [`Multishot x${Math.min(weaponValue, 10)} Max`, isBow? "ItemTooltipWeaponModifier" : "ItemTooltipWeaponModifierInactive"],
			isEquipment && (weaponModifier & WeaponModifier.Zoom) !== 0
			&& ["Zoom", isBow ? "ItemTooltipWeaponModifier" : "ItemTooltipWeaponModifierInactive"],
			isEquipment && (weaponModifier & WeaponModifier.QuickShot) !== 0
			&& [`Bow Draw Speed ${getFixedPointReductionString(weaponValue)}`, isBow ? "ItemTooltipWeaponModifier" : "ItemTooltipWeaponModifierInactive"],
			isEquipment && (weaponModifier & WeaponModifier.SurfMaster) !== 0
			&& [`Surf Friction =${weaponValue}`, item.type === ItemType.Shield ? "ItemTooltipWeaponModifier" : "ItemTooltipWeaponModifierInactive"],
			isEquipment && (weaponModifier & WeaponModifier.GuardUp) !== 0
			&& [`Shield Guard +${weaponValue}`, item.type === ItemType.Shield ? "ItemTooltipWeaponModifier" : "ItemTooltipWeaponModifierInactive"],
			isEquipment && (weaponModifier & WeaponModifier.Yellow) !== 0
			&& ["Yellow Modifier", "ItemTooltipFoodEffect"],

			[translate(`category.${ItemType[item.type]}`), "ItemTooltipType"]
		].filter(Boolean) as [string, string][];
	}

	private isEquipment(): boolean {
		return [ItemType.Weapon, ItemType.Bow, ItemType.Shield].includes(this.stack.item.type);
	}
}

const getFixedPointReductionString = (value: number): string => {
	if(value === 1000){
		return "not changed";
	}
	const percentage = (value-1000)/10;

	return `${percentage > 0 ? "+":""}${percentage}%`;
};

const selectModifier = (stack: ItemStack): number | undefined=> {
	// https://discord.com/channels/269611402854006785/269616041435332608/1041497732474482698
	const applicableModifiers: number[] = [
		WeaponModifier.AttackUp,
		WeaponModifier.DurabilityUp,
		WeaponModifier.GuardUp,
		WeaponModifier.CriticalHit,
		WeaponModifier.LongThrow,
		WeaponModifier.MultiShot,
		WeaponModifier.Zoom,
		WeaponModifier.QuickShot,
		WeaponModifier.SurfMaster
	];

	let selectedModifier: number = WeaponModifier.None;
	for(let i=0;i<applicableModifiers.length;i++){
		if((applicableModifiers[i] & stack.weaponModifier) !== WeaponModifier.None){
			selectedModifier = applicableModifiers[i];
			break;
		}
	}

	if(selectedModifier === WeaponModifier.None){
		return undefined;
	}
	return selectedModifier;
};
