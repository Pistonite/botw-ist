# 物品语法

物品语法由三部分组成： [`数量`](#数量), [`名称`](#名称), 和 [`属性`](#属性)

```skybook
get    3        pot-lid   [durability=3]
#      ^ 数量   ^ 名称     ^ 属性
```

```admonish tip
同一指令中设定多种物品时，可直接连着写，如<skyb>2 apples 3 bananas</skyb>，不需要分隔。

当指令中只有一种物品，且数量为`1`时，数量可以省略。如<skyb>get 1 apple</skyb>可缩写为<skyb>get apple</skyb>。但要注意，当有2种或以上物品时，物品数量不可省略。
```

根据不同指令，物品语法可以有三种语境：

1. `FINITE_ITEM_LIST` （有限物品表）
   - 数量必须为数字，而如<skyb>all</skyb>的关键词。
   - 名称必须为物品而非类别。
   - 设定的属性用于描述物品属性。
   - 通常在获取物品的语境中使用，例如<skyb>get</skyb>指令。

2. `INFINITE_ITEM_LIST` （无限物品表）
   - 数量可以是数字，或<skyb>infinite</skyb>关键词。
   - 名称可以是物品或类别。
   - 设定的属性用于描述物品属性。
   - 当前没有指令使用此语境。

3. `CONSTRAINED_ITEM_LIST` （指定物品表）
   - 数量可以是数字，或：
     - 关键词<skyb>all</skyb>，指所有。
     - <skyb>all but X</skyb>, 指除了`X`个之外所有。
   - 名称可以是物品或类别。
   - 设定的属性用于匹配某个表（比如背包）中的物品
   - 通常用于需要选定物品的指令，比如<skyb>hold</skyb> 和 <skyb>eat</skyb>。
   - 可使用[位置属性](#从多个匹配物品中选择)。


## 数量

数量在不同指令中可能意思稍有不同。比如，在吃东西<skyb>eat</skyb>指令中，数量指格子内部数值（堆值）。因为讹转后的食物可以多次吃。而在出售<skyb>sell</skyb>指令中，对于不可堆叠物品（比如食物），数量通常指格子数。


在指定物品表`CONSTRAINED_ITEM_LIST`中，有两种特殊数量语法：<skyb>all</skyb> 和 <skyb>all but</skyb>
- <skyb>all</skyb> 所有：寻找此物品并执行指令，直到找不到更多。
- <skyb>all but X</skyb> 会先计算物品数量，然后减去`X`，再对物品执行该次数操作。注意如上所述，不同指令计算物品数量方式可能不同。

## 名称

物品名称可用以下4种方式设定：

1. `Identifier`（标识符）-
   物品标识符由字母，短横线和下划线组成。例如<skyb>royal-claymore</skyb>或<skyb>trav-bow</skyb>。标识符由固定算法解析到物品名。
   - 算法基于物品英文名。在物品名前还可以加英文名的料理效果，如 <skyb>hasty-elixir</skyb>, <skyb>sneaky-wild-greens</skyb>。
   - 支持英文名复数后缀 `-s`, `-es`, `-ies`
   - 支持某些物品的缩写。比如大鹫弓为<skyb>geb</skyb>，古代箭为<skyb>aa</skyb>。
2. `Actor` （内部名）-
   内部名由尖括号词表示，直接指定游戏内部物品名。如<skyb>get <Weapon_Sword_070></skyb> （大师剑）。
3. `Localization` （全语言名）-
   若不确定物品英文名，可使用物品其他语言的名称加引号。比如<skyb>"espadon royal"</skyb>或<skyb>"王族双手剑"</skyb>。
   - 引号内的内容将模糊匹配到物品
   - 可使用语言代码锁定语言。如<skyb>"fr:espadon royal"</skyb>锁定法语。
4. `Category` （类型）-
   当在背包或其他表中选择物品时，可使用类型关键词代替物品名来匹配该类型的第一个物品。比如，当只有一个盾装备时，可使用<skyb>unequip shield</skyb>解除当前盾。或<skyb>pick-up 3 weapons</skyb>捡起地上3个物品，但是无所谓捡起的是什么武器。

```admonish info
类型关键词为：`weapon`武器， `bow`弓，`shield`盾, `armor`衣服，`material`材料, `food`食物/料理，`key-item`重要道具。可加`s`变复数。
```

## 属性
[属性语法](./syntax.md#属性语法)可用于设定物品附加属性：

- 在有限物品表`FINITE_ITEM_LIST`中, 属性用于设定物品自身属性。
  - 例如<skyb>get pot-lid[durability=1]</skyb> 指令取得耐久为`1`的锅盖。
- 在指定物品表`CONSTRAINED_ITEM_LIST`中, 属性用于匹配选择的物品。
  - 例如，如果背包中有多个锅盖，<skyb>drop pot-lid[durability=1]</skyb> 将匹配耐久为`1`的锅盖并丢弃。

属性表：

| 属性名 | 别名 | 说明 |
|-|-|-|
| `durability` | `dura` |(`int`整数) 等同于设定 `value` 为设定值乘 100|
| `effect` | | (`int`整数或 `string`字符串) 设定料理效果ID。数字直接指定内部ID（就算数字不合法）。字符串会被转换为ID。见 [料理效果](../generated/constants.md#cook-effects) |
| `equipped` |`equip` | (`bool`布尔) 物品是否装备 |
| `ingr` | | (`string`字符串) 设定料理的材料。材料名必须是标识符（见上）。此属性可多次使用设定多个材料。 |
| `level`| | (`int`整数) 设定料理效果等级|
| `life-recover`| `hp`, `modpower` | (`int`整数) 设定料理回复值，单位为四分之一心。同时可指定附魔威力。 |
| `modifier` | `modtype` | (`int`整数或 `string`字符串) 设定附魔类型。<br><br>**不可用于设定料理效果**。 <br><br> 整数值同`price`。字符串值可多次设定以添加多个附魔效果。<br><br> 当作为匹配使用时，如果仅设定了一个附魔类型，则可以匹配任何包括该附魔的附魔类型。若设定超过一个类型，则附魔类型必须完全匹配。<br> 见 [附魔类型](../generated/constants.md#weapon-modifiers) |
| `price` | |(`int`整数) 设定料理出售价格。同时可指定附魔类型值。 |
| `star` | | (`int`整数) 装备升级（星）数。合法区间为 `0-4` （包含）。 <br>注意星数为修改物品名的语法糖。不同星数的同一件衣服实为不同物品。 |
| `time` | | (`int`整数) 设定料理效果持续时间。单位为秒。 |
| `value` | `life` | (`int`整数) 设定物品值（可堆叠物品的数量，或武器耐久乘100） <br>**注意不要和`life-recover`混淆** |
  
## 从多个匹配物品中选择
在指定物品表`CONSTRAINED_ITEM_LIST`中，可能会有背包中有多个完全一样的物品格的情况。这种情况下可以使用位置属性直接指定物品位置。

通过`from-slot`属性，可以指定第`n`个物品。比如，如果背包中有`3`个锅盖，<skyb>drop pot-lid[from-slot=2]</skyb> 会丢弃第二个。
序列号从`1`开始计算。

也可以通过以下几种方法直接指定格子的位置：

```skybook
# `slot` 为 `from-slot` 缩写
# 如果背包中有大于等于2格苹果，下面指令会从第二格吃
eat 2 apple[slot=2]

# 类别可以作为名称
# 下面指令会从背包所有类型为材料的格子中的第二格吃
eat 2 material[slot=2]

# 从材料页面，第一行，第二列吃苹果 （行数列数从1开始算）
eat 2 apple[category=material, row=1, col=2]

# 从第二个材料页面，第一行，第二列吃苹果
eat 2 apple[category=material, tab=2, row=1, col=2]

# 从第二个材料页面，第0号格子吃苹果
# 页面序号从1算，格子需要从0算, 下面是格子序号表
# 00 01 02 03 04
# 05 06 07 08 09
# 10 11 12 13 14
# 15 16 17 18 19
eat 2 apple[category=material, tab=2, slot=0]

# 从第0页面，第3号格子吃苹果
# 此处页面序号为所有页面总序号，从0开始算
eat 2 apple[tab=0, slot=3]
```

```admonish note
- 若以位置指定的格子中物品和指令中物品不同，指令将报错。
- 使用 `row` 和 `col` 属性时，必须放在`category`和`tab`之后。
```

```admonish warning
模拟器会在即将寻找该物品时计算物品位置。所以，当同一操作中该物品之前有物品，并因指令的操作改变了其他物品的位置，此时需要指定的位置可能和模拟器中一开始看到的位置不同。所以，不建议在用位置指定物品的指令中指定多个物品。可以将每一步分成单独的指令。
```
