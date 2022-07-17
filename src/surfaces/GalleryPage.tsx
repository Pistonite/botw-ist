import { ItemSlot } from "components/ItemSlot";
import { BodyText} from "components/Text";
import { TitledList } from "components/TitledList";
import { itemStackToDisplayableSlot } from "core/DisplayableInventory";
import { useAllItems } from "data/item";
import React, { PropsWithChildren } from "react";

type Prop = PropsWithChildren<{
    isIconAnimated: boolean
}>;
export const GalleryPage: React.FC<Prop> = React.memo(({isIconAnimated})=>{
	const allItems = useAllItems();
	return (
		<div className="OtherPage">
			<TitledList title="Item Gallery">
				<div className="OtherPageContent">
					<BodyText>
						You can find every single item here. 
					</BodyText>
					<BodyText>
						The value at the bottom left of equipments is the default durability
					</BodyText>
					<div>
						{
							Object.values(allItems).map((item, i)=>{
								return <ItemSlot key={i} slot={itemStackToDisplayableSlot(
									item.createDefaultStack(),
									false,
									isIconAnimated
								)} />;
							})
						}
					</div>
				</div>
                
			</TitledList>
		</div>
	);
});
