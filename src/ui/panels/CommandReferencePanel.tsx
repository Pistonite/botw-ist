import React from "react";
import { Category, Description, ItemSlot, Label, ParseCode } from "ui/components";
import { Page } from "ui/surfaces";

export const ReferencePage: React.FC = React.memo(()=>{

	return (
		<Page title="Command Reference">
			<Description />
			<Label>This page will help you with writing and understanding commands</Label>
			<Category title="Syntax for Items">
				<Label>
					By
					<span className="Highlight"> name</span>
					: <code>apple = </code>
				</Label>
				<ItemSlot slot={{
					image: "assets/img/Material/Apple.png",
					count: 1,
					durability: undefined,
					isEquipped: false,
					isBrokenSlot: false,
					getTooltip: ()=>[]
				}} />
				<Label><code>royal claym = </code></Label>
				<ItemSlot slot={{
					image: "assets/img/Weapon/RoyalClaymore.png",
					count: undefined,
					durability: "40",
					isEquipped: false,
					isBrokenSlot: false,
					getTooltip: ()=>[]
				}} />
				<Description className="Primary">
					You can get an item by putting the name of the item or parts of the name.
					<span className="Important"> The names are case-insensitive.</span>
				</Description>
				<Description>
					You can also specify the name without spaces like <code>royalclaymore</code>.
					However, space-separated names are easier to read.
				</Description>
				<Label>
					By
					<span className="Highlight"> amount + name</span>
					: <code>3 apples = </code>
				</Label>
				<ItemSlot slot={{
					image: "assets/img/Material/Apple.png",
					count: 3,
					durability: undefined,
					isEquipped: false,
					isBrokenSlot: false,
					getTooltip: ()=>[]
				}} />
				<Label><code>3 royal claym = </code></Label>
				<ItemSlot slot={{
					image: "assets/img/Weapon/RoyalClaymore.png",
					count: undefined,
					durability: "40",
					isEquipped: false,
					isBrokenSlot: false,
					getTooltip: ()=>[]
				}} />
				<ItemSlot slot={{
					image: "assets/img/Weapon/RoyalClaymore.png",
					count: undefined,
					durability: "40",
					isEquipped: false,
					isBrokenSlot: false,
					getTooltip: ()=>[]
				}} />
				<ItemSlot slot={{
					image: "assets/img/Weapon/RoyalClaymore.png",
					count: undefined,
					durability: "40",
					isEquipped: false,
					isBrokenSlot: false,
					getTooltip: ()=>[]
				}} />
				<Description className="Primary">
					By adding a number before the item, you can specify how many of that item you want.
				</Description>
				<Description className="Primary">
					If the item is unstackble, the amount usually translates to how many slots of the item you want.
					For some commands, you can specify <span className="Highlight"> "all"</span> in the place of the amount.
				</Description>
				<Description className="Important">
					Plurals are accepted.
				</Description>
				<Label>
					By
					<span className="Highlight"> name[metadata]</span>
					: <code>elixir[modifier=speed] = </code>
				</Label>
				<ItemSlot slot={{
					image: "assets/img/Food/HastyElixir.png",
					count: undefined,
					durability: undefined,
					isEquipped: false,
					isBrokenSlot: false,
					modifierImage: "assets/img/Modifiers/CookHasty.png",
					getTooltip: ()=>[]
				}} />
				<Label><code>royal claym[equip, life:80000] = </code></Label>
				<ItemSlot slot={{
					image: "assets/img/Weapon/RoyalClaymore.png",
					count: undefined,
					durability: "800",
					isEquipped: true,
					isBrokenSlot: false,
					getTooltip: ()=>[]
				}} />
				<Description className="Primary">
					You can put additional property on the item by putting them in <code className="Highlight"> [ ] </code>
					after the item name.
				</Description>
				<Description className="Primary">
					Inside the bracket, the metadata values are specified like <code className="Important">key=value</code> or <code className="Important">key:value</code>, separated by commas.
				</Description>
				<Description className="Primary">
					The value can be an integer, true/false, or text depending on the key. If you specify a key without a value, the value is default to true (like <code>[equip]</code>)
				</Description>
				<Description useDiv>
					List of metadata keys:
					<ul>
						<li><code className="Important">life=integer</code>: For equipments, durability*100, for others, stack size</li>
						<li><code className="Important">equip=true/false</code>: Whether the slot is equipped</li>
						<li><code className="Important">hp=integer</code>: For equipments, the modifier value. For food, how many quarter hearts it recovers. (4 = 1 heart)</li>
						<li><code className="Important">price=integer</code>: For equipments, the corrupted modifier. For food, the price when you sell</li>
						<li><code className="Important">modifier=text</code>: Either a food effect (like <code>mighty</code>) or a single weapon modifier name (like <code>durability</code>). You don't need to put the full modifier name to match one</li>
					</ul>
				</Description>
				<Label>
					You can also mix them like
					<span className="Highlight"> amount + name[metadata]</span>
				</Label>
			</Category>
			<Category title="Common Commands">
				<Label className="Important">All commands are case-insensitive</Label>
				<Description />

				<div>
					<code className="CommandColorKeywordCommand">init </code>
					<code className="CommandColorItemName">[items ...]</code>
				</div>

				<ParseCode>
					init apple
				</ParseCode>
				<ParseCode>
					init 1 apple
				</ParseCode>
				<ParseCode>
					init 3 pot lid 5 diamonds 1 slate 1 glider 4 orb
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Initialize inventory before simulation</Label>

				<Description className="Primary">
					Fully resets the inventory by clearing all items and set Count to 0, then forcefully write the item list to inventory.
                    This would reset any broken slot you already have, and any in-game checks that happen when adding items are disabled.
                    For example, the items will appear in the order you specify, not in the in-game tab order
				</Description>
				<Description className="Primary Important">
					Game Data will be synced with Visible Inventory after the reset
				</Description>
				<Description>
                    Note that this will not clear saves. You can use this command to initialize multiple saves
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">break </code>
					<code className="CommandColorSlotNumber">X </code>
					<code className="CommandColorKeywordCommand">slot(s) </code>
					<code className="CommandColorKeywordOther">[with </code>
					<code className="CommandColorItemName">items ... </code>
					<code className="CommandColorKeywordOther">[from slot </code>
					<code className="CommandColorSlotNumber">Y</code>
					<code className="CommandColorKeywordOther">]] </code>
				</div>

				<ParseCode>
					break 1 slot
				</ParseCode>
				<ParseCode>
					break 5 slots with 2 apple 2 pepper 2 durian 2 lotus 2 shroom
				</ParseCode>
				<ParseCode>
					break 1 slots with apple from slot 3
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Simulate making X broken slots with the glitch</Label>

				<Description className="Primary">
					If you add <code className="CommandColorKeywordOther"> with</code>, the items after <code className="CommandColorKeywordOther"> with</code> are automatically removed.
				</Description>
				<Description className="Primary">
					If you add <code className="CommandColorKeywordOther"> from slot</code>. The items are removed from the n-th matched slot. In the 3rd example above, the apple is removed from the 3rd slot of apples
				</Description>
				<Description className="Highlight">
					<code className="CommandColorKeywordOther"> from slot</code> or <code className="CommandColorKeywordOther"> in slot</code> or <code className="CommandColorKeywordOther"> to slot</code> appear
					in many commands and they behave the same way.
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">get|add|buy|pickup </code>
					<code className="CommandColorItemName">items ...</code>
				</div>

				<ParseCode>
					get apple
				</ParseCode>
				<ParseCode>
					add 3 pot lid 5 diamonds 1 slate 1 glider 4 orb
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Add items to inventory</Label>

				<Description className="Primary">
					Add the item(s) to visible inventory. Sync with Game Data unless you are on Eventide or inside TOTS
				</Description>
				<Description className="Primary">
					Like in game, you won't be able to obtain multiple unstackable key items, or multiple master sword in this way.
					If a stackable item is at 999 or more when you invoke this command, the count is set to 999 (not fully accurate since you won't be able to pick up more items in game).
				</Description>
				<Description>
					If you specify a count for unstackable items, they are added in different slots as if you pick them up in game, one after another.
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">remove|sell|eat|drop </code>
					<code className="CommandColorItemName">items ... </code>
					<code className="CommandColorKeywordOther">[from slot </code>
					<code className="CommandColorSlotNumber">X</code>
					<code className="CommandColorKeywordOther">] </code>
				</div>

				<ParseCode>
					remove apple
				</ParseCode>
				<ParseCode>
					sell 5 diamonds from slot 3
				</ParseCode>
				<ParseCode>
					remove all diamonds
				</ParseCode>
				<ParseCode>
					eat all normal arrows
				</ParseCode>
				<ParseCode>
					drop axe[equip=true]
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Simulate removing items in game</Label>

				<Description className="Primary">
					Remove the item(s) from visible inventory. Sync with Game Data unless you are on Eventide or inside TOTS
				</Description>
				<Description className="Primary">
					When removing items, the simulator will try to match the stack exactly first, including metadata.
					In the 4th example, the equipped axe will be dropped instead of the left most one.
				</Description>
				<Description className="Important">
					Eat: only 1 of the corrupted food will be eaten, and can be used to remove empty arrow slots
				</Description>
				<div>
					<code className="CommandColorKeywordCommand">remove|drop|sell|with all</code>
					<code className="CommandColorIdentifierOther"> type</code>
				</div>

				<ParseCode>
					remove all materials
				</ParseCode>
				<ParseCode>
					remove all key items
				</ParseCode>
				<ParseCode>
					drop all shield
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Easy way to get rid of everything</Label>
				<Description >
					Remove the all items of a type from visible inventory. Sync with Game Data unless you are on Eventide or inside TOTS
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">save </code>
					<code className="CommandColorKeywordCommand">[as </code>
					<code className="CommandColorIdentifierOther">file name ...</code>
					<code className="CommandColorKeywordCommand">] </code>
				</div>

				<ParseCode>
					save
				</ParseCode>
				<ParseCode>
					save as auto save at shop
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Simulates a hard save or auto save</Label>

				<Description className="Primary">
					Writes Game Data to the corresponding save slot. The auto saves are specified by <code className="CommandColorIdentifierOther">file name</code>.
                    You can have as many auto saves as you want in the simulator. If no file is specified, it will save to the manual save slot.
				</Description>
				<Description className="Important">
					The save file name is case-insensitive
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">reload </code>
					<code className="CommandColorIdentifierOther">[file name ...]</code>
				</div>

				<ParseCode>
					reload
				</ParseCode>
				<ParseCode>
					reload auto save at shop
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Simulates reloading a save</Label>

				<Description className="Primary">
					First, reads Game Data from the corresponding save slot (manual or specified by <code className="CommandColorIdentifierOther">file name</code>)
				</Description>
				<Description className="Primary">
					After that, the first mCount items in the visible inventory is removed, and mCount is decreased accordingly.
                    Then, each item slot in the Game Data is added to the inventory.
				</Description>
				<Description className="Important">
					The save file name is case-insensitive
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">equip </code>
					<code className="CommandColorItemName">item name </code>
					<code className="CommandColorKeywordOther">[in slot </code>
					<code className="CommandColorSlotNumber">X</code>
					<code className="CommandColorKeywordOther">] </code>
				</div>

				<ParseCode>
					equip royal claymore
				</ParseCode>
				<ParseCode>
					equip potlid in slot 3
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Simulates equipping something</Label>

				<Description className="Primary">
					When equipping an item, all other item of the same type in the first tab is unequipped, then the item selected is equipped.
				</Description>
				<Description >
					Slot can be used if you have multiple of the same item. When slot is not specified, the leftmost item will be equipped.
                    Note that you can use this command to equip something that is already equipped, which is not possible in game.
                    You can also equip unequippable items like materials, but it is not meaningful
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">unequip </code>
					<code className="CommandColorItemName">item name </code>
					<code className="CommandColorKeywordOther">[in slot </code>
					<code className="CommandColorSlotNumber">X</code>
					<code className="CommandColorKeywordOther">] </code>
				</div>
				<div>
					<code className="CommandColorKeywordCommand">unequip </code>
					<code className="CommandColorKeywordCommand">[all] </code>
					<code className="CommandColorIdentifierOther">type</code>
				</div>

				<ParseCode>
					unequip royal claymore
				</ParseCode>
				<ParseCode>
					unequip potlid in slot 3
				</ParseCode>
				<ParseCode>
					unequip weapon
				</ParseCode>
				<ParseCode>
					unequip all shields
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Simulates unequipping something</Label>

				<Description className="Primary">
					When unequipping an item, only the selected item is unequipped, not the other equipped items of the same type.
				</Description>
				<Description className="Primary">
					Slot can be used if you have multiple of the same item. When slot is not specified, the leftmost equipped item will be unequipped.
                    Note that you can use this command to unequip something that is already unequipped, which is useless.
				</Description>
				<Description className="Important">
					You cannot unequip arrows. However, you can use the <code className="CommandColorKeywordCommand">write</code> command to do that.
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">shoot </code>
					<code className="CommandColorItemAmount">X </code>
					<code className="CommandColorKeywordCommand">arrow(s) </code>
				</div>

				<ParseCode>
					shoot 1 arrow
				</ParseCode>
				<ParseCode>
					shoot 2 arrows
				</ParseCode>
				<ParseCode>
					shoot all arrows
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Simulates shooting arrow without opening inventory</Label>

				<Description className="Primary">
					When reloading a save with desynced game data, the equipped weapon/bow/shield are automatically corrupted, but not the arrows.
                    To corrupt the equipped arrow slot, you need to shoot an arrow.
				</Description>
				<Description className="Primary">
					Note that equipped arrow can only be found in the first arrow tab. If there is no equipped arrow before the first shield/armor/material/food/key item, the game cannot find the equipped arrow and will have no arrow equipped
				</Description>
				<Description className="Important">
					This command does not let you select which arrow to shoot.
                    When you reload a save, Link should have the last equipped arrow slot equipped in the overworld.
					<span className="Highlight">[needs confirmation]</span>
				</Description>

				<ParseCode>
					close game
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Simulates closing the game and restarting</Label>

				<Description>
					This command does exactly what you think it does.
				</Description>

				<ParseCode>
					# comment
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Add comments to make your steps easier to understand</Label>
				<Description />

			</Category>
			<Category title="Advanced Commands">
				<code className="CommandColorKeywordCommand">dnp </code>
				<code className="CommandColorItemName">items ... </code>
				<code className="CommandColorKeywordOther">[from slot </code>
				<code className="CommandColorSlotNumber">X</code>
				<code className="CommandColorKeywordOther">] </code>

				<ParseCode>
					dnp 5 lizalfos tails 5 farosh horns
				</ParseCode>
				<ParseCode>
					dnp apple from slot 2
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Shortcut for drop and pick up, for sorting inventory</Label>

				<Description>
					This command drops and pick up each item stack in the specified order.
                    You can also repeat items if you are combining more than 2 slots.
				</Description>

				<ParseCode>
					sync gamedata
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Copy Visible Inventory to Game Data</Label>
				<Description className="Primary">
					Certain actions in game will cause gamedata be synced with visible inventory, including but not limited to: open and close inventory, dpad quick menu, drop items.
				</Description>
				<Description className="Important">
					Furthermore, if visible inventory has Count = 0, Game Data will be empty.
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">init gamedata </code>
					<code className="CommandColorItemName">[items ...]</code>
				</div>

				<ParseCode>
					init gamedata apple
				</ParseCode>
				<ParseCode>
					init gamedata 3 pot lid 5 diamonds 1 slate 1 glider 4 orb
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Initialize Game Data</Label>

				<Description>
					Like init, but only changes Game Data, not Visible Inventory. Use this to force Game Data to desync.
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">enter|exit|leave eventide|tots </code>
				</div>

				<ParseCode>
					enter eventide
				</ParseCode>
				<ParseCode>
					leave eventide
				</ParseCode>
				<ParseCode>
					enter tots
				</ParseCode>
				<ParseCode>
					exit tots
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Simulates entering/exiting Eventide or Trial of the Sword</Label>

				<Description className="Primary">
					When entering Eventide or TotS, the entire inventory is cleared except for key items regardless of inventory count.
                    While the challenge is active, none of the inventory changes are synced to game data.
				</Description>
				<Description>
					When exiting the challenge, the game reloads the game data as if reloading a save
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">write </code>
					<code className="CommandColorMetaKey">meta </code>
					<code className="CommandColorKeywordCommand">to </code>
					<code className="CommandColorItemName">item </code>
					<code className="CommandColorKeywordOther">[in slot </code>
					<code className="CommandColorSlotNumber">X</code>
					<code className="CommandColorKeywordOther">]</code>
				</div>

				<ParseCode>
					write [life=700] to potlid
				</ParseCode>
				<ParseCode>
					write [life=700] to potlid in slot 2
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Change the metadata of item</Label>

				<Description className="Primary">
					This can be used to modify durability on weapon or count on items. This will NOT sync inventory to game data.
				</Description>
				<Description className="Important">
					if you write 0 to life, it will not cause the slot to be removed
				</Description>

				<div>
					<code className="CommandColorKeywordOther">has </code>
					<code className="CommandColorKeywordOther">[not] </code>
					<code className="CommandColorMetaValue">value </code>
					<code className="CommandColorMetaKey">flag name ... </code>

				</div>

				<ParseCode>
					has 10 weapon slots
				</ParseCode>
				<ParseCode>
					has 20 shield slots
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Change game flags</Label>

				<Description className="Primary">
					This can be used to modify state that are not part of the inventory. For example, how many slots you have. The <code className="CommandColorMetaKey">flag name ... </code> part is concatenated and matched by prefix.
				</Description>
				<Description className="Primary">
					If the flag is a boolean, the value will be ignored and you will always get <code>true</code>. You can get <code>false</code> by adding <code className="CommandColorKeywordOther">not</code>.
				</Description>
				<Description className="Primary Error">
					Flag system is new so it's very unstable. Please report inconsistencies with discord DM or on github.
				</Description>
				<Description useDiv>
					List of flag names:
					<ul>
						<li><code className="Important">weaponSlots=integer</code>: Number of weapon slots</li>
						<li><code className="Important">bowSlots=integer</code>: Number of bow slots</li>
						<li><code className="Important">shieldSlots=integer</code>: Number of shield slots</li>
					</ul>
				</Description>

				<div>
					<code className="CommandColorKeywordCommand">cook [heart crit] with </code>
					<code className="CommandColorItemName">items ... </code>
					<code className="CommandColorKeywordOther">[from slot </code>
					<code className="CommandColorSlotNumber">X</code>
					<code className="CommandColorKeywordOther">]</code>
				</div>

				<ParseCode>
					cook with 1 farosh horn 3 lotus 1 swift carrot
				</ParseCode>
				<ParseCode>
					cook heart crit with apple from slot 3
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Simulate cooking</Label>
				<Description className="Primary Error">
					This command will be available in a future update
				</Description>

				<Description className="Primary">
					Removes the list of items and add the cook result to inventory. If you specify more than 5 items, all of them will be removed, but only the first 5 will be used in cooking.
				</Description>
				<Description className="Primary">
					If <code>heart crit</code> is specified, will simulate a critical cooking on hearts.
				</Description>
				<Description className="Error">
					You cannot cook in game if you have 60 food. This is NOT enforced in the simulator.
				</Description>

			</Category>
			<Category title="Super Commands">
				<Description className="Error">
					This feature will be available in a future update
				</Description>
				{/* <Description className="Important">
					Slot indices in commands below are all 0-based
				</Description>

				<div>
					<code className="CommandColorKeywordSuper">!swap </code>
					<code className="CommandColorSlotNumber">i j</code>
				</div>

				<ParseCode>
					!swap 3 5
				</ParseCode>
				<Description className="Secondary"/>
				<Label>Swap 2 slots</Label>
				<Description>
					Swap the i-th and j-th slot. Does not sync GameData.
				</Description> */}
			</Category>

		</Page>
	);
});
