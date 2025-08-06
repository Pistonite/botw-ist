# 过载

在游戏中，生成了超过物件系统(Actor System)上限的物件后，再生成物件会失败。此状态称为过载。

由于模拟器中没有像游戏一样的物件系统，仅提供指令触发和取消模拟的过载状态。

- <skyb>overload</skyb> 用于触发过载
- <skyb>unoverload</skyb> 用于取消过载

## 语法
> `overload`<br>
> `unoverload`<br>

## 物品强持
若关闭背包时处于过载状态，则手持物品不会生成在主世界，称之为物品强持。

```skybook
# 随便拿一些物品
get 2 apple 2 banana 2 core
# 触发过载
overload
# 手持并退出背包
hold 1 apple 1 banana 1 core
unpause
# 此时处于强持状态，所以可以正常拿物品
get diamond
```

## 物品置换
物品强持可用于物品置换（把物品A变成物品B)。

```skybook
# 拿需要牺牲的材料并强持5个
get 6 apple; overload hold 5 apple
# 卖掉手持的格子
sell apple
# 拿需要置换的物品
get giant-ancient-core
# 取消手持完成置换
unhold
```

## 制作转存格
物品强持可用于制作转存格，见[强持法制作转存格](./break_slots.md#强持法)。

## 耐久继承
触发过载可以在不切换主世界装备的情况下切换背包装备，从而导致主世界和背包装备不同步，可以用于同类物品继承耐久。

```skybook
# 把斧头耐久继承到近卫
get axe royal-guard-claymore
overload
equip royal-guard-claymore
unoverload
use weapon
```

## 细节
- <skyb>overload</skyb>可以在任何界面执行。
- <skyb>unoverload</skyb>取消过载需要[主世界](../user/screen_system.md)界面。
  - 这是因为在游戏中打开背包后是没有办法取消过载的。

