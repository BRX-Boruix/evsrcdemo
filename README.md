# evsrcdemo

BORUIX 的验收程序：验证行编辑库的事件源组件在真机上的行为。

[English](README.en.md)

## 测什么

行编辑库有一个独立的事件源组件，负责从设备读取事件并喂给编辑器。此前该组件只有宿主机单元
测试；本程序在真实系统上驱动它：

- 打开 `/devices/input/events`，经事件源逐批取事件
- 解码为编辑器按键：提交、退格、删除、方向、补全、历史前后翻阅
- 按键逐个打印，按 `q` 退出，Ctrl+C 直接退出

输出：

```
[evsrcdemo] open ok; press keys (q quits)
[evsrcdemo] SUBMIT]
[evsrcdemo] BACKSPACE]
[evsrcdemo] PASS
```

## 退出码

- `0`——正常退出
- `1`——事件设备打不开

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
    └── main.rs   # 事件源循环与按键打印
```

## 相关项目

- [`libline`](https://github.com/BRX-Boruix/libline) —— 被验收的事件源组件
- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 事件设备接口

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。
