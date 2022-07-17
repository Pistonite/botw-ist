import { ItemIdMap, ItemStack } from "./type";

// Legacy item names before all items were added

const LegacyMap = {
	"Slate": "SheikahSlate",
	"Glider": "Paraglider",
	"SpiritOrb": "SpiritOrb",
	"Lotus": "FleetLotusSeeds",
	"SilentPrincess": "SilentPrincess",
	"Honey": "CourserBeeHoney",
	"Acorn": "Acorn",
	"FaroshScale": "FaroshsScale",
	"FaroshClaw": "FaroshsClaw",
	"FaroshHorn": "ShardOfFaroshsHorn",
	"HeartyBass": "HeartyBass",
	"Beetle": "EnergeticRhinoBeetle",
	"Opal": "Opal",
	"Diamond": "Diamond",
	"Tail": "LizalfosTail",
	"Spring": "AncientSpring",
	"Shaft": "AncientShaft",
	"Core": "AncientCore",
	"Wood": "Wood",
	"Rushroom": "Rushroom",
	"Screw": "AncientScrew",
	"HyruleBass": "HyruleBass",
	"LizalfosHorn": "LizalfosHorn",
	"LizalfosTalon": "LizalfosTalon",
	"Weapon": "Weapon",
	"Bow": "Bow",
	"NormalArrow": "NormalArrow",
	"FireArrow": "FireArrow",
	"IceArrow": "IceArrow",
	"ShockArrow": "ShockArrow",
	"BombArrow": "BombArrow",
	"AncientArrow": "AncientArrow",
	"Shield": "Shield",
	"Apple": "Apple",
	"HylianShroom": "HylianShroom",
	"SpicyPepper": "SpicyPepper",
	"EnduraShroom": "EnduraShroom",
	"HeartyRadish": "HeartyRadish",
	"BigHeartyRadish": "BigHeartyRadish",
	"Fairy": "Fairy",
	"MasterSword": "MasterSword",
	"ZoraArmor": "ZoraArmor"
};

export const searchLegacyItemNames = (name: string, idMap: ItemIdMap): ItemStack | undefined => {
	if (name in LegacyMap){
		return idMap[LegacyMap[name as keyof typeof LegacyMap]].createDefaultStack();
	}
	if(name === "SpeedFood"){
		// TODO: effect metadata
		return idMap.SteamedFruit.createDefaultStack();
	}
	if(name === "EnduraFood"){
		// TODO: effect metadata
		return idMap.MushroomSkewer.createDefaultStack();
	}
	return undefined;
};
