# 指令参考

以下是按字母顺序排序的所有指令。点击指令查看详细说明。

<div class="skybook--wide-table">

| 指令 | 说明 |
|-|-|
| <skyb>:accurately-simulate</skyb><br> 可用于 ([<skyb>get</skyb>](../action/get.md#性能优化), [<skyb>sort</skyb>](../action/sort.md#性能优化) ) | 禁用可能不精确的优化 |
| [<skyb>!arrowless-smuggle</skyb>](../action/material.md#无箭强持) | 用当前手持的物品执行无箭强持（可用于<skyb>:smug</skyb>不适用的场景）|
| [<skyb>!add-slot</skyb>](../action/low_level.md) | 修改内存，绕过所有检查添加物品 |
| [<skyb>!break</skyb>](../action/low_level.md) | 修改内存制作转存格 |
| [<skyb>buy</skyb>](../action/get.md) | 买东西 |
| <skyb>close-dialog</skyb> | 同 <skyb>untalk</skyb> |
| <skyb>close-inv</skyb> | 同 <skyb>unpause</skyb> |
| <skyb>close-inventory</skyb> | 同 <skyb>unpause</skyb> |
| [<skyb>close-game</skyb>](../action/save.md) | 关闭游戏 |
| [<skyb>:discovered</skyb>](../action/flags.md#修改页面是否解锁) | 修改页面是否已经解锁 |
| <skyb>dnp</skyb> | 丢弃材料或装备，然后捡起 |
| [<skyb>:dpad</skyb>](../action/equip.md) | 指定切换装备由十字界面执行 |
| <skyb>drop</skyb> | 丢弃材料或装备 |
| [<skyb>eat</skyb>](../action/material.md) | 吃东西|
| [<skyb>entangle</skyb>](../action/entangle.md) | 触发选项纠缠 |
| [<skyb>equip</skyb>](../action/equip.md) | 装备物品 |
| [<skyb>get</skyb>](../action/get.md) | 拿新物品 |
| [<skyb>hold</skyb>](../action/material.md) | 手持材料 |
| [<skyb>!init</skyb>](../action/low_level.md) | 初始化背包内存为某些物品 |
| [<skyb>new-game</skyb>](../action/save.md) | 开始新游戏 |
| <skyb>open-inv</skyb> | 同 <skyb>pause</skyb> |
| <skyb>open-inventory</skyb> | 同 <skyb>pause</skyb> |
| [<skyb>overload</skyb>](../action/overload.md) | 触发过载 |
| <skyb>:overworld</skyb> | 指定下一个指令在主世界中执行 |
| <skyb>pause</skyb> | 打开背包 |
| [<skyb>:pause-during</skyb>](../action/get.md#新物品提示时打开背包) | 某些指令执行时，中途打开背包 |
| [<skyb>:per-use</skyb>](../action/overworld.md) | 指定下一个<skyb>use</skyb>指令消耗的耐久 |
| [<skyb>pick-up</skyb>](../action/get.md) | 从地上捡起物品 |
| [<skyb>!remove</skyb>](../action/low_level.md) | 强制删除物品，无法交互的物品也能删除 |
| [<skyb>reload</skyb>](../action/save.md) | 加载手动或命名档 |
| <skyb>:same-dialog</skyb> <br>(可用于 [<skyb>buy</skyb>](../action/get.md#从NPC处买东西), [<skyb>sort</skyb>](../action/sort.md#在出售界面排序) ) | 指定下一个操作在同一个事件对话中执行 |
| [<skyb>save</skyb>](../action/save.md) | 存手动档 |
| [<skyb>save-as</skyb>](../action/save.md) | 存档并命名存档 |
| [<skyb>!set-gdt</skyb>](../action/flags.md#修改任意旗标) | 修改GDT旗标 |
| [<skyb>:slot</skyb>](../action/flags.md#修改装备格解锁数-呀哈哈升级) | 同 <skyb>:slots</skyb> |
| [<skyb>:slots</skyb>](../action/flags.md#修改装备格解锁数-呀哈哈升级) | 修改装备类物品解锁了几个格子 |
| [<skyb>:smug</skyb>](../action/material.md#无箭强持) | 执行下一个 <skyb>hold</skyb> 或 <skyb>drop</skyb> 指令后，执行无箭强持 |
| [<skyb>sort</skyb>](../action/sort.md) | 物品排序 |
| [<skyb>spawn</skyb>](../action/overworld.md#主世界生成物品) |主世界生成物品|
| [<skyb>!swap</skyb>](../action/low_level.md#修改物品数据) | 交互两个物品节点位置 |
| [<skyb>!system</skyb>](../action/system.md) | 系统操作 |
| <skyb>talk-to</skyb> | 和NPC对话触发购买或出售 |
| [<skyb>:targeting</skyb>](../action/entangle.md) | 修改选项纠缠目标物品 |
| [<skyb>!trial-end</skyb>](../action/trial.md) | 结束挑战，恢复背包 |
| [<skyb>!trial-start</skyb>](../action/trial.md) | 开始挑战，清空背包 |
| [<skyb>unequip</skyb>](../action/equip.md) | 解除物品装备 |
| [<skyb>unhold</skyb>](../action/material.md) | 取消手持 |
| [<skyb>unoverload</skyb>](../action/overload.md) | 取消过载 |
| <skyb>unpause</skyb> | 关闭背包 |
| <skyb>untalk</skyb> | 关闭NPC（购买/出售）对话框 |
| [<skyb>use</skyb>](../action/overworld.md) | 在主世界中使用装备或材料 |
| [<skyb>!write</skyb>](../action/low_level.md#修改物品数据) | 修改物品内存数据 |

</div>
