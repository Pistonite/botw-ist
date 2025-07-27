# Overworld Operations

Things you can do in the overworld:

- <skyb>use</skyb> command uses and decreases durability of equipments,
  or can be used to remove items while in the overworld, e.g. <skyb>use fairy</skyb>.
- <skyb>shoot</skyb> is an alias of <skyb>use bow</skyb>.
- <skyb>:overworld drop</skyb> command drops equipped equipments.

## Syntax
> `use CATEGORY_OR_ITEM` (defaults to 1 time) <br>
> `use CATEGORY_OR_ITEM X times` <br>
> `shoot` <br>
> `shoot X times` <br>
> `:overworld drop` [`CONTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>

Annotations:
  - [`:per-use X`](#using-equipments) - sets the value to decrease per use
  - `:overworld` - changes the semantic of <skyb>drop</skyb>

## Using Equipments

To <skyb>use</skyb> an equipped weapon, you can either specify the category,
or the item name (given the item is equipped).

```skybook
# Use the currently-equipped weapon to hit something
use weapon
# Use the currently-equipped weapon to hit something 5 times
use weapon 5 times
# Shoot with Royal Bow. Royal Bow must be the currently equipped bow
use royal-bow
# Shoot with currently equipped bow
shoot
```

The <skyb>:per-use</skyb> annotation changes how much durability is consumed
per use. The default is `100`.

```skybook
# Bombing the shield takes 30 durability off
:per-use 3000 use shield
```

Special cases:
  - Using a weapon with `IsLifeInfinite=true` will not decrease durability
  - Using Bow of Light/Twilight Bow will not decrease arrows
  - Using MasterSword while the GDT `Open_MasterSword_FullPower=true` will
    consume `0.2x` specified value, if its value is currently `>=300`

## Using Non-equipments

When the item specified for <skyb>use</skyb> is not an equipment,
it will attempt to remove the item instead. The only legitimate use
of this is <skyb>use fairy</skyb>. However, the simulator will permit
any item to be used this way.

For example, create Broken Slot with fairy:
```skybook
hold fairy; use fairy; drop
```

## Dropping the Overworld Equipment

```admonish todo
This command is WIP.
```
In some cases, you can drop the equipment in the overworld without interacting
with the inventory; for example, when getting shocked.

TODO

## Detail

- <skyb>use</skyb> requires [`Overworld`](../user/screen_system.md).
- In [Arrowless Smuggle](./material.md#smuggle-state-for-arrowless-offset) state,
  using a <skyb>weapon</skyb> or <skyb>shield</skyb> will <skyb>unhold</skyb>
  the item, while using a <skyb>bow</skyb> will <skyb>drop</skyb> the items.
