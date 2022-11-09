import clsx from "clsx";
import { DisplayableSlot } from "core/inventory/DisplayableInventory";
import { useI18n } from "data/i18n";
import { Tooltip } from "ui/surfaces";

type ItemSlotProps = {
  slot: DisplayableSlot
};

export const ItemSlot: React.FC<ItemSlotProps> = ({slot: {
	image,
	getTooltip,
	count,
	durability,
	isBrokenSlot,
	isEquipped,
	propertyString
}})=>{
	const t = useI18n();
	const [propertyText, propertyClass] = propertyString;
	const tooltips = getTooltip(t);
	if(isBrokenSlot){
		tooltips.push(["Will not be removed on reload", "ItemTooltipBrokenSlot"]);
	}
	const tooltipNodes = tooltips.map(([text, className])=>{
		if (!className){
			return text;
		}
		return <span className={className}>{text}</span>
	});
	return (
		<Tooltip title={tooltipNodes}>
			<span className="ItemSlot">
				<div className={clsx("ItemLayer",isBrokenSlot&&"ItemSlotBroken")} style={{zIndex: 1}}>
					<div className={clsx("ItemSlot", "ItemSlotNoBg", isEquipped && "ItemSlotEquipped")}>
						<img className={clsx("ItemImage")} src={image}/>
					</div>
					
				</div>
				{
					count!==undefined && 
					<div className="ItemLayer" style={{zIndex: 2}}>
							<span className={"ItemCount"}>
							x{count}
							</span>
					</div>
				}
				{
					durability!==undefined && 
					<div className="ItemLayer" style={{zIndex: 2}}>
						<span className="ItemFloatWindow ItemDurability">
							{durability}
						</span>
					</div>
				}
				{
					propertyText && 
					<div className="ItemLayer" style={{zIndex: 2}}>
						<span className={clsx("ItemFloatWindow ItemPropertyString", propertyClass)}>
							{propertyText}
						</span>
					</div>
				}

			</span>
		</Tooltip>
		
	);
};

export const DoubleItemSlot: React.FC<{first?: ItemSlotProps, second?: ItemSlotProps}> = ({first, second})=>{
	return (
		<span style={{display: "inline-block", width: 72, height: 144, verticalAlign:"top"}}>
			<div className="SheikaBackground" style={{height: 72}} >
				{first && <ItemSlot {...first}/>}
			</div>
			<div style={{height: 72}}>
				{second && <ItemSlot {...second}/>}
			</div>
		</span>
	);
};
