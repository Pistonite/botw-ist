# 模拟器的使用

```admonish info title="信息"
了解IST基本原理可以帮助理解模拟器功能。见[物品转存简介](../ist/index.md)。
```

## 基本功能
模拟器通过运行**IST脚本**执行IST步骤。脚本中包含了**指令**。每个指令对应游戏中的**操作**。

如下是一个示例脚本。每一行是一个指令。

```skybook
get 1 pot-lid 1 apple 1 slate 1 glider
equip Shield
!break 3 slots
save
unequip shield
hold apple; drop
reload
save
drop apple
reload
```

详见[指令语法](./syntax.md)和[指令参考](./commands.md)。

在App界面中，脚本需要在脚本编辑器内编辑。脚本改动后，模拟器会自动开始重新执行脚本。界面右侧会显示当前光标所在位置对应步骤的背包状态（状态为执行当前所在指令之后的状态）。

## 模式

模拟器App有3种编辑模式：
- 自动保存：默认模式。脚本的改动会自动保存到浏览器本地。下次打开模拟器的时候会自动读取。
- 不保存：脚本改动不会自动保存到浏览器，关闭浏览器会丢失改动。
- 只读：打开内嵌脚本网址时的默认模式。此模式脚本无法编辑。你可以切换到不保存模式做临时改动。
  注意只读模式下，脚本报错不会显示。

模式可以用App标题栏左上按钮切换。


~~~admonish warning title="注意"
切换到自动保存时会覆盖浏览器本地保存的脚本！

如果不小心覆盖了本地脚本，可以使用浏览器命令行恢复。打开命令行(F12)并输入以下指令（可能需要先按照说明开启指令复制）：
```typescript
console.log(localStorage.getItem("Skybook.AutoBackupScript"))
```

按回车，并复制输出的指令。

注意每次切换到自动保存模式时，这里都会保存覆盖掉的指令。所以再次覆盖后就无法恢复了！
~~~

## V3脚本转换

V3脚本可以自动转换为V4脚本，只需把网址中`itntpiston` 替换为 `pistonite` 替换为 `pistonite`。

由于脚本是机器转换的，可能有些地方需要手动调整。可以把模式从只读切换为不保存，然后查看是否有报错。也可以直接看最后一步的结果是不是一样。

重要修改：
- <skyb>drop</skyb> 指令在V4中只能丢能丢的物品。比如<skyb>drop hasty-elixir</skyb>（丢弃速速药）会报错，需要改为<skyb>eat hasty-elixir</skyb>（吃掉速速药）。
- `pick up` 会转换为 <skyb>get</skyb>。这是因为V4中<skyb>pick-up</skyb>只能指定地上的物品。这不会导致出错，但是可能地上会多很多物品。可以在最后一步之后加一行<skyb>!system [clear-ground]</skyb>来删除地上所有物品。
- 选项纠缠需要用<skyb>entangle</skyb>指令触发。

```admonish info title="信息"
虽然可以用如<skyb>!remove</skyb>等高级指令使转换后的脚本更贴合V3，但这并不是高级指令原本用途，不值得为一些边界情况牺牲脚本本来的含义。
```
