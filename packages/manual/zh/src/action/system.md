# 系统操作

<skyb>!system</skyb>指令可直接操控模拟器底层系统。此指令不稳定，可能因为模拟器内部系统改变而在后续版本改变。

## 语法
> `!system [SYSTEM_META]`

`SYSTEM_META` 元属性会解析为系统指令，并按顺序执行。

<div class="skybook--wide-table">

| 属性 | 说明 |
| - | - |
| `dlc` | (`int` 整数或 `string`字符串) 更改游戏中`AocManager`存的DLC版本号。<br><br>数字 `0`, `1`, `2` 对应无DLC，第一天免费版DLC以及大师试炼。其他数字对应英杰之诗。字符串数值见[DLC 版本名](../generated/constants.md) |
| `delete-save` | (无值或`string`字符串) 删除存档数据。值为存档名，无值为手动存档。|
| `clear-ground` | (无值) 删除地上的所有物品，包括正在生成的物品。 |
| `clear-overworld` | (无值) 删除主世界所有物品，包括玩家身上的装备。 |
| `sync-overworld` | (无值) 同步 (即重新生成) 主世界玩家身上的装备。 |
| `reload-gdt` | (无值或`string`字符串) 载入存档数据到GDT，但是不载入背包。值为存档名，无值为手动存档。 |
| `loading-screen` | (无值或`string`字符串) 触发加载界面。可用特殊值`no-remove-translucent`在不删除虚像格的情况下触发。 |

</div>

例子:
```skybook
# 拿传送标注器，然后存档并删除DLC
get travel-medallion; save; close-game
!system [dlc=master-trials]
reload # 传送标注器不会载入背包

# 模拟进入神庙并通关神庙
!system [loading-screen]
get spirit-orb
!system [loading-screen]
```

