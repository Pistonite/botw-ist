#![allow(non_camel_case_types)] // non camel case to match closer to actor names
pub enum ItemCategoryRaw {
    Weapon_Sword, // Weapon_Sword_XXX
    Weapon_Lsword, // Weapon_Lsword_XXX
    Weapon_Spear, // Weapon_Spear_XXX
    Weapon_Bow, // Weapon_Bow_XXX
    Weapon_Shield, // Weapon_Shield_XXX
    Armor_Head, // Armor_XXX_Head
    Armor_Upper, // Armor_XXX_Upper
    Armor_Lower, // Armor_XXX_Lower
    Item, // Item_
    Item_Cook, // Item_Cook
    Key
}

pub struct ItemDataRaw {
    pub category: ItemCategoryRaw,
    pub actor_name: &'static str,
    pub item_name: &'static str,
    pub search_term: &'static str,
    pub priority: u32,
    pub weapon_power: u32,
    pub durability: u32,
    pub armor_upgrade: u32,
    pub stackable: bool,
    pub bow_zoom: bool,
    pub bow_multishot: bool,
    pub bow_rapidfire: bool,
}

impl ItemDataRaw {
    pub const fn new_const(category: ItemCategoryRaw, actor_name: &'static str, item_name: &'static str) -> Self {
        ItemDataRaw {
            category,
            actor_name,
            item_name,
            search_term: "",
            priority: 0,
            weapon_power: 0,
            durability: 0,
            armor_upgrade: 0,
            stackable: false,
            bow_zoom: false,
            bow_multishot: false,
            bow_rapidfire: false,
        }
    }

}

macro_rules! actor_name {
    (Weapon_Sword $id:literal) => {
        concat!("Weapon_Sword_", $id)
    };
    (Weapon_Lsword $id:literal) => {
        concat!("Weapon_Lsword_", $id)
    };
    (Weapon_Spear $id:literal) => {
        concat!("Weapon_Spear_", $id)
    };
    (Weapon_Bow $id:literal) => {
        concat!("Weapon_Bow_", $id)
    };
    (Weapon_Shield $id:literal) => {
        concat!("Weapon_Shield_", $id)
    };
    (Armor_Head $id:literal) => {
        concat!("Armor_", $id, "_Head")
    };
    (Armor_Upper $id:literal) => {
        concat!("Armor_", $id, "_Upper")
    };
    (Armor_Lower $id:literal) => {
        concat!("Armor_", $id, "_Lower")
    };
    (Item $id:literal) => {
        concat!("Item_", $id)
    };
    (Item_Cook $id:literal) => {
        concat!("Item_Cook_", $id)
    };
    (Key $id:literal) => {
        $id
    };
}

macro_rules! item_data_raw {
    ($type:ident $id:literal: $item_name:literal 
    $( {$($field:ident : $value:literal),*} )?
) => {
        {
            #[allow(unused_mut)]
            let mut item = ItemDataRaw::new_const(
                ItemCategoryRaw::$type,
                actor_name!($type $id),
                $item_name
            );
            $(
                $(item.$field = $value;)*
            )?
            item
        }
    }
}

macro_rules! item_map {
    (
    $(
            $type:ident $id:literal: $item_name:literal
    $( {$($field:ident : $value:literal),*} )?
                
),* 
    ) => {
        phf::phf_map! {
            $(
                $item_name => item_data_raw!($type $id: $item_name $( {$($field : $value),*} )?),
            )*
        }
    }
}

pub const ITEMS: phf::Map<&'static str, ItemDataRaw> = item_map! {
    Weapon_Sword "070": "MasterSword" { durability: 40, weapon_power: 30 }
"Weapon_Sword_001": "Traveler's Sword",
//   - TravelersSword:travelersword:
//       durability: 20
	"Weapon_Sword_002": "Soldier's Broadsword",
//   - SoldiersBroadsword:soldiersword:
//       durability: 23
	"Weapon_Sword_003": "Knight's Broadsword",
//   - KnightsBroadsword:knightsword:
//       durability: 27
	"Weapon_Sword_004": "Boko Club",
//   - BokoClub:
//       durability: 12
	"Weapon_Sword_005": "Spiked Boko Club",
//   - SpikedBokoClub:
//       durability: 14
	"Weapon_Sword_006": "Dragonbone Boko Club",
//   - DragonboneBokoClub:
//       durability: 18
	"Weapon_Sword_007": "Lizal Boomerang",
//   - LizalBoomerang:
//       durability: 17
	"Weapon_Sword_008": "Lizal Forked Boomerang",
//   - LizalForkedBoomerang:
//       durability: 23
	"Weapon_Sword_009": "Lizal Tri-Boomerang",
