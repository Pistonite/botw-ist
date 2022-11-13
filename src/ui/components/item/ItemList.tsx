import { SlotDisplay } from "core/inventory";
import { ItemSlot } from "./ItemSlot";

export type ItemListProps = {
    slots: SlotDisplay[]
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
