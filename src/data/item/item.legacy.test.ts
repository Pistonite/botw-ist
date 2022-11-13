import { CookEffect, ItemStack } from "./type";

export {};
describe("data/item.legacy", ()=>{
	it("should match exactly for legacy items", ()=>{
		expect("slate").toMatchItemSearch("SheikahSlate");
		expect("glider").toMatchItemSearch("Paraglider");
		expect("spiritorb").toMatchItemSearch("SpiritOrb");
		expect("lotus").toMatchItemSearch("FleetLotusSeeds");
		expect("silentprincess").toMatchItemSearch("SilentPrincess");
		expect("honey").toMatchItemSearch("CourserBeeHoney");
		expect("acorn").toMatchItemSearch("Acorn");
		expect("faroshscale").toMatchItemSearch("FaroshsScale");
		expect("faroshclaw").toMatchItemSearch("FaroshsClaw");
		expect("faroshhorn").toMatchItemSearch("ShardOfFaroshsHorn");
		expect("heartybass").toMatchItemSearch("HeartyBass");
		expect("beetle").toMatchItemSearch("EnergeticRhinoBeetle");
		expect("opal").toMatchItemSearch("Opal");
		expect("diamond").toMatchItemSearch("Diamond");
		expect("tail").toMatchItemSearch("LizalfosTail");
		expect("spring").toMatchItemSearch("AncientSpring");
		expect("shaft").toMatchItemSearch("AncientShaft");
		expect("core").toMatchItemSearch("AncientCore");
		expect("wood").toMatchItemSearch("Wood");
		expect("rushroom").toMatchItemSearch("Rushroom");
		expect("screw").toMatchItemSearch("AncientScrew");
		expect("hyrulebass").toMatchItemSearch("HyruleBass");
		expect("lizalfoshorn").toMatchItemSearch("LizalfosHorn");
		expect("lizalfostalon").toMatchItemSearch("LizalfosTalon");
		expect("weapon").toMatchItemSearch("Weapon");
		expect("bow").toMatchItemSearch("Bow");
		expect("normalarrow").toMatchItemSearch("NormalArrow");
		expect("firearrow").toMatchItemSearch("FireArrow");
		expect("icearrow").toMatchItemSearch("IceArrow");
		expect("shockarrow").toMatchItemSearch("ShockArrow");
		expect("bombarrow").toMatchItemSearch("BombArrow");
		expect("ancientarrow").toMatchItemSearch("AncientArrow");
		expect("shield").toMatchItemSearch("Shield");
		expect("apple").toMatchItemSearch("Apple");
		expect("hylianshroom").toMatchItemSearch("HylianShroom");
		expect("spicypepper").toMatchItemSearch("SpicyPepper");
		expect("endurashroom").toMatchItemSearch("EnduraShroom");
		expect("heartyradish").toMatchItemSearch("HeartyRadish");
		expect("bigheartyradish").toMatchItemSearch("BigHeartyRadish");
		expect("fairy").toMatchItemSearch("Fairy");
		expect("masterSword").toMatchItemSearch("MasterSword");
		expect("zoraarmor").toMatchItemSearch("ZoraArmor");

		// Legacy food. Need to add metadata
		expect("speedfood").toMatchItemSearch((stack: ItemStack)=>{
			expect(stack.item.id).toEqual("SteamedFruit");
			return stack.modifyMeta({cookEffect: CookEffect.Hasty});
		});
		expect("endurafood").toMatchItemSearch((stack: ItemStack)=>{
			expect(stack.item.id).toEqual("MushroomSkewer");
			return stack.modifyMeta({cookEffect: CookEffect.Enduring});
		});

	});
});
