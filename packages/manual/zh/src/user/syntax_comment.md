# 注释和笔记

注释和笔记是脚本中不影响指令输出的文本。

## 注释
以`#`或`//`开头的注释会被完全忽略。

```skybook
# This is a comment
// This is also a comment
```

> [!TIP]
> 在脚本编辑器中，可以使用快捷键 `Ctrl + /` 快速切换选中行的注释状态。

## 注释块
注释块是以`'''`（三个单引号）开头和结尾的多行块。

```skybook
'''
This is a block literal

It can have multiple lines
'''
```

此外，注释块可以有一个`标签`，即紧跟在起始`'''`后面的字符串。例如，`note`标签可以用于为一组指令添加笔记，这些笔记可在`Notes`扩展中查看。

```skybook
'''note
Drop these in the same pile
'''
drop all weapons
drop all shields
```

> [!NOTE]
> `Notes`功能尚未实现。