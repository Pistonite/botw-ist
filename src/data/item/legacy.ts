import { CookEffect, ItemIdMap, ItemStack } from "./type";


export const searchLegacyItemNames = (name: string, idMap: ItemIdMap): ItemStack | undefined => {
	if(name === "speedfood"){
		return idMap.SteamedFruit.defaultStack.modifyMeta({cookEffect: CookEffect.Hasty});
	}
	if(name === "endurafood"){
		return idMap.MushroomSkewer.defaultStack.modifyMeta({cookEffect: CookEffect.Enduring});
	}
	return undefined;
};
