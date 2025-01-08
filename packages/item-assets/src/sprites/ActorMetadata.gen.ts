import chunk0x32 from "./chunk0x32.webp?url";
import chunk1x32 from "./chunk1x32.webp?url";
import chunk2x32 from "./chunk2x32.webp?url";
import chunk0x64 from "./chunk0x64.webp?url";
import chunk1x64 from "./chunk1x64.webp?url";
import chunk2x64 from "./chunk2x64.webp?url";
export const ActorChunkClasses = {
    ".sprite-chunk0x32": { backgroundImage: `url(${chunk0x32})` },
    ".sprite-chunk1x32": { backgroundImage: `url(${chunk1x32})` },
    ".sprite-chunk2x32": { backgroundImage: `url(${chunk2x32})` },
    ".sprite-chunk0x64": { backgroundImage: `url(${chunk0x64})` },
    ".sprite-chunk1x64": { backgroundImage: `url(${chunk1x64})` },
    ".sprite-chunk2x64": { backgroundImage: `url(${chunk2x64})` },
    ".sprite-mask-chunk0x32": { maskImage: `url(${chunk0x32})` },
    ".sprite-mask-chunk1x32": { maskImage: `url(${chunk1x32})` },
    ".sprite-mask-chunk2x32": { maskImage: `url(${chunk2x32})` },
    ".sprite-mask-chunk0x64": { maskImage: `url(${chunk0x64})` },
    ".sprite-mask-chunk1x64": { maskImage: `url(${chunk1x64})` },
    ".sprite-mask-chunk2x64": { maskImage: `url(${chunk2x64})` },
} as const;
/** Actor => [Chunk, Position]*/
export type ActorMetadata = Record<string,[0|1|2,number]>;
export const ActorMetadata: ActorMetadata = JSON.parse(`{"AncientArrow":[1,0],"Animal_Insect_A":[0,0],"Animal_Insect_AA":[0,1],"Animal_Insect_AB":[0,2],"Animal_Insect_B":[0,3],"Animal_Insect_C":[0,4],"Animal_Insect_E":[0,5],"Animal_Insect_F":[0,6],"Animal_Insect_G":[0,7],"Animal_Insect_H":[0,8],"Animal_Insect_I":[0,9],"Animal_Insect_M":[0,10],"Animal_Insect_N":[0,11],"Animal_Insect_P":[0,12],"Animal_Insect_Q":[0,13],"Animal_Insect_R":[0,14],"Animal_Insect_S":[0,15],"Animal_Insect_T":[0,16],"Animal_Insect_X":[0,17],"Armor_001_Head":[2,0],"Armor_001_Lower":[2,47],"Armor_001_Upper":[2,76],"Armor_005_Head":[2,1],"Armor_005_Lower":[2,48],"Armor_005_Upper":[2,77],"Armor_006_Head":[2,2],"Armor_006_Lower":[2,49],"Armor_006_Upper":[2,78],"Armor_008_Head":[2,3],"Armor_008_Lower":[2,50],"Armor_008_Upper":[2,79],"Armor_009_Head":[2,4],"Armor_009_Lower":[2,51],"Armor_009_Upper":[2,80],"Armor_011_Head":[2,5],"Armor_011_Lower":[2,52],"Armor_011_Upper":[2,81],"Armor_012_Head":[2,6],"Armor_012_Lower":[2,53],"Armor_012_Upper":[2,82],"Armor_014_Head":[2,7],"Armor_014_Lower":[2,54],"Armor_014_Upper":[2,83],"Armor_017_Head":[2,8],"Armor_017_Lower":[2,55],"Armor_017_Upper":[2,84],"Armor_020_Head":[2,9],"Armor_020_Lower":[2,56],"Armor_020_Upper":[2,85],"Armor_021_Head":[2,10],"Armor_021_Lower":[2,57],"Armor_021_Upper":[2,86],"Armor_022_Head":[2,11],"Armor_043_Lower":[2,58],"Armor_043_Upper":[2,87],"Armor_044_Upper":[2,88],"Armor_045_Head":[2,12],"Armor_046_Head":[2,13],"Armor_046_Lower":[2,59],"Armor_046_Upper":[2,89],"Armor_048_Head":[2,14],"Armor_048_Lower":[2,60],"Armor_048_Upper":[2,90],"Armor_049_Lower":[2,61],"Armor_053_Head":[2,15],"Armor_053_Lower":[2,62],"Armor_053_Upper":[2,91],"Armor_055_Head":[2,16],"Armor_056_Head":[2,17],"Armor_115_Head":[2,18],"Armor_116_Upper":[2,92],"Armor_141_Lower":[2,63],"Armor_160_Head":[2,19],"Armor_160_Lower":[2,64],"Armor_160_Upper":[2,93],"Armor_170_Upper":[2,94],"Armor_171_Head":[2,20],"Armor_171_Lower":[2,65],"Armor_171_Upper":[2,95],"Armor_172_Head":[2,21],"Armor_173_Head":[2,22],"Armor_174_Head":[2,23],"Armor_174_Lower":[2,66],"Armor_174_Upper":[2,96],"Armor_175_Upper":[2,97],"Armor_176_Head":[2,24],"Armor_177_Head":[2,25],"Armor_178_Head":[2,26],"Armor_179_Head":[2,27],"Armor_179_Lower":[2,67],"Armor_179_Upper":[2,98],"Armor_180_Head":[2,28],"Armor_180_Lower":[2,68],"Armor_180_Upper":[2,99],"Armor_181_Head":[2,29],"Armor_182_Head":[2,30],"Armor_183_Head":[2,31],"Armor_184_Head":[2,32],"Armor_185_Head":[2,33],"Armor_185_Lower":[2,69],"Armor_185_Upper":[2,100],"Armor_200_Head":[2,34],"Armor_200_Lower":[2,70],"Armor_200_Upper":[2,101],"Armor_205_Head":[2,35],"Armor_205_Lower":[2,71],"Armor_205_Upper":[2,102],"Armor_210_Head":[2,36],"Armor_210_Lower":[2,72],"Armor_210_Upper":[2,103],"Armor_215_Head":[2,37],"Armor_215_Lower":[2,73],"Armor_215_Upper":[2,104],"Armor_220_Head":[2,38],"Armor_225_Head":[2,39],"Armor_225_Lower":[2,74],"Armor_225_Upper":[2,105],"Armor_230_Head":[2,40],"Armor_230_Lower":[2,75],"Armor_230_Upper":[2,106],"BeeHome":[0,18],"BombArrow_A":[1,1],"BrightArrow":[1,2],"BrightArrowTP":[1,3],"Dummy":[2,248],"ElectricArrow":[1,4],"FireArrow":[1,5],"GameRomHorseReins_01":[2,107],"GameRomHorseReins_02":[2,108],"GameRomHorseReins_03":[2,109],"GameRomHorseReins_04":[2,110],"GameRomHorseReins_05":[2,111],"GameRomHorseReins_10":[2,112],"GameRomHorseSaddle_01":[2,113],"GameRomHorseSaddle_02":[2,114],"GameRomHorseSaddle_03":[2,115],"GameRomHorseSaddle_04":[2,116],"GameRomHorseSaddle_05":[2,117],"GameRomHorseSaddle_10":[2,118],"Get_TwnObj_DLC_MemorialPicture_A_01":[0,19],"IceArrow":[1,6],"Item_Boiled_01":[0,20],"Item_ChilledFish_01":[0,21],"Item_ChilledFish_02":[0,22],"Item_ChilledFish_03":[0,23],"Item_ChilledFish_04":[0,24],"Item_ChilledFish_05":[0,25],"Item_ChilledFish_06":[0,26],"Item_ChilledFish_07":[0,27],"Item_ChilledFish_08":[0,28],"Item_ChilledFish_09":[0,29],"Item_Chilled_01":[0,30],"Item_Chilled_02":[0,31],"Item_Chilled_03":[0,32],"Item_Chilled_04":[0,33],"Item_Chilled_05":[0,34],"Item_Chilled_06":[0,35],"Item_Cook_A_01":[2,119],"Item_Cook_A_02":[2,120],"Item_Cook_A_03":[2,121],"Item_Cook_A_04":[2,122],"Item_Cook_A_05":[2,123],"Item_Cook_A_07":[2,124],"Item_Cook_A_08":[2,125],"Item_Cook_A_09":[2,126],"Item_Cook_A_10":[2,127],"Item_Cook_A_11":[2,128],"Item_Cook_A_12":[2,129],"Item_Cook_A_13":[2,130],"Item_Cook_A_14":[2,131],"Item_Cook_B_01":[2,132],"Item_Cook_B_02":[2,133],"Item_Cook_B_05":[2,134],"Item_Cook_B_06":[2,135],"Item_Cook_B_11":[2,136],"Item_Cook_B_12":[2,137],"Item_Cook_B_13":[2,138],"Item_Cook_B_15":[2,139],"Item_Cook_B_16":[2,140],"Item_Cook_B_17":[2,141],"Item_Cook_B_18":[2,142],"Item_Cook_B_19":[2,143],"Item_Cook_B_20":[2,144],"Item_Cook_B_21":[2,145],"Item_Cook_B_22":[2,146],"Item_Cook_B_23":[2,147],"Item_Cook_C_16":[2,148],"Item_Cook_C_17":[2,149],"Item_Cook_C_17_AllSpeed":[2,150],"Item_Cook_C_17_AttackUp":[2,151],"Item_Cook_C_17_DefenseUp":[2,152],"Item_Cook_C_17_ExGutsMaxUp":[2,153],"Item_Cook_C_17_Fireproof":[2,154],"Item_Cook_C_17_GutsRecover":[2,155],"Item_Cook_C_17_LifeMaxUp":[2,156],"Item_Cook_C_17_Quietness":[2,157],"Item_Cook_C_17_ResistCold":[2,158],"Item_Cook_C_17_ResistElectric":[2,159],"Item_Cook_C_17_ResistHot":[2,160],"Item_Cook_D_01":[2,161],"Item_Cook_D_02":[2,162],"Item_Cook_D_03":[2,163],"Item_Cook_D_04":[2,164],"Item_Cook_D_05":[2,165],"Item_Cook_D_06":[2,166],"Item_Cook_D_07":[2,167],"Item_Cook_D_08":[2,168],"Item_Cook_D_09":[2,169],"Item_Cook_D_10":[2,170],"Item_Cook_E_01":[2,171],"Item_Cook_E_02":[2,172],"Item_Cook_E_03":[2,173],"Item_Cook_E_04":[2,174],"Item_Cook_F_01":[2,175],"Item_Cook_F_02":[2,176],"Item_Cook_F_03":[2,177],"Item_Cook_F_04":[2,178],"Item_Cook_G_02":[2,179],"Item_Cook_G_03":[2,180],"Item_Cook_G_04":[2,181],"Item_Cook_G_05":[2,182],"Item_Cook_G_06":[2,183],"Item_Cook_G_09":[2,184],"Item_Cook_G_10":[2,185],"Item_Cook_G_11":[2,186],"Item_Cook_G_12":[2,187],"Item_Cook_G_13":[2,188],"Item_Cook_G_14":[2,189],"Item_Cook_G_15":[2,190],"Item_Cook_G_16":[2,191],"Item_Cook_G_17":[2,192],"Item_Cook_H_01":[2,193],"Item_Cook_H_02":[2,194],"Item_Cook_H_03":[2,195],"Item_Cook_I_01":[2,196],"Item_Cook_I_02":[2,197],"Item_Cook_I_03":[2,198],"Item_Cook_I_04":[2,199],"Item_Cook_I_05":[2,200],"Item_Cook_I_06":[2,201],"Item_Cook_I_07":[2,202],"Item_Cook_I_08":[2,203],"Item_Cook_I_09":[2,204],"Item_Cook_I_10":[2,205],"Item_Cook_I_11":[2,206],"Item_Cook_I_12":[2,207],"Item_Cook_I_13":[2,208],"Item_Cook_I_14":[2,209],"Item_Cook_I_15":[2,210],"Item_Cook_I_16":[2,211],"Item_Cook_I_17":[2,212],"Item_Cook_J_01":[2,213],"Item_Cook_J_02":[2,214],"Item_Cook_J_03":[2,215],"Item_Cook_J_04":[2,216],"Item_Cook_J_05":[2,217],"Item_Cook_J_06":[2,218],"Item_Cook_J_07":[2,219],"Item_Cook_J_08":[2,220],"Item_Cook_J_09":[2,221],"Item_Cook_K_01":[2,222],"Item_Cook_K_02":[2,223],"Item_Cook_K_03":[2,224],"Item_Cook_K_04":[2,225],"Item_Cook_K_05":[2,226],"Item_Cook_K_06":[2,227],"Item_Cook_K_07":[2,228],"Item_Cook_K_08":[2,229],"Item_Cook_K_09":[2,230],"Item_Cook_L_01":[2,231],"Item_Cook_L_02":[2,232],"Item_Cook_L_03":[2,233],"Item_Cook_L_04":[2,234],"Item_Cook_L_05":[2,235],"Item_Cook_M_01":[2,236],"Item_Cook_N_01":[2,237],"Item_Cook_N_02":[2,238],"Item_Cook_N_03":[2,239],"Item_Cook_N_04":[2,240],"Item_Cook_O_01":[2,241],"Item_Cook_O_02":[2,242],"Item_Cook_P_01":[2,243],"Item_Cook_P_02":[2,244],"Item_Cook_P_03":[2,245],"Item_Cook_P_04":[2,246],"Item_Cook_P_05":[2,247],"Item_Enemy_00":[0,36],"Item_Enemy_01":[0,37],"Item_Enemy_02":[0,38],"Item_Enemy_03":[0,39],"Item_Enemy_04":[0,40],"Item_Enemy_05":[0,41],"Item_Enemy_06":[0,42],"Item_Enemy_07":[0,43],"Item_Enemy_08":[0,44],"Item_Enemy_12":[0,45],"Item_Enemy_13":[0,46],"Item_Enemy_14":[0,47],"Item_Enemy_15":[0,48],"Item_Enemy_16":[0,49],"Item_Enemy_17":[0,50],"Item_Enemy_18":[0,51],"Item_Enemy_19":[0,52],"Item_Enemy_20":[0,53],"Item_Enemy_21":[0,54],"Item_Enemy_24":[0,55],"Item_Enemy_25":[0,56],"Item_Enemy_26":[0,57],"Item_Enemy_27":[0,58],"Item_Enemy_28":[0,59],"Item_Enemy_29":[0,60],"Item_Enemy_30":[0,61],"Item_Enemy_31":[0,62],"Item_Enemy_32":[0,63],"Item_Enemy_33":[0,64],"Item_Enemy_34":[0,65],"Item_Enemy_38":[0,66],"Item_Enemy_39":[0,67],"Item_Enemy_40":[0,68],"Item_Enemy_41":[0,69],"Item_Enemy_42":[0,70],"Item_Enemy_43":[0,71],"Item_Enemy_44":[0,72],"Item_Enemy_45":[0,73],"Item_Enemy_46":[0,74],"Item_Enemy_47":[0,75],"Item_Enemy_48":[0,76],"Item_Enemy_49":[0,77],"Item_Enemy_50":[0,78],"Item_Enemy_51":[0,79],"Item_Enemy_52":[0,80],"Item_Enemy_53":[0,81],"Item_Enemy_54":[0,82],"Item_Enemy_55":[0,83],"Item_Enemy_56":[0,84],"Item_Enemy_57":[0,85],"Item_FishGet_A":[0,86],"Item_FishGet_B":[0,87],"Item_FishGet_C":[0,88],"Item_FishGet_D":[0,89],"Item_FishGet_E":[0,90],"Item_FishGet_F":[0,91],"Item_FishGet_G":[0,92],"Item_FishGet_H":[0,93],"Item_FishGet_I":[0,94],"Item_FishGet_J":[0,95],"Item_FishGet_K":[0,96],"Item_FishGet_L":[0,97],"Item_FishGet_M":[0,98],"Item_FishGet_X":[0,99],"Item_FishGet_Z":[0,100],"Item_Fruit_A":[0,101],"Item_Fruit_B":[0,102],"Item_Fruit_C":[0,103],"Item_Fruit_D":[0,104],"Item_Fruit_E":[0,105],"Item_Fruit_F":[0,106],"Item_Fruit_G":[0,107],"Item_Fruit_H":[0,108],"Item_Fruit_I":[0,109],"Item_Fruit_J":[0,110],"Item_Fruit_K":[0,111],"Item_Fruit_L":[0,112],"Item_InsectGet_K":[0,113],"Item_InsectGet_O":[0,114],"Item_InsectGet_Z":[0,115],"Item_Material_01":[0,116],"Item_Material_02":[0,117],"Item_Material_03":[0,118],"Item_Material_04":[0,119],"Item_Material_05":[0,120],"Item_Material_06":[0,121],"Item_Material_07":[0,122],"Item_Material_08":[0,123],"Item_Meat_01":[0,124],"Item_Meat_02":[0,125],"Item_Meat_06":[0,126],"Item_Meat_07":[0,127],"Item_Meat_11":[0,128],"Item_Meat_12":[0,129],"Item_MushroomGet_D":[0,130],"Item_Mushroom_A":[0,131],"Item_Mushroom_B":[0,132],"Item_Mushroom_C":[0,133],"Item_Mushroom_E":[0,134],"Item_Mushroom_F":[0,135],"Item_Mushroom_H":[0,136],"Item_Mushroom_J":[0,137],"Item_Mushroom_L":[0,138],"Item_Mushroom_M":[0,139],"Item_Mushroom_N":[0,140],"Item_Mushroom_O":[0,141],"Item_Ore_A":[0,142],"Item_Ore_B":[0,143],"Item_Ore_C":[0,144],"Item_Ore_D":[0,145],"Item_Ore_E":[0,146],"Item_Ore_F":[0,147],"Item_Ore_G":[0,148],"Item_Ore_H":[0,149],"Item_Ore_I":[0,150],"Item_Ore_J":[0,151],"Item_PlantGet_A":[0,152],"Item_PlantGet_B":[0,153],"Item_PlantGet_C":[0,154],"Item_PlantGet_E":[0,155],"Item_PlantGet_F":[0,156],"Item_PlantGet_G":[0,157],"Item_PlantGet_H":[0,158],"Item_PlantGet_I":[0,159],"Item_PlantGet_J":[0,160],"Item_PlantGet_L":[0,161],"Item_PlantGet_M":[0,162],"Item_PlantGet_O":[0,163],"Item_PlantGet_Q":[0,164],"Item_RoastFish_01":[0,165],"Item_RoastFish_02":[0,166],"Item_RoastFish_03":[0,167],"Item_RoastFish_04":[0,168],"Item_RoastFish_07":[0,169],"Item_RoastFish_09":[0,170],"Item_RoastFish_11":[0,171],"Item_RoastFish_13":[0,172],"Item_RoastFish_15":[0,173],"Item_Roast_01":[0,174],"Item_Roast_02":[0,175],"Item_Roast_03":[0,176],"Item_Roast_04":[0,177],"Item_Roast_05":[0,178],"Item_Roast_06":[0,179],"Item_Roast_07":[0,180],"Item_Roast_08":[0,181],"Item_Roast_09":[0,182],"Item_Roast_10":[0,183],"Item_Roast_11":[0,184],"Item_Roast_12":[0,185],"Item_Roast_13":[0,186],"Item_Roast_15":[0,187],"Item_Roast_16":[0,188],"Item_Roast_18":[0,189],"Item_Roast_19":[0,190],"Item_Roast_24":[0,191],"Item_Roast_27":[0,192],"Item_Roast_28":[0,193],"Item_Roast_31":[0,194],"Item_Roast_32":[0,195],"Item_Roast_33":[0,196],"Item_Roast_36":[0,197],"Item_Roast_37":[0,198],"Item_Roast_38":[0,199],"Item_Roast_39":[0,200],"Item_Roast_40":[0,201],"Item_Roast_41":[0,202],"Item_Roast_45":[0,203],"Item_Roast_46":[0,204],"Item_Roast_48":[0,205],"Item_Roast_49":[0,206],"Item_Roast_50":[0,207],"Item_Roast_51":[0,208],"Item_Roast_52":[0,209],"Item_Roast_53":[0,210],"NormalArrow":[1,7],"Obj_Armor_115_Head":[0,211],"Obj_DLC_HeroSeal_Gerudo":[0,212],"Obj_DLC_HeroSeal_Goron":[0,213],"Obj_DLC_HeroSeal_Rito":[0,214],"Obj_DLC_HeroSeal_Zora":[0,215],"Obj_DLC_HeroSoul_Gerudo_Disabled":[0,216],"Obj_DLC_HeroSoul_Goron_Disabled":[0,217],"Obj_DLC_HeroSoul_Rito_Disabled":[0,218],"Obj_DLC_HeroSoul_Zora_Disabled":[0,219],"Obj_DRStone_Get":[0,234],"Obj_DungeonClearSeal":[0,220],"Obj_FireWoodBundle":[0,221],"Obj_Head_024":[2,41],"Obj_Head_025":[2,42],"Obj_Head_026":[2,43],"Obj_Head_027":[2,44],"Obj_Head_028":[2,45],"Obj_Head_029":[2,46],"Obj_HeroSoul_Gerudo_Disabled":[0,222],"Obj_HeroSoul_Goron_Disabled":[0,223],"Obj_HeroSoul_Rito_Disabled":[0,224],"Obj_HeroSoul_Zora_Disabled":[0,225],"Obj_KorokNuts":[0,226],"Obj_Maracas":[0,227],"Obj_ProofBook":[0,228],"Obj_ProofGiantKiller":[0,229],"Obj_ProofGolemKiller":[0,230],"Obj_ProofKorok":[0,231],"Obj_ProofSandwormKiller":[0,232],"Obj_WarpDLC":[0,233],"PlayerStole2":[0,235],"Weapon_Bow_001":[1,8],"Weapon_Bow_002":[1,9],"Weapon_Bow_003":[1,10],"Weapon_Bow_004":[1,11],"Weapon_Bow_006":[1,12],"Weapon_Bow_009":[1,13],"Weapon_Bow_011":[1,14],"Weapon_Bow_013":[1,15],"Weapon_Bow_014":[1,16],"Weapon_Bow_015":[1,17],"Weapon_Bow_016":[1,18],"Weapon_Bow_017":[1,19],"Weapon_Bow_023":[1,20],"Weapon_Bow_026":[1,21],"Weapon_Bow_027":[1,22],"Weapon_Bow_028":[1,23],"Weapon_Bow_029":[1,24],"Weapon_Bow_030":[1,25],"Weapon_Bow_032":[1,26],"Weapon_Bow_033":[1,27],"Weapon_Bow_035":[1,28],"Weapon_Bow_036":[1,29],"Weapon_Bow_038":[1,30],"Weapon_Bow_040":[1,31],"Weapon_Bow_071":[1,32],"Weapon_Bow_072":[1,33],"Weapon_Lsword_001":[1,34],"Weapon_Lsword_002":[1,35],"Weapon_Lsword_003":[1,36],"Weapon_Lsword_004":[1,37],"Weapon_Lsword_005":[1,38],"Weapon_Lsword_006":[1,39],"Weapon_Lsword_010":[1,40],"Weapon_Lsword_011":[1,41],"Weapon_Lsword_012":[1,42],"Weapon_Lsword_013":[1,43],"Weapon_Lsword_014":[1,44],"Weapon_Lsword_015":[1,45],"Weapon_Lsword_016":[1,46],"Weapon_Lsword_017":[1,47],"Weapon_Lsword_018":[1,48],"Weapon_Lsword_019":[1,49],"Weapon_Lsword_020":[1,50],"Weapon_Lsword_023":[1,51],"Weapon_Lsword_024":[1,52],"Weapon_Lsword_027":[1,53],"Weapon_Lsword_029":[1,54],"Weapon_Lsword_030":[1,55],"Weapon_Lsword_031":[1,56],"Weapon_Lsword_032":[1,57],"Weapon_Lsword_033":[1,58],"Weapon_Lsword_034":[1,59],"Weapon_Lsword_035":[1,60],"Weapon_Lsword_036":[1,61],"Weapon_Lsword_037":[1,62],"Weapon_Lsword_038":[1,63],"Weapon_Lsword_041":[1,64],"Weapon_Lsword_045":[1,65],"Weapon_Lsword_047":[1,66],"Weapon_Lsword_051":[1,67],"Weapon_Lsword_054":[1,68],"Weapon_Lsword_055":[1,69],"Weapon_Lsword_056":[1,70],"Weapon_Lsword_057":[1,71],"Weapon_Lsword_059":[1,72],"Weapon_Lsword_060":[1,73],"Weapon_Lsword_074":[1,74],"Weapon_Shield_001":[1,75],"Weapon_Shield_002":[1,76],"Weapon_Shield_003":[1,77],"Weapon_Shield_004":[1,78],"Weapon_Shield_005":[1,79],"Weapon_Shield_006":[1,80],"Weapon_Shield_007":[1,81],"Weapon_Shield_008":[1,82],"Weapon_Shield_009":[1,83],"Weapon_Shield_013":[1,84],"Weapon_Shield_014":[1,85],"Weapon_Shield_015":[1,86],"Weapon_Shield_016":[1,87],"Weapon_Shield_017":[1,88],"Weapon_Shield_018":[1,89],"Weapon_Shield_021":[1,90],"Weapon_Shield_022":[1,91],"Weapon_Shield_023":[1,92],"Weapon_Shield_025":[1,93],"Weapon_Shield_026":[1,94],"Weapon_Shield_030":[1,95],"Weapon_Shield_031":[1,96],"Weapon_Shield_032":[1,97],"Weapon_Shield_033":[1,98],"Weapon_Shield_034":[1,99],"Weapon_Shield_035":[1,100],"Weapon_Shield_036":[1,101],"Weapon_Shield_037":[1,102],"Weapon_Shield_038":[1,103],"Weapon_Shield_040":[1,104],"Weapon_Shield_041":[1,105],"Weapon_Shield_042":[1,106],"Weapon_Shield_057":[1,107],"Weapon_Spear_001":[1,160],"Weapon_Spear_002":[1,161],"Weapon_Spear_003":[1,162],"Weapon_Spear_004":[1,163],"Weapon_Spear_005":[1,164],"Weapon_Spear_006":[1,165],"Weapon_Spear_007":[1,166],"Weapon_Spear_008":[1,167],"Weapon_Spear_009":[1,168],"Weapon_Spear_010":[1,169],"Weapon_Spear_011":[1,170],"Weapon_Spear_012":[1,171],"Weapon_Spear_013":[1,172],"Weapon_Spear_014":[1,173],"Weapon_Spear_015":[1,174],"Weapon_Spear_016":[1,175],"Weapon_Spear_017":[1,176],"Weapon_Spear_018":[1,177],"Weapon_Spear_021":[1,178],"Weapon_Spear_022":[1,179],"Weapon_Spear_023":[1,180],"Weapon_Spear_024":[1,181],"Weapon_Spear_025":[1,182],"Weapon_Spear_027":[1,183],"Weapon_Spear_028":[1,184],"Weapon_Spear_029":[1,185],"Weapon_Spear_030":[1,186],"Weapon_Spear_031":[1,187],"Weapon_Spear_032":[1,188],"Weapon_Spear_033":[1,189],"Weapon_Spear_034":[1,190],"Weapon_Spear_035":[1,191],"Weapon_Spear_036":[1,192],"Weapon_Spear_037":[1,193],"Weapon_Spear_038":[1,194],"Weapon_Spear_047":[1,195],"Weapon_Spear_049":[1,196],"Weapon_Spear_050":[1,197],"Weapon_Sword_001":[1,108],"Weapon_Sword_002":[1,109],"Weapon_Sword_003":[1,110],"Weapon_Sword_004":[1,111],"Weapon_Sword_005":[1,112],"Weapon_Sword_006":[1,113],"Weapon_Sword_007":[1,114],"Weapon_Sword_008":[1,115],"Weapon_Sword_009":[1,116],"Weapon_Sword_013":[1,117],"Weapon_Sword_014":[1,118],"Weapon_Sword_015":[1,119],"Weapon_Sword_016":[1,120],"Weapon_Sword_017":[1,121],"Weapon_Sword_018":[1,122],"Weapon_Sword_019":[1,123],"Weapon_Sword_020":[1,124],"Weapon_Sword_021":[1,125],"Weapon_Sword_022":[1,126],"Weapon_Sword_023":[1,127],"Weapon_Sword_024":[1,128],"Weapon_Sword_025":[1,129],"Weapon_Sword_027":[1,130],"Weapon_Sword_029":[1,131],"Weapon_Sword_030":[1,132],"Weapon_Sword_031":[1,133],"Weapon_Sword_033":[1,134],"Weapon_Sword_034":[1,135],"Weapon_Sword_035":[1,136],"Weapon_Sword_040":[1,137],"Weapon_Sword_041":[1,138],"Weapon_Sword_043":[1,139],"Weapon_Sword_044":[1,140],"Weapon_Sword_047":[1,141],"Weapon_Sword_048":[1,142],"Weapon_Sword_049":[1,143],"Weapon_Sword_050":[1,144],"Weapon_Sword_051":[1,145],"Weapon_Sword_052":[1,146],"Weapon_Sword_053":[1,147],"Weapon_Sword_057":[1,148],"Weapon_Sword_058":[1,149],"Weapon_Sword_059":[1,150],"Weapon_Sword_060":[1,151],"Weapon_Sword_061":[1,152],"Weapon_Sword_062":[1,153],"Weapon_Sword_070":[1,154],"Weapon_Sword_070_Disabled":[1,155],"Weapon_Sword_072":[1,156],"Weapon_Sword_073":[1,157],"Weapon_Sword_502":[1,158],"Weapon_Sword_503":[1,159]}`)