# evsrcdemo

**简体中文** | [English](#english)

BORUIX 上对 `libline::EventSource`**组件路径**的真机验收程序。

```
/devices/input/events → EventSource → 词法单元（字符 / 编辑键 / 控制键）
```

按 `q` 退出。

---

## 它与 evdemo 的分工

[`evdemo`](https://github.com/BRX-Boruix/evdemo) 走的是**诊断形态**——它刻意绕开了
`EventSource`，手写 `open → read → 解析 → 转换` 四步，为的是能看到每轮读取的原始真值。

代价是：`EventSource` **本身**没有被真机验证过，只有宿主机上的单元测试。

`evsrcdemo` 补上这一环：它使用**真正的组件路径**，验证 `EventSource` 在真机上工作正常。
在让 shell 之类的交互程序改用 `EventSource` 之前，必须先确认这个组件本身是可靠的。

| | evdemo | evsrcdemo |
| --- | --- | --- |
| 路径 | 手写全链路（诊断形态） | `EventSource` 组件 |
| 目的 | 看清每轮读取的真值 | 验证组件本身可用 |

## 用法契约：两个曾经踩过的坑

这个程序的源码里记录了两条**实测踩过**的契约。它们值得外部读者注意，因为违反后真机表现是
"打开成功但毫无反应"——完全没有错误提示，极难定位：

**一、`next_item()` 不是取数入口。** 缓冲为空时它会立即返回"无数据"。真正负责取数的是
`refill()`。

**二、`refill()` 返回 false 不是文件结束。** 它只表示"本轮没有新字节"。**绝不应当因此退出
循环**——正确做法是让出 CPU 后继续下一轮。

本程序的初版两条都违反了，结果就是"程序起来了、什么也不打印"。源码里保留了这段说明作为警示。

## 输出的词法单元

程序把事件流转换出的每个词法单元打印出来，覆盖字符、编辑键与控制键：

| 输出 | 含义 |
| --- | --- |
| `[CHAR x]` | 一个字符 |
| `[SUBMIT]` | 回车提交 |
| `[BACKSPACE]` `[DELETE]` | 删除 |
| `[LEFT]` `[RIGHT]` `[HOME]` `[END]` | 光标移动 |
| `[HISTPREV]` `[HISTNEXT]` | 历史记录浏览 |
| `[COMPLETE]` | 触发补全 |
| `[INTERRUPT]` | 中断（`^C`） |
| `[EOF]` | 输入结束 |

按 `^C` 或 `q` 都会退出，保证程序在异常情况下仍可控。

## 退出时的统计

退出前打印一行统计，用于核对数据链是否正常：

```
[evsrcdemo] chars=<字符数> items=<词法单元总数> wouldblock=<空转次数> rcalls=... rbytes=... drecs=... dbytes=... dcalls=...
```

其中 `wouldblock` 单独计数，用来观察"无数据"信号出现的频率——它不计入词法单元总数。

## 构建

```bash
cargo build --release
```

编译产物部署为 BORUIX 系统中的 `/programs/evsrcdemo.elf`，然后在 shell 中执行。

## 文件结构

```
evsrcdemo/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 程序本体
```

## 相关项目

- [`evdemo`](https://github.com/BRX-Boruix/evdemo) —— 同一数据链的诊断形态
- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 用户态系统调用封装

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。

---

# English

[简体中文](#evsrcdemo) | **English**

An on-hardware acceptance program for the `libline::EventSource` **component path** on BORUIX.

```
/devices/input/events → EventSource → lexical items (characters / editing keys / control keys)
```

Press `q` to quit.

---

## How it divides work with evdemo

[`evdemo`](https://github.com/BRX-Boruix/evdemo) takes the **diagnostic form** — it deliberately
bypasses `EventSource` and hand-rolls `open → read → parse → translate` so that the raw truth of
each read round is visible.

The cost is that `EventSource` **itself** was never verified on hardware, only by unit tests on the
host.

`evsrcdemo` closes that gap: it uses the **real component path** and verifies that `EventSource`
works on hardware. Before switching an interactive program such as the shell over to
`EventSource`, the component itself must first be confirmed sound.

| | evdemo | evsrcdemo |
| --- | --- | --- |
| Path | the whole chain by hand (diagnostic form) | the `EventSource` component |
| Purpose | see the per-round read truth | verify the component itself works |

## Usage contract: two pitfalls already hit

The source of this program records two contracts learned **from real failures**. They are worth an
outsider's attention because violating them makes the program "start successfully and then do
nothing at all" — with no error message whatsoever, making it very hard to diagnose:

**1. `next_item()` is not the data-fetching entry point.** It returns "no data" immediately when
the buffer is empty. `refill()` is what actually fetches.

**2. `refill()` returning false is not end-of-file.** It only means "no new bytes this round".
The loop should **never exit on it** — the right move is to yield the CPU and continue.

The first version of this program broke both, and the result was a program that came up and printed
nothing. That explanation is kept in the source as a warning.

## Lexical items it prints

The program prints every lexical item translated from the event stream, covering characters, editing
keys, and control keys:

| Output | Meaning |
| --- | --- |
| `[CHAR x]` | A character |
| `[SUBMIT]` | Enter to submit |
| `[BACKSPACE]` `[DELETE]` | Deletion |
| `[LEFT]` `[RIGHT]` `[HOME]` `[END]` | Cursor movement |
| `[HISTPREV]` `[HISTNEXT]` | History browsing |
| `[COMPLETE]` | Completion triggered |
| `[INTERRUPT]` | Interrupt (`^C`) |
| `[EOF]` | End of input |

Both `^C` and `q` exit, keeping the program controllable when something goes wrong.

## Statistics on exit

Before exiting it prints one statistics line for checking the data chain:

```
[evsrcdemo] chars=<chars> items=<total items> wouldblock=<spins> rcalls=... rbytes=... drecs=... dbytes=... dcalls=...
```

`wouldblock` is counted separately to observe how often the "no data" signal appears; it is not
included in the lexical item total.

## Building

```bash
cargo build --release
```

The artifact is deployed as `/programs/evsrcdemo.elf` in a BORUIX system, then run from the shell.

## Layout

```
evsrcdemo/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the program itself
```

## Related projects

- [`evdemo`](https://github.com/BRX-Boruix/evdemo) — the diagnostic form of the same data chain
- [`libsys`](https://github.com/BRX-Boruix/libsys) — the user-space syscall wrapper

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
