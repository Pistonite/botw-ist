import { ItemSlot } from "components/ItemSlot";
import { BodyText} from "components/Text";
import { Section } from "ui/components";
import { itemStackToDisplayableSlot } from "core/DisplayableInventory";
import { useAllItems } from "data/item";
import React, { PropsWithChildren } from "react";
import { Page } from "ui/surfaces";

type Prop = PropsWithChildren<{
    isIconAnimated: boolean
}>;
export const GalleryPage: React.FC<Prop> = React.memo(({isIconAnimated})=>{
	const allItems = useAllItems();
	return (
		<Page title="Item Gallery">
				
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
				

		</Page>
	);
});