//   - LizalTriBoomerang:
//       durability: 27
//       priority: 1
	"Weapon_Sword_013": "Guardian Sword",
//   - GuardianSword:
//       durability: 17
	"Weapon_Sword_014": "Guardian Sword+",
//   - GuardianSwordPlus:
//       durability: 26
	"Weapon_Sword_015": "Guardian Sword++",
//   - GuardianSwordPlusPlus:
//       durability: 32
	"Weapon_Sword_016": "Lynel Sword",
//   - LynelSword:
//       durability: 26
	"Weapon_Sword_017": "Mighty Lynel Sword",
//   - MightyLynelSword:
//       durability: 32
	"Weapon_Sword_018": "Savage Lynel Sword",
//   - SavageLynelSword:
//       durability: 41
	"Weapon_Sword_019": "Bokoblin Arm",
//   - BokoblinArm:
//       durability: 5
	"Weapon_Sword_020": "Lizalfos Arm",
//   - LizalfosArm:
//       durability: 8
	"Weapon_Sword_021": "Rusty Broadsword",
//   - RustyBroadsword:
//       durability: 8
	"Weapon_Sword_022": "Soup Ladle",
//   - SoupLadle:
//       durability: 5
//       priority: 1
	"Weapon_Sword_023": "Ancient Short Sword",
//   - AncientShortSword:
//       durability: 54
	"Weapon_Sword_024": "Royal Broadsword",
//   - RoyalBroadsword:royalswordrsword:
//       durability: 36
	"Weapon_Sword_025": "Forest Dweller's Sword",
//   - ForestDwellersSword:fdsword:
//       durability: 27
	"Weapon_Sword_027": "Zora Sword",
//   - ZoraSword:
//       durability: 27
	"Weapon_Sword_029": "Gerudo Scimitar",
//   - GerudoScimitar:
//       durability: 23
	"Weapon_Sword_030": "Moonlight Scimitar",
//   - MoonlightScimitar:
//       durability: 32
	"Weapon_Sword_031": "Feathered Edge",
//   - FeatheredEdge:
//       durability: 27
	"Weapon_Sword_033": "Flameblade",
//   - Flameblade:
//       durability: 36
	"Weapon_Sword_034": "Frostblade",
//   - Frostblade:
//       durability: 30
	"Weapon_Sword_035": "Thunderblade",
//   - Thunderblade:
//       durability: 36
	"Weapon_Sword_040": "Spring-Loaded Hammer",
	"Weapon_Sword_041": "Eightfold Blade",
//   - EightfoldBlade:
//       durability: 26
	"Weapon_Sword_042": "Spring-Loaded Hammer",
//   - SpringLoadedHammer:
//       durability: 80
	"Weapon_Sword_043": "Torch",
//   - Torch:
//       durability: 8
//       priority: 1
	"Weapon_Sword_044": "Tree Branch",
//   - TreeBranch:
//       durability: 4
//       priority: 1
	"Weapon_Sword_047": "Royal Guard's Sword",
//   - RoyalGuardsSword:rgsword:
//       durability: 14
	"Weapon_Sword_048": "Meteor Rod",
//   - MeteorRod:
//       durability: 32
	"Weapon_Sword_049": "Blizzard Rod",
//   - BlizzardRod:
//       durability: 32
	"Weapon_Sword_050": "Thunderstorm Rod",
//   - ThunderstormRod:
//       durability: 32
	"Weapon_Sword_051": "Boomerang",
//   - Boomerang:
//       durability: 18
	"Weapon_Sword_052": "Scimitar of the Seven",
//   - ScimitarOfTheSeven:
//       durability: 60
	"Weapon_Sword_053": "Vicious Sickle",
//   - ViciousSickle:
//       durability: 14
	"Weapon_Sword_057": "Goddess Sword",
//   - GoddessSword:
//       durability: 45
	"Weapon_Sword_058": "Sword",
//   - Sword:
//       durability: 27
	"Weapon_Sword_059": "Sea-Breeze Boomerang",
//   - SeaBreezeBoomerang:
//       durability: 20
	"Weapon_Sword_060": "Fire Rod",
//   - FireRod:
//       durability: 14
	"Weapon_Sword_061": "Ice Rod",
//   - IceRod:
//       durability: 14
	"Weapon_Sword_062": "Lightning Rod",
//   - LightningRod:
//       durability: 14
	"Weapon_Sword_070": "Master Sword",
	"Weapon_Sword_073": "Demon Carver",
//   - DemonCarver:
//       durability: 25
"Weapon_Sword_502": "One-Hit Obliterator",
//   - OneHitObliterator:oho:
//       durability: 40
};

