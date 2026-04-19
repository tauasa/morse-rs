# Morse — Rust CLI

A command-line Morse code converter written in Rust.  
Encode plain text to Morse, decode Morse to text, and optionally play audio tones.

**Repository:** https://github.com/tauasa/morse-rs

---

## Requirements

| Tool | Version | Install |
|------|---------|---------|
| Rust + Cargo | 1.70+ | https://rustup.rs |

> On Linux you may also need `libasound2-dev` (ALSA) for audio:
> ```bash
> sudo apt install libasound2-dev   # Debian / Ubuntu
> sudo dnf install alsa-lib-devel   # Fedora / RHEL
> ```
> macOS and Windows use native audio APIs and need no extra packages.

---

## Build

```bash
# Debug build (fast compile, larger binary)
cargo build

# Release build (optimised, stripped binary)
cargo build --release

# Run tests
cargo test
```

The compiled binary is at:
- Debug:   `target/debug/morse`
- Release: `target/release/morse`

---

## Usage

```
morse <COMMAND> [OPTIONS] <INPUT…>

Commands:
  encode    Encode plain text to Morse code
  decode    Decode Morse code to plain text

Options:
  -p, --play    Play 700 Hz audio tones while printing output
  -h, --help    Print help
  -V, --version Print version
```

### Encode

```bash
morse encode "Hello World"
morse encode --play SOS
morse encode -p "Meeting at 3pm"

# Multiple arguments are joined with a space:
morse encode Hello World
```

### Decode

```bash
morse decode "... --- ..."
morse decode --play ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."

# Use quotes — the shell interprets dots and dashes literally,
# but spaces and slashes need quoting to arrive as one argument.
```

---

## Output format

```
┌────────────────────────────────────────────────────────────┐
│ Input  (Text):                                             │
│ Hello World                                                │
├────────────────────────────────────────────────────────────┤
│ Output (Morse):                                            │
│ .... . .-.. .-.. --- / .-- --- .-. .-.. -..                │
└────────────────────────────────────────────────────────────┘
```

Long values are word-wrapped to fit within the box.

---

## Morse format

| Symbol | Meaning |
|--------|---------|
| `.` | Dot |
| `-` | Dash |
| ` ` | Letter separator (single space) |
| ` / ` | Word separator (space-slash-space) |

---

## Supported characters

| Category | Characters |
|----------|-----------|
| Letters | A–Z (case-insensitive) |
| Digits | 0–9 |
| Punctuation | `. , ? ! - / @ ( )` |

---

## Audio playback (`--play`)

Uses **rodio** (pure Rust audio library, no C dependencies on most platforms).

| Parameter | Value |
|-----------|-------|
| Frequency | 700 Hz (classic Morse receiver tone) |
| Dot | 60 ms |
| Dash | 180 ms |
| Intra-character gap | 60 ms |
| Inter-letter gap | 180 ms |
| Inter-word gap | 420 ms |
| Envelope | 10 ms linear ramp-up/down (no clicks) |

Playback is synchronous — the process exits after all tones have played.

---

## Project structure

```
morse-rs/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs       CLI parsing (clap derive) + pretty-print output
    ├── morse.rs      Encode / decode logic + unit tests
    └── audio.rs      Sine-wave tone generation via rodio
```
