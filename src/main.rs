//! BORUIX evsrcdemo：`libline::EventSource`（**组件路径**）的真机验收。
//!
//! `evdemo` 是**诊断形态**——手搓 open/read/parse/feed，明确「绕开
//! `libline::EventSource`」。故 `EventSource` **本身**此前只有宿主单测。
//! 阶段 3 的最小形态要让 shell/login 改用 `EventSource`，故需先验组件。
//!
//! ## 用法契约（实测踩过，务必遵守）
//!
//! 1. `next_item()` **不是**入口：缓冲空时立即返回 `WouldBlock`，
//!    取数由 `refill()` 负责（`libline/src/source.rs` 的 `InputSource` 文档）。
//! 2. `refill() == false` **不是** EOF，只表示「本轮无新字节」。
//!    权威用法见 `libline/src/editor.rs:471-475`：
//!    `if refill() { continue; } yield_now(); continue;` —— **绝不 break**。
//!
//! 本程序初版两条都违反了，真机表现为「open 成功但毫无反应」。
#![no_std]
#![no_main]

use libline::{EventSource, InputItem, InputSource};
use libsys::{write, STDOUT};

fn out(b: &[u8]) {
    let _ = write(STDOUT, b);
}

fn outln(b: &[u8]) {
    out(b);
    out(b"\n");
}

fn out_u64(mut v: u64) {
    let mut buf = [0u8; 20];
    let mut i = buf.len();
    if v == 0 {
        i -= 1;
        buf[i] = b'0';
    }
    while v > 0 {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    out(&buf[i..]);
}

/// 打印一个词法单元，返回是否应退出（用户按 q）。
fn show(item: &InputItem) -> bool {
    out(b"[");
    match item {
        InputItem::Char(c) => {
            out(b"CHAR ");
            let mut b = [0u8; 4];
            out(c.encode_utf8(&mut b).as_bytes());
            out(b"]");
            return *c == 'q';
        }
        InputItem::Submit => out(b"SUBMIT]"),
        InputItem::Backspace => out(b"BACKSPACE]"),
        InputItem::Delete => out(b"DELETE]"),
        InputItem::Left => out(b"LEFT]"),
        InputItem::Right => out(b"RIGHT]"),
        InputItem::Home => out(b"HOME]"),
        InputItem::End => out(b"END]"),
        InputItem::Complete => out(b"COMPLETE]"),
        InputItem::HistoryPrev => out(b"HISTPREV]"),
        InputItem::HistoryNext => out(b"HISTNEXT]"),
        InputItem::Interrupt => out(b"INTERRUPT]"),
        InputItem::Eof => out(b"EOF]"),
        InputItem::WouldBlock => out(b"WOULDBLOCK]"),
    }
    outln(b"");
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn user_main(_argc: isize, _argv: *const *const u8) -> i32 {
    outln(b"[evsrcdemo] using libline::EventSource (component path)");
    let mut src = match EventSource::open() {
        Ok(s) => s,
        Err(_) => {
            outln(b"[evsrcdemo] FAIL open(/devices/input/events)");
            return 1;
        }
    };
    outln(b"[evsrcdemo] open ok; press keys (q quits)");
    outln(b"[evsrcdemo] v2-instrumented");
    let mut chars: u64 = 0;
    let mut items: u64 = 0;
    let mut wb: u64 = 0;
    loop {
        match src.next_item() {
            InputItem::WouldBlock => {
                wb += 1;
                // 临时取证（定位后移除）：空转上限，保证统计行必然浮出。
                if wb > 4000 {
                    outln(b"[evsrcdemo] wb-cap reached");
                    break;
                }
                // 契约：false 只表示「本轮无新字节」，**不是** EOF。
                if src.refill() {
                    continue;
                }
                let _ = libsys::yield_now();
                continue;
            }
            InputItem::Eof => {
                outln(b"[evsrcdemo] EOF; stopping");
                break;
            }
            item => {
                if let InputItem::Char(_) = item {
                    chars += 1;
                }
                // γ 判据的 items 真值（§6.14.4p）：**全部**产出单元（不止 Char）。
                // WouldBlock 不算——它是「外泄的空转信号」，由 wb 单独计。
                items += 1;
                // 临时取证（定位后移除）：Interrupt (^C) 也退出，保证卡死时可控。
                if let InputItem::Interrupt = item {
                    show(&item);
                    outln(b"[evsrcdemo] ^C exit");
                    break;
                }
                if show(&item) {
                    break;
                }
            }
        }
    }
    out(b"[evsrcdemo] chars=");
    out_u64(chars);
    out(b" items=");
    out_u64(items);
    out(b" wouldblock=");
    out_u64(wb);
    let (rc, rb, drecs, dbytes, dcalls) = libsys::event::debug_read_stats();
    out(b" rcalls=");
    out_u64(rc);
    out(b" rbytes=");
    out_u64(rb);
    out(b" drecs=");
    out_u64(drecs);
    out(b" dbytes=");
    out_u64(dbytes);
    out(b" dcalls=");
    out_u64(dcalls);
    outln(b"");
    outln(b"[evsrcdemo] PASS");
    0
}
