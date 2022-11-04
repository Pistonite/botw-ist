import { BodyText, Emphasized, Header, SubHeader, SubTitle } from "components/Text";
import React from "react";
import { Page } from "ui/surfaces";

export const ReferencePage: React.FC = React.memo(()=>{

	return (
		<Page title="Command Reference">

			<Header>Item Syntax</Header>
			<BodyText>
						Item are specified by using <Emphasized>search keys</Emphasized> and <Emphasized>metadata</Emphasized>.
			</BodyText>
			<SubHeader>Search Keys</SubHeader>
			<BodyText>
						Search keys are used to find the item. For example, in the command <Emphasized>"Add 1 master sword"</Emphasized>,
				<Emphasized>1</Emphasized> specifies the count, and <Emphasized>master sword</Emphasized> are 2 search keys to locate the item.
						Search keys are designed to "just work", but they sometimes find the wrong item. In this case you can add more keys to narrow the search.
						For example, <Emphasized>shroom</Emphasized> will match <Emphasized>Rushroom</Emphasized>, but <Emphasized>shroom hy</Emphasized> will match <Emphasized>HylianShroom</Emphasized>.
						Another example, <Emphasized>hammer</Emphasized> will match <Emphasized>IronSledgeHammer</Emphasized>, but <Emphasized>spr ham</Emphasized> will match <Emphasized>SpringLoadedHammer</Emphasized>
			</BodyText>
			<BodyText>
						In certain commands like <Emphasized>Equip</Emphasized>, you can only use 1 search key. In this case, you can use "*" to combine search keys into 1.
						For example, <Emphasized>snow*head</Emphasized> is one search key that functions like <Emphasized>snow head</Emphasized>, which will give you the snowquill headdress
			</BodyText>
			<BodyText>
						The order of the keys doesn't matter and they are case-insensitive.
			</BodyText>
			<SubHeader>Number of Items</SubHeader>
			<BodyText>
						When specifying number of items, you can use an integer or <Emphasized>all</Emphasized>, which is equivalent to -1.
						When using -1 as the count in Remove commands, it will remove all items.
			</BodyText>

			<SubHeader>Metadata</SubHeader>
			<BodyText>
						Metadata is used to specify extra information. The syntax is <Emphasized>[key1=value1,key2=value2,...]</Emphasized>.
						Metadata is not available in all commands. If you see <Emphasized>item[meta]</Emphasized>, then it's supported in that particular command.
			</BodyText>
			<BodyText>
						Here is a list of available metadata. More will be added as they are implemented in the future
			</BodyText>
			<SubTitle>life (integer)</SubTitle>
			<BodyText>Used to specify stack size or durability</BodyText>
			<BodyText emphasized>
						Example: hammer[life=300] makes a hammer with 3 durability
			</BodyText>
			<SubTitle>equip (boolean)</SubTitle>
			<BodyText>Used to specify if a slot is equipped</BodyText>
			<BodyText emphasized>
						Example: hammer[equip] makes the hammer equipped. [equip] and [equip=true] are the same
			</BodyText>

			<Header>Command List</Header>
			<BodyText>
                        This is a list of available commands. All commands and items are case-insensitive
			</BodyText>
			<SubHeader>Initialize</SubHeader>
			<SubHeader connected>Initialize X item1[meta] Y item2[meta] Z item3[meta] ...</SubHeader>
			<SubTitle>Used for initializing inventory before simulation</SubTitle>
			<BodyText>
                        Fully resets the inventory by clearing all items and set Count to 0, then forcefully write the item list to inventory.
                        This would reset any broken slot you already have, and any in-game checks that happen when adding items are disabled.
                        For example, the items will appear in the order you specify, not in the in-game tab order
			</BodyText>
			<BodyText>
                        If you specify count &gt; 1 for unstackable items like weapon or sheika slate, multiple of that item would be added.
                        Game Data will be synced with Visible Inventory after the reset
			</BodyText>
			<BodyText>
                        If you don't specify any item, it will make an empty inventory
			</BodyText>
			<BodyText>
                        Note that this will not clear saves. You can use this command to initialize multiple saves
			</BodyText>
			<BodyText emphasized>Example: Initialize 1 Apple 2 Axe 3 Slate 4 SpiritOrb</BodyText>

			<h3 className="Reference">Save</h3>
			<h3 className="Reference2">Save As NAME</h3>

			<h4 className="Reference">Simulates a hard save or auto save action</h4>
			<p className="Reference">
                        Writes Game Data to the corresponding save slot. The auto saves are specified by NAME.
                        You can have as many auto saves as you want in the simulator.
			</p>
			<p className="Reference">
                        You cannot save on Eventide/ToTS. However, the simulator does not enforce that.
			</p>

			<p className="Reference Example">Example 1: Save</p>
			<p className="Reference Example">Example 2: Save As MySave</p>
			<p className="Reference">
                        Example 1 will save to the manual save slot, while example 2 will save to the slot named "MySave".
                        There cannot be spaces in the name. If "MySave" doesn't exist, a new slot is created
			</p>

			<h3 className="Reference">Reload (NAME)</h3>
			<h4 className="Reference">Simulates reloading a save</h4>
			<p className="Reference">
                        First, reads Game Data from the corresponding save slot.
                        If NAME is not given, the manual save is used unless "Use" commands are used before this (see below).
                        If NAME is given, the corresponding save slot with that name is used
			</p>
			<p className="Reference">
                        After that, the first Count items in the visible inventory is removed, and Count is decreased accordingly.
                        Then, each item slot in the Game Data is added to the inventory.
			</p>

			<p className="Reference Example">Example 1: Reload</p>
			<p className="Reference Example">Example 2: Reload MySave</p>

			<h3 className="Reference">Break X Slots</h3>
			<h4 className="Reference">Simulate making X broken slots with hold smuggle glitch</h4>
			<p className="Reference">
                        Decrease inventory Count by X
			</p>
			<p className="Reference">
                        This command does not automatically simulate the hold smuggle and sell process.
                        It just changes count (i.e. make broken slots) with magic.
			</p>

			<p className="Reference Example">Example: Break 4 Slots</p>

			<h3 className="Reference">Get/Add/Cook/Pickup/Buy item</h3>
			<h3 className="Reference2">Get/Add/Cook/Pickup/Buy X item1[meta] Y item2[meta] Z item3[meta] ...</h3>
			<h4 className="Reference">Simulate obtaining items in game</h4>
			<p className="Reference">
                        Add the item(s) to visible inventory. Sync with Game Data unless you are on Eventide or inside TOTS
			</p>
			<p className="Reference">
                        Like in game, you won't be able to obtain multiple unstackable key items, or multiple master sword in this way.
                        If a stackable item is at 999 or more when you invoke this command, the count is set to 999 (not fully accurate since you won't be able to pick up more items in game).
			</p>
			<p className="Reference">
                        If you specify a count for unstackable items, they are added in different slots as if you pick them up in game, one after another.
			</p>
			<p className="Reference">
                        Try to avoid plural forms as they often make search fail
			</p>

			<p className="Reference Example">Example 1: Add Apple</p>
			<p className="Reference Example">Example 2: Get 10 Apple</p>
			<p className="Reference Example">Example 3: Pickup 10 Apple 5 Diamond 1 Slate 5 MasterSword[life=700]</p>

			<h3 className="Reference">With/Remove/Sell/Eat/Drop item</h3>
			<h3 className="Reference2">With/Remove/Sell/Eat/Drop item From Slot Y</h3>
			<h3 className="Reference2">With/Remove/Sell/Eat/Drop X item From Slot Y</h3>
			<h3 className="Reference2">With/Remove/Sell/Eat/Drop X item1[meta] Y item2[meta] Z item3[meta] ...</h3>
			<h4 className="Reference">Simulate removing items in game</h4>
			<p className="Reference">
                        Remove the item(s) to visible inventory. Sync with Game Data unless you are on Eventide or inside TOTS
			</p>
			<p className="Reference">
                        When number of item is not specified, it defaults to 1. Up to X items will be removed from inventory, even when they span multiple slots.
                        If X &gt; total number of items in inventory, all of them will be removed.
			</p>
			<p className="Reference">
                        When slot is specified, it starts removing from slot X (slot 1 is the leftmost slot with that item, slot 2 is the second leftmost slot with that item).
			</p>
			<BodyText>
						When removing items, the simulator will try to match the stack exactly, including metadata. If no items are found this way, the simulator will then try to match an item while ignoring the metadata
			</BodyText>
			<p className="Reference">
                        Try to avoid plural forms as they often make search fail
			</p>

			<p className="Reference Example">Example 1: Remove Apple</p>
			<p className="Reference Example">Example 2: Drop 10 Diamond</p>
			<p className="Reference Example">Example 3: Sell 10 Apple 5 Diamond</p>
			<p className="Reference Example">Example 4: Sell 5 Apple From Slot 3</p>

			<h3 className="Reference">D&amp;P X item1[meta] Y item2[meta] Z item3[meta] ...</h3>
			<h4 className="Reference">Shortcut for drop and pick up, for sorting inventory</h4>
			<p className="Reference">
                        This command drops and pick up each item stack in the specified order.
                        You can also repeat items if you are combining more than 2 slots.
			</p>
			<p className="Reference">
                        You can only drop from slot 1 with this shortcut.
			</p>
			<p className="Reference Example">Example 1: D&amp;P 5 Diamond</p>
			<p className="Reference Example">Example 2: D&amp;P 20 Shaft 5 Diamond</p>
			<p className="Reference Example">Example 3: D&amp;P 5 Diamond 10 Diamond</p>

			<h3 className="Reference">Equip item</h3>
			<h3 className="Reference2">Equip item In Slot X</h3>
			<h4 className="Reference">Simulates equipping something</h4>
			<p className="Reference">
                        When equipping an item, all other item of the same type in the first tab is unequipped, then the item selected is equipped.
			</p>
			<p className="Reference">
                        Slot can be used if you have multiple of the same item. When slot is not specified, the leftmost item will be equipped.
                        Note that you can use this command to equip something that is already equipped, which is not possible in game.
                        You can also equip unequippable items like materials, but it is not meaningful
			</p>
			<BodyText emphasized>
						Note that you cannot specify metadata. If you need to equip a specific item among others of the same type, use slot to specify which
			</BodyText>
			<p className="Reference Example">Example 1: Equip Weapon</p>
			<p className="Reference Example">Example 2: Equip Weapon In Slot 3</p>

			<h3 className="Reference">Unequip item</h3>
			<h3 className="Reference2">Unequip item In Slot X</h3>
			<h4 className="Reference">Simulates unequipping something</h4>
			<p className="Reference">
                        When unequipping an item, only the selected item is unequipped.
			</p>
			<p className="Reference">
                        Slot can be used if you have multiple of the same item. When slot is not specified, the leftmost equipped item will be unequipped.
                        Note that you can use this command to unequip something that is already unequipped, which is useless.
                        You cannot unequip arrows.
			</p>
			<BodyText emphasized>
						Note that you cannot specify metadata. If you need to equip a specific item among others of the same type, use slot to specify which
			</BodyText>
			<p className="Reference Example">Example 1: Unequip Shield</p>
			<p className="Reference Example">Example 2: Unequip Shield In Slot 5</p>

			<h3 className="Reference">Close Game</h3>
			<h4 className="Reference">Simulates closing the game and restarting</h4>
			<p className="Reference">
                        When closing the game, Visible Inventory and Game Data are erased
			</p>
			<p className="Reference Example">Example: Close Game</p>

			<SubHeader>Sync GameData</SubHeader>
			<SubTitle>Copy Visible Inventory to Game Data</SubTitle>
			<BodyText>
                        Certain actions in game will cause gamedata be synced with visible inventory, including but not limited to: open and close inventory, dpad quick menu, drop items.
			</BodyText>
			<BodyText>
                        Furthermore, if visible inventory has Count = 0, Game Data will be empty.
			</BodyText>
			<BodyText emphasized>Example: Sync GameData</BodyText>

			<SubHeader>Init GameData</SubHeader>
			<SubHeader connected>Init GameData X item1[meta] Y item2[meta] Z item3[meta] ...</SubHeader>
			<SubTitle>Used for forcing the game data to be desynced with visible inventory in simulation</SubTitle>
			<BodyText>
                        Similar to <Emphasized>Initialize</Emphasized>, this command sets up game data with items in the specified order. However, this command does not change visible inventory.
						This can be used to set up an initial state where Game Data is desynced.
			</BodyText>
			<BodyText emphasized>Example: Init GameData 1 Apple 2 Axe 3 Slate 4 SpiritOrb</BodyText>

			<h3 className="Reference">Shoot X Arrow</h3>
			<h4 className="Reference">Simulates shooting arrow without opening inventory</h4>
			<p className="Reference">
                        When reloading a save with desynced game data, the equipped weapon/bow/shield are automatically corrupted, but not the arrows.
                        To corrupt the equipped arrow slot, you need to shoot an arrow.
			</p>
			<p className="Reference">
                        This command does not let you select which arrow to shoot.
                        When you reload a save, Link should have the last equipped arrow slot equipped in the overworld.
				<span className="Example">[needs confirmation]</span>
			</p>
			<p className="Reference Example">Example: Shoot 1 Arrow</p>

			<h3 className="Reference">Enter/Exit Eventide</h3>
			<h3 className="Reference2">Enter/Exit TOTS</h3>
			<h4 className="Reference">Simulates entering/exiting Eventide or Trial of the Sword</h4>
			<p className="Reference">
                        When entering Eventide or TotS, the entire inventory is cleared except for key items regardless of inventory count.
                        While the challenge is active, none of the inventory changes are synced to game data.
			</p>
			<p className="Reference">
                        When exiting the challenge, the game reloads the game data as if reloading a save
			</p>
			<p className="Reference Example">Example: Enter Eventide</p>

			<SubHeader>Write [meta] To item</SubHeader>
			<SubHeader connected>Write [meta] To item In Slot X</SubHeader>
			<SubTitle>Change the metadata of item</SubTitle>
			<BodyText>
						This can be used to modify durability on weapon or count on items. This will sync inventory to game data.
			</BodyText>
			<BodyText>
						if you write 0 to life, it will not cause the slot to be removed
			</BodyText>
			<BodyText emphasized>Example: Write [life=1000] To pot lid</BodyText>
			<BodyText emphasized>Example: Write [life=1000] To pot lid In Slot 3</BodyText>

		</Page>
	);
});
