import clsx from "clsx";
type ItemSlotProps = {
  image: string,
  count: number,
  isBroken: boolean,
  isSave: boolean,
  isEquipped: boolean,
};

export const ItemSlot: React.FC<ItemSlotProps> = ({image, count, isBroken, isSave, isEquipped})=>{
	return (
    <span className={clsx("ItemSlot", isBroken && "ItemSlotBroken", isSave && "ItemSlotSave", isEquipped && "ItemSlotEquipped")}>
      <img className={clsx("ItemImage", isSave && "ItemImageSave")}src={image} />
      {
        count > 0 && <span className={"ItemCount"}>
          x{count}
        </span> 
      }
    </span>
  );
};

export const DoubleItemSlot: React.FC<{first?: ItemSlotProps, second?: ItemSlotProps}> = ({first, second})=>{
  return (
    <span style={{display: "inline-block", width: 72, height: 144, verticalAlign:"top"}}>
      {first ? <ItemSlot {...first}/> : <div style={{height: 72}}/>}
      {second ? <ItemSlot {...second}/> : <div style={{height: 72}}/> }
    </span>
  )
} 