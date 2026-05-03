# quickterm

A small drop-down terminal for Sway/i3 IPC. Rust port of [`i3-quickterm`](https://github.com/lbonn/i3-quickterm).

## Installation

Build and install with Cargo:

```bash
cargo install --path .
```

Or build a local binary:

```bash
cargo build --release
```

The binary will be available at `target/release/quickterm`.

## Usage

```bash
quickterm
quickterm shell
quickterm --in-place shell
```

Suggested bindings:

```bash
bindsym $mod+p exec quickterm
bindsym $mod+b exec quickterm shell
```

## Configuration

`quickterm` reads `quickterm.json` from:

- `$XDG_CONFIG_DIR/quickterm.json`, if `XDG_CONFIG_DIR` is set
- `~/.config/quickterm.json`, otherwise

Defaults:

```json
{
  "menu": "rofi -dmenu -p 'quickterm: ' -no-custom -auto-select",
  "term": "urxvt",
  "history": "{$HOME}/.cache/quickterm.order",
  "ratio": 0.25,
  "pos": "top",
  "shells": {
    "haskell": "ghci",
    "js": "node",
    "python": "ipython3 --no-banner",
    "shell": "{$SHELL}"
  }
}
```

## License

MIT
