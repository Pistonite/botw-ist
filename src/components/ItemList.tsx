import { ItemSlot } from "./ItemSlot";

export type ItemListItemProps = {
	image: string, 
	count: number, 
	isEquipped: boolean,
}

export type ItemListProps = {
	isSave: boolean,
    items: ItemListItemProps[],
    numBroken: number
}

export const ItemList: React.FC<ItemListProps> = ({items, numBroken, isSave}) => {
	return <>
		{
			items.map(({image, count, isEquipped}, i)=>{
				const broken = i+numBroken >= items.length;
				return <ItemSlot key={i} image={image} count={count} isBroken={broken} isSave={isSave} isEquipped={isEquipped}/>;
			})
		}
	</>;
};

