import { ItemSlot } from "components/ItemSlot";
import { BodyText} from "components/Text";
import { itemStackToDisplayableSlot } from "core/DisplayableInventory";
import { useAllItems } from "data/item";
import { Page } from "ui/surfaces";
import { useRuntime } from "data/runtime";

export const GalleryPage: React.FC = ()=>{
	const allItems = useAllItems();
	const { setting } = useRuntime();
	const isIconAnimated = setting("animatedIcon");
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
};
