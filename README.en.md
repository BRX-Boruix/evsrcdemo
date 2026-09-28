# evsrcdemo

An **end-to-end acceptance program** for BORUIX, verifying that the regular event source component works on real hardware.

[简体中文](README.md)

## Why it is needed

[`evdemo`](https://github.com/BRX-Boruix/evdemo) is the **diagnostic form** — it deliberately bypasses the event source component and hand-writes the reading and parsing. So the component **itself** had only host-side unit tests and had never run on real hardware.

The system's input wiring needs that component, so it had to be verified first.

## Usage contract (learned the hard way — follow it)

These two are **real traps that were hit**. The first version of this program violated both, and on real hardware the symptom was "it opened successfully and then did nothing at all" — quite time-consuming to track down.

### 1. `next_item()` is not the entry point

With an empty buffer it **returns immediately** with "try again later" rather than fetching data. **Fetching is `refill()`'s job**.

### 2. `refill()` returning "no new bytes" is not end of file

It only means **no new data this round**, not that input has ended. The correct response is to **keep looping**, not to break out:

```rust
if refill() { continue; }
yield_now();
continue;      // never break
```

**Never treat it as end of file and `break`** — the program would then exit silently, looking exactly like "nothing happened".

## Exit codes

| Exit code | Meaning |
| --- | --- |
| `0` | Everything passed |
| Non-zero | Number of the first failing check |

## Building

```bash
cargo build --release
```

## Layout

```
evsrcdemo/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # real-hardware acceptance for the event source component
```

## Related projects

- [`libline`](https://github.com/BRX-Boruix/libline) — provides the event source component
- [`evdemo`](https://github.com/BRX-Boruix/evdemo) — the diagnostic form of the same chain

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
