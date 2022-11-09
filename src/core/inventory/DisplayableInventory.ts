import { CookEffect, ItemStack, ItemType, WeaponModifier } from "data/item";
import { SlotDisplay } from "./types";

export class SlotDisplayForItemStack implements SlotDisplay {
	private stack: ItemStack;
	isBrokenSlot: boolean = false;
	isIconAnimated: boolean = false;
	propertyString: string = "";
	propertyClassName: string = "";
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
		if(!this.isEquipment()){
			// try food effect
			if(this.stack.foodEffect === CookEffect.None){
				return undefined;
			}
			return `assets/img/Modifiers/${CookEffect[this.stack.foodEffect]}.png`;
		}
		const applicableModifiers: number[] = [
			WeaponModifier.AttackUp,
			WeaponModifier.DurabilityUp
		];
		// Add the right modifiers for the type
		switch(this.stack.item.type){
			case ItemType.Weapon:
				applicableModifiers.push(
					WeaponModifier.CriticalHit,
					WeaponModifier.LongThrow
				);
				break;
			case ItemType.Bow:
				applicableModifiers.push(
					WeaponModifier.MultiShot,
					WeaponModifier.Zoom,
					WeaponModifier.QuickShot
				);
				break;
			case ItemType.Shield:
				applicableModifiers.push(
					WeaponModifier.SurfMaster,
					WeaponModifier.GuardUp
				);
		}
		let selectedModifier: number = WeaponModifier.None;
		for(let i=0;i<applicableModifiers.length;i++){
			if((applicableModifiers[i] & this.stack.weaponModifier) !== WeaponModifier.None){
				selectedModifier = applicableModifiers[i];
			}
		}
		// Add default bow modifier (only disable multishot)
		if(selectedModifier === WeaponModifier.None && this.stack.item.type === ItemType.Bow){
			if (this.stack.item.bowMultishot > 0 || this.stack.item.bowRapidfire > 0){
				selectedModifier = WeaponModifier.MultiShot;
			}
		}
		if(selectedModifier === WeaponModifier.None){
			return undefined;
		}
		// TODO: return the image
		
	}

	get modifierText(): string | undefined{
		return undefined; //TODO
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
			&& [`Durablity Up`, "ItemTooltipWeaponModifier"],
			isEquipment && (weaponModifier & WeaponModifier.CriticalHit) !== 0 
			&& [`Critical Hit`, item.type === ItemType.Weapon ? "ItemTooltipWeaponModifier" : "ItemTooltipWeaponModifierInactive"],
			isEquipment && (weaponModifier & WeaponModifier.LongThrow) !== 0 
			&& [`Throw Speed ${getFixedPointReductionString(weaponValue)}`, item.type === ItemType.Weapon ? "ItemTooltipWeaponModifier" : "ItemTooltipWeaponModifierInactive"],
			isEquipment && (weaponModifier & WeaponModifier.MultiShot) !== 0 
			&& [`Multishot x${Math.min(weaponValue, 10)} Max`, isBow? "ItemTooltipWeaponModifier" : "ItemTooltipWeaponModifierInactive"],
			isEquipment && (weaponModifier & WeaponModifier.Zoom) !== 0 
			&& [`Zoom`, isBow ? "ItemTooltipWeaponModifier" : "ItemTooltipWeaponModifierInactive"],
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
}
