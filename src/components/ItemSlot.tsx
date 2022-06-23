import clsx from "clsx";
import { DisplayableSlot } from "core/DisplayableInventory";
import Background from "assets/Background.png";


type ItemSlotProps = {
  slot: DisplayableSlot
};

export const ItemSlot: React.FC<ItemSlotProps> = ({slot: {image, count, isBrokenSlot, isEquipped, displayCount}})=>{
	return (
		<span className={clsx("ItemSlot", isBrokenSlot && "ItemSlotBroken", isEquipped && "ItemSlotEquipped")}>
			<img className={clsx("ItemImage")} src={image} />
			{
				displayCount && <span className={"ItemCount"}>
          x{count}
				</span> 
			}
		</span>
	);
};

export const DoubleItemSlot: React.FC<{first?: ItemSlotProps, second?: ItemSlotProps}> = ({first, second})=>{
	return (
		<span style={{display: "inline-block", width: 72, height: 144, verticalAlign:"top"}}>
			<div style={{height: 72, background: `url(${Background})`}} >
				{first && <ItemSlot {...first}/>}
			</div>
			<div style={{height: 72}}>
				{second && <ItemSlot {...second}/>}
			</div>
		</span>
	);
}; 
