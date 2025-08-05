# 主世界系统

The `Overworld` system simulates objects that the player interacts in the overworld,
known as `Actor`s. However, the actual overworld in the game is very complex, and
most of the actors don't even have anything to do with the inventory.
Therefore, the `Overworld` system is a ultra-simplified simulation of only
the actors that are involved in inventory glitches:

- The player's equipment (Weapon, Bow and Shield)
- Any items currently being held by the player in the overworld
- Any items (including materials and equipments) dropped by the player

## 丢弃物品上限
In the game, you can drop at most `10` items on the ground at a time.
When you drop the `11`-th item, the least-recent dropped item will despawn.
This limit is simulated by the `Overworld` system in the following way:

- When dropping material with the <skyb>drop</skyb> command, or auto-dropped
  from a smuggled state, it gets added to the list of items on the ground
- The least-recently dropped items will be removed from the list, until
  there are at most 10 items on the ground
- The removed items are not deleted immediately. You will see `Will despawn`
  in the tooltip text of the item in the simulator UI.
- If you perform any action that takes some time so it's impossible to preserve
  the despawning item, the item will be deleted.

```admonish tip
It is implemented like this because it is possible to drop more than 10 items,
but pick up the items fast enough before it despawns to keep the materials on the ground
under the limit. This can be used to optimize IST steps.
```

For example, the following script will result in `15` apples in the overworld,
`5` of which are in the `Will despawn` state.
```skybook
hold 5 apples; drop
hold 5 apples; drop
hold 5 apples; drop
```

Then:
- If you <skyb>pick-up 5 apples</skyb> right after, there will be `10` apples
  left on the ground, and `5` are added to the inventory.
- If you <skyb>pause</skyb>, there will still be `15` apples on the ground,
  since you could <skyb>unpause</skyb> and pick them up.
- If you <skyb>get 3 bananas</skyb>, the despawning items will now be deleted,
  and there will be `10` apples left on the ground. This is because it's unlikely
  the apples are still there after you pick up some other item.

## 重置主世界

很长的IST步骤可能包含了游戏中多个区域，或是进入、离开神庙。主世界中的物品会因玩家离开区域而消失。在模拟器中，可以通过一些方式模拟：

- 正常会重置主世界的指令会自动重置，比如<skyb>reload</skyb>读档。
    - 可用<skyb>!system [loading-screen]</skyb>系统指令模拟触发加载界面。
- <skyb>!system [clear-ground]</skyb>指令可以清除地上的物品，比如在离开某区域时。
