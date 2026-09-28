# evsrcdemo

BORUIX 的**端到端验收程序**：验证常规事件源组件在真机上可用。

[English](README.en.md)

## 为什么需要它

[`evdemo`](https://github.com/BRX-Boruix/evdemo) 是**诊断形态**——它刻意绕开事件源组件，手写读取与解析。
因此组件**本身**此前只有宿主侧单元测试，没有在真机上跑过。

而系统的输入接线要用到这个组件，所以必须先验证它。

## 用法契约（实测踩过，务必遵守）

下面两条是**真实踩过的坑**。本程序初版两条都违反了，真机表现为「打开成功但毫无反应」——
排查起来相当费时。

### 1. `next_item()` 不是入口

缓冲为空时它会**立即返回**「稍后重试」，而不是去取数据。**取数由 `refill()` 负责**。

### 2. `refill()` 返回"没有新字节"不是文件结束

它只表示**本轮没有新数据**，不代表输入结束。正确的处理是**继续循环**，而不是跳出：

```rust
if refill() { continue; }
yield_now();
continue;      // 绝不 break
```

**绝不能把它当成文件结束而 `break`**——那样程序会静默退出，看起来像"什么都没发生"。

## 退出码

| 退出码 | 含义 |
| --- | --- |
| `0` | 全部通过 |
| 非零 | 首个失败项编号 |

## 构建

```bash
cargo build --release
```

## 文件结构

```
evsrcdemo/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 事件源组件的真机验收
```

## 相关项目

- [`libline`](https://github.com/BRX-Boruix/libline) —— 提供事件源组件
- [`evdemo`](https://github.com/BRX-Boruix/evdemo) —— 同一条链的诊断形态

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。
