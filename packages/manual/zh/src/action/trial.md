# 挑战

游戏中一些任务触发时会清空背包或提供临时道具，称之为挑战。只有挑战结束后，才能拿回原本的道具。

挑战包括：
- 剑之试炼
- 孤岛试炼（赛哈特诺岛）
- 英杰之诗的四个Boss战

每种挑战都有各自的事件流，比较复杂，所以模拟器仅提供<skyb>!trial-start</skyb>和<skyb>!trial-end</skyb>指令来触发和解除背包系统的挑战模式。

## 语法

> `!trial-start` <br>
> `!trial-end`

- 开始挑战时：
  - 所有除重要道具外的道具会被删除。
  - 主世界武器会重置。
- 结束挑战时，背包的挑战模式会解除，并且GDT中物品会重新载入背包。
  - 大多数情况下，使用<skyb>reload</skyb>指令读档也会自动解除挑战，无需其他操作。

大多数情况下，仅使用<skyb>!trial-start</skyb>或<skyb>!trial-end</skyb>就足以模拟游戏中的情况。但有些边界情况可能需要模拟事件流，请参考下面。

```admonish danger
大多数情况下，开始挑战前，请确保模拟器界面处于主世界。在背包界面情况下开启挑战虽然支持，但是可能会有奇怪的结果。模拟器执行此指令时不会自动切换界面。
```

## 孤岛试炼
登上赛哈特诺岛时，手持的物品会自动放回背包。若无箭强持，则会掉落。请手动确保无手持物品。

```skybook
# 确保处于主世界
unpause
# 开始挑战
!trial-start
```

放弃挑战离开岛时，会有一个加载界面。但完成挑战时不会有。
```skybook
!trial-end
!system [loading-screen]
```

## 剑之试炼
剑之试炼更复杂:
```skybook
# 确保无手持（可以强持）
unhold
# 剑试的事件流会自动装备大师剑，删除大师剑，然后再还给你
# 注意这里的指令并不能完美模拟事件流
equip master-sword; unpause; !remove master-sword; get master-sword
# 进入试炼模式
!trial-start
# 进入试炼时，会有一个加载界面传送到剑试地图
!system [loading-screen]
```

离开剑试：
```skybook
# 解除试炼模式，有一个加载界面返回主世界
!trial-end
!system [loading-screen]
# 游戏自动重新给你大师剑
get master-sword
# 如果三阶全通，还需要设定开光的Flag
!set-gdt <Open_MasterSword_FullPower>[bool=true]
```
