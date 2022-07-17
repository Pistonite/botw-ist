import clsx from "clsx";
import { DisplayableSlot } from "core/DisplayableInventory";
import Background from "assets/Background.png";
import { useI18n } from "data/i18n";

type ItemSlotProps = {
  slot: DisplayableSlot
};

export const ItemSlot: React.FC<ItemSlotProps> = ({slot: {
	image, 
	descKey, 
	count, 
	durability,
	isBrokenSlot, 
	isEquipped
}})=>{
	const t = useI18n();
	return (
		<span className={clsx("ItemSlot", isBrokenSlot && "ItemSlotBroken", isEquipped && "ItemSlotEquipped")} 
			title={t(descKey)}
		>
			<img className={clsx("ItemImage")} src={image} />
			{
				count!==undefined && <div className="ItemLayer">
					{
						<span className={"ItemCount"}>
                          x{count}
						</span> 
					}
				</div>
			}
			{
				durability!==undefined && <div className="ItemLayer">
					{
						<span className={"ItemDurability"}>
							{durability}
						</span> 
					}
				</div>
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
