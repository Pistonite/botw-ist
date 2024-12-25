import modifiers from "./modifiers.webp?url";
export const ModifierChunkClasses = {
    ".sprite-modifiers": { backgroundImage: `url(${modifiers})` },
} as const;
/** Modifier => [Chunk, Position]*/
export type ModifierMetadata = Record<string,[0,number]>;
export const ModifierMetadata: ModifierMetadata = JSON.parse(`{"AddGuard":[0,0],"AddGuardPlus":[0,1],"AddLife":[0,2],"AddLifePlus":[0,3],"AddPower":[0,4],"AddPowerPlus":[0,5],"AddPowerPlus_Bow":[0,6],"AddPower_Bow":[0,7],"AllSpeed":[0,8],"AttackUp":[0,9],"ClimbSpeedUp":[0,10],"Critical":[0,11],"DefenseUp":[0,12],"ExGutsMaxUp":[0,13],"Fireproof":[0,14],"GutsRecover":[0,15],"LifeMaxUp":[0,16],"LongThrow":[0,17],"Quietness":[0,18],"RapidFire":[0,19],"ReduceAncientEnemyDamge":[0,20],"ResistCold":[0,21],"ResistElectric":[0,22],"ResistFreeze":[0,23],"ResistHot":[0,24],"ResistLightning":[0,25],"SandMoveSpeedUp":[0,26],"SnowMovingSpeed":[0,27],"SpreadFire_3":[0,28],"SpreadFire_5":[0,29],"SpreadFire_X":[0,30],"SurfMaster":[0,31],"SurfMaster_White":[0,32],"SwimSpeedUp":[0,33],"Zoom":[0,34],"Zoom_White":[0,35]}`)
