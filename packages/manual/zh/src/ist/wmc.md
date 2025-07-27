# Weapon Modifier Corruption (WMC)

A **Weapon Modifier** refers to the power-ups that can be found
on an equipment, such as `Attack Up` or `Durability Up`.
**Weapon Modifier Corruption** is a glitch enabled by IST
to transfer data from a cooked item to a weapon, interpreting the memory
as a Weapon Modifier.

During this corruption:
- The `Health Recover` value of the cooked item becomes the `Value` of the modifier.
  - For regular food, this value is the number of quarter-hearts recovered.
    Between 0 and 120.
  - For hearty food, this is the number of yellow hearts.
- The `Sell Price` value of the cooked item becomes the `Type` of the modifier.
  - The `Type` is a bit-flag, so WMC can enable multiple modifiers on the same equipment.
  - See [`actWeapon.h`](https://github.com/zeldaret/botw/blob/master/src/Game/Actor/actWeapon.h)
    for values for each modifier type.

```admonish info
You will see [Prompt Entanglement (PE)](./pe.md) often brought up together with WMC.
This is because WMC relies on the data from a cooked item, and you can only get
a very limited subset of possible cook data values by cooking with normal ingredients.
PE allows cooking with unusual ingredients, which is required to get some modifier value/flag.
However, WMC and PE are 2 separate glitches, and neither is required to perform the other.
```

## Base Mechanism
WMC is possible because:

1. The data for cooked item and the data for modifier is in the same memory
   location for each item. i.e. if the item is a food, then the data is used
   as cooked data, and if the item is a weapon, then it is used as modifier data.
   - This is only true for `Visible Inventory`, not `GameData`.
2. The data for cooked item is added separately from the item itself.

In step 2, the cook data is added after adding the cook item, and the game
assumes the *last added item* is the cook item that is supposed to receive
the data.

Therefore, a WMC setup typically involves:
1. Making sure the *last added item* is the weapon to receive the corrupted modifier
2. Make the cooked item that *donates* the data fail to load while
   the *last added item* is still the weapon, transferring the data to the weapon

Note that #2 essentially means that *no item should be loaded between the weapon
and the cook item.*

Depending on *how* these 2 conditions are satisfied, the WMC setups
can be further categorized.

```admonish info
In general, WMC can refer to any of the cases where the *last added item*
is not the item that is *supposed* to receive the data.

You can technically corrupt any item in `Visible Inventory`,
but only Equipments and Foods will have the data saved to `GameData`.

It is not possible to transfer modifier between Weapons, because unlike
food, the data for weapons are added in the same step as the item itself (condition 2 from above).
```

Currently, WMC is only possible when reloading the inventory from `GameData` (for example
when reloading a save). This is because there is no known way to trigger adding a cook item
during normal game play while at the same time making it so the cook item doesn't get added
successfully.

## Food Limit
Using the food limit was one of the first methods to WMC. This WMC setup
uses the fact that during a reload, any food after the first 60 will not load (under normal circumstances).
This can be achieved by transferring 60 food into a save with a weapon, followed by the donor meal.

Example script:
```skybook
get 1 hammer 1 wild-green[hp=120, price=113]
save
eat wild-green
get 60 dubious-food
!break 60 slots
reload
```

The step-by-step explanation:

1. Note that in the save, the items are 1 `hammer` and the donor meal: these are what will be added during reload.
2. Note that we are transferring 60 `dubious-food`.
3. During the reload, the `hammer` loads in first.
4. When loading the donor meal, since there are already 60 food, it fails to load.
5. Since the last-added item was the `hammer`, the data from the donor meal is transferred to the hammer.

```admonish info
The main drawback for this method is that it requires 60 food and 60 broken slots. Getting 60 broken slots
is very tedious.
```

## Stackable Food Limit
The reason why the previous setup required 60 broken slots, is because if we don't transfer 60 food,
then there will be food items that get loaded after the hammer. As a result, the cook data
will be transferred to that item, instead of the weapon.

But wait! *What if that's exactly what we wanted?* Instead of transferring the modifier directly to the weapon,
we can do 2 transfers:
1. Using the Food Limit method, transfer the data from the donor meal to a *Stackable* food (i.e. a roasted/baked/frozen food)
2. Using [Direct Inventory Corruption](./dic.md), corrupt the food value to >=500
3. When loading a stackable item slot, if the value being loaded - plus the value of the item in the inventory - is greater than 999,
   the stack will fail to load (this is why we need >=500 in the previous step)
4. When the stackable food fails to load, it can trigger WMC


Example script
```skybook
# Setup
get 1 hammer
!break 1 slots
get 58 dubious 1 roasted-endura-carrot 1 wild-green[hp=120, price=113]

# This will make only the last food fail to load, transferring the data to the carrot
save; reload  

# Clean up the foods now
eat wild-green; eat all dubious
save

# Corrupt the carrot stack
unequip hammer
eat roasted-endura-carrot
reload
save

# Finally transfer the data to the hammer
reload
```

```admonish info
As you can see, this method can work with a minimum of 1 broken slots, which is a big improvements over 60 broken slots.
In speedruns, typically we use more broken slots anyway to corrupt other stuff and for duplicating food to quickly
reach the 60 food required for the first transfer.
```

## The Nullptr Exploit
Both previous setups require 60 food to make the first transfer happen. It would be really nice if that's not the case.
This is where the Nullptr Exploit comes in.

Recall that after adding the cook item (or fails to add), the game applies the cook data to the *last added item*
without checking if it's the cook item. However, when *there is no last added item*, the data is not added, and crucially,
the game does not advance to the next cook data slot. This means when the next food loads, it will keep using the cook
data of the previous food!

```admonish info
This can happen because in the `GameData`, the cook data and the item themselves are not stored in the same array:

- The item name array has 420 strings, one for each item.
- The cook data array has 60 elements, one for each food.

When the Nullptr Exploit is triggered, the counter for the item array increments, but not the cook data array.
```

Using this exploit, 60 food is no longer required. However, this exploit is pretty tricky to trigger;
this is because `mLastAddedItem` is only `nullptr` before any item is added (with one exception being the Master Sword).

## Master Sword (MSWMC)

This setup triggers the Nullptr Exploit using the fact that Master Sword cannot be duplicated.

The condition for this trigger is somewhat complicated:
1. A Master Sword is transferred (can be broken or not broken)
2. The save being loaded doesn't have Master Sword being the first item
   - This is because adding the first item skips the part check we rely on to trigger the exploit
3. The Master Sword Recharge Timer in the save being loaded is non-zero
4. The save being loaded has a Master Sword (either broken or not broken)
   - 3 and 4 usually means the save has a broken Master Sword

```admonish tip
In one sentence, this means to "transfer a Master Sword into a save with a broken Master Sword".
```

When all of the above conditions are met, the last-added item is set to `nullptr`, enabling the exploit.

The whole setup would be:
1. Enable the exploit as described above
2. In the same reload, load a food unsuccessfully, without loading any item in between
   - This uses the exploit described in the previous section to cause the reuse of cook data
3. Now any food that loads after will have the data that's supposed to be on the food before it
4. Continue the Stackable Food Limit setup

## Travel Medallion (TMWMC)

```admonish todo
This section is WIP and may contain inaccurate information. If you see any issues,
or want to improve this section, please create a Pull Request.
```

The Travel Medallion is added in the Master Trials DLC. Unlike any other DLC items, it gets removed
if you uninstall the DLC.

This can be used to trigger the Nullptr Exploit:
1. Make a save with Travel Medallion being the first item, and food after it 
   (using [Unsorted Inventory](./dic.md#unsorted-inventory-and-forward-corruption) )
2. Close the game and uninstall the DLC
   - If you have physical game without DLC + digital game with DLC, this can be done by ejecting the Virtual Game Card
   - Otherwise, follow the steps on this [Nintendo Support Article](https://en-americas-support.nintendo.com/app/answers/detail/a_id/22433/~/how-to-redownload-nintendo-switch%26nbsp%3B2-or-nintendo-switch-digital-content)
3. Reload the save, setup IST again to transfer some food to make the food fail to load
4. Reload the save again, the Travel Medallion will not load as the first item since DLC is uninstalled, and the next food will fail to load because of the transfer
5. Now any food that loads after will have the data that's supposed to be on the food before it
6. Continue the Stackable Food Limit setup

Unlike MSWMC, Travel Medallion does not set last added item to `nullptr`, so it has very limited usefulness.


## Zelda Notes (ZNWMC)
```admonish todo
This section is WIP and may contain inaccurate information. If you see any issues,
or want to improve this section, please create a Pull Request.
```

```admonish warning
This method has not be verified. It is only theorized based on reverse engineering of version 1.8.0 of the Switch 1 Edition.
```

Zelda Notes is an item exclusive to the Nintendo Switch 2 Edition. When you uninstall NS2E,
it gets removed from your inventory and can be used to trigger the Nullptr Exploit:
1. Make a save with the Daily Bonus and Deposit Item being the first 2 items, and food after it
   (using [Unsorted Inventory](./dic.md#unsorted-inventory-and-forward-corruption) )
2. Close the game and uninstall NS2E
   - If you have the physical NS2E Game Card and the NS1E game (physical or digital), this can be done by removing the physical NS2E Game Card (then inserting the NS1E Game Card if you have a physical copy)
   - If you only have the physical NS2E Game Card, this is not possible.
   - Otherwise, follow the steps on this [Nintendo Support Article](https://en-americas-support.nintendo.com/app/answers/detail/a_id/22433/~/how-to-redownload-nintendo-switch%26nbsp%3B2-or-nintendo-switch-digital-content)
     to download the game without NS2E
3. The rest is similar to TMWMC
