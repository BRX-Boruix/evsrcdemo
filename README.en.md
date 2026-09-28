# evsrcdemo

A BORUIX acceptance test exercising the line-editing library's event source component on real hardware.

[简体中文](README.md)

## What it tests

The line-editing library has a standalone event source component that reads events from the device
and feeds the editor. That component previously had only host unit tests; this program drives it on
the real system:

- Opens `/devices/input/events` and pulls events through the event source
- Decodes them into editor keys: submit, backspace, delete, arrows, completion, history prev/next
- Prints each key, quits on `q`; Ctrl+C exits directly

Output:

```
[evsrcdemo] open ok; press keys (q quits)
[evsrcdemo] SUBMIT]
[evsrcdemo] BACKSPACE]
[evsrcdemo] PASS
```

## Exit codes

- `0` — normal exit
- `1` — the event device could not be opened

## Building

```bash
cargo build --release
```

## Repository layout

```
evsrcdemo/
├── Cargo.toml    # package manifest
├── build.rs      # injects the linker script
├── linker.ld     # user-space segment layout
└── src/
    └── main.rs   # event source loop and key printing
```

## Related projects

- [`libline`](https://github.com/BRX-Boruix/libline) — the event source component under test
- [`libsys`](https://github.com/BRX-Boruix/libsys) — the event device interface

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
