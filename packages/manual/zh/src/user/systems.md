# 模拟器系统
Skybook旨在100%准确模拟IST。其中使用了2种模拟方法：完全模拟(Emulation)和重实现(Reimplementation)。我们仅可能地完全模拟游戏中的子系统。但是，不是所有子系统目前都能完全模拟，特别是未逆向研究的子系统。有些子系统通过重实现已经能基本模拟所有功能，这些也不值得完全模拟。

模拟器中的子系统包括：
- 背包系统
- GDT旗标系统
- [存档系统](../action/save.md)
- [界面系统](./screen_system.md)
- [主世界系统](./overworld_system.md)
