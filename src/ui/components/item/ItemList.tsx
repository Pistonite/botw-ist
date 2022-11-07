import { DisplayableSlot } from "core/DisplayableInventory";
import { ItemSlot } from "./ItemSlot";

export type ItemListProps = {
    slots: DisplayableSlot[]
}

export const ItemList: React.FC<ItemListProps> = ({slots}) => {
	return <>
		{
			slots.map((slot, i)=>{
				return <ItemSlot key={i} slot={slot}/>;
			})
		}
	</>;
};
