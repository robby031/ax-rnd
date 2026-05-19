# ax-rnd

Fast and lightweight random number generator library and CLI tool for Rust.

- Small single-state RNG
- Fast bulk operations
- Pure Rust implementation
- `no_std` support
- Installable CLI tool

[![ax-rnd](https://img.shields.io/crates/v/ax-rnd.svg)](https://crates.io/crates/ax-rnd)
[![docs-ax-rnd](https://docs.rs/ax-rnd/badge.svg?label=docs-ax-rnd)](https://docs.rs/ax-rnd)
[![no_std](https://img.shields.io/badge/no__std-supported-success)](#)
[![rust](https://img.shields.io/badge/rust-1.85%2B-orange)](https://www.rust-lang.org)
[![license](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/licenses/MIT)

---

## Installation

### Library

```toml
[dependencies]
ax-rnd = "0.1"
```

### CLI

```bash
cargo install ax-rnd
```

---

## Quick Start

### Convenience API

```rust
use ax_rnd::{
    u32,
    u64,
    bool,
    f64,
    alphanumeric,
    base64url,
    fill,
};

let n = u64();
let ok = bool();

let token = alphanumeric(32);
let url = base64url(32);

let mut buf = [0u8; 1024];
fill(&mut buf);
```

---

### Seeded RNG

```rust
use ax_rnd::rng;

let mut rng = rng(12345);

let x = rng.next_u64();
let y = rng.next_u32();
let f = rng.next_f64();
```

---

## Bounded Numbers

```rust
use ax_rnd::rng;

let mut rng = rng(42);

let x = rng.bounded_u64(100);
let y = rng.bounded_u32(50);
```

---

## Random Strings

```rust
use ax_rnd::rng;

let mut rng = rng(42);

let alpha = rng.alpha(32);
let token = rng.token(32);
```

---

## Bulk Operations

```rust
use ax_rnd::rng;

let mut rng = rng(42);

let mut bytes = [0u8; 1024];
rng.fill_bytes(&mut bytes);

let mut u64s = [0u64; 16];
rng.fill_u64(&mut u64s);
```

---

## State Management

```rust
use ax_rnd::AxRng;

let mut rng = AxRng::new(12345);

let state = rng.state();

let restored = AxRng::from_raw(state[0]);

let other = rng.split();

rng.reseed(999);
```

---

## Top-level Helpers

```rust
use ax_rnd;

let x = ax_rnd::random_u64(42);

let token = ax_rnd::token(42, 32);

let bounded = ax_rnd::bounded_u64(42, 100);
```

---

## CLI Usage

```bash
ax_rnd <command> [args]
```

| Command | Description |
|---|---|
| `bytes` | Generate random bytes |
| `alpha` | Generate alphanumeric string |
| `token` | Generate URL-safe token |
| `shuffle` | Shuffle input lines |
| `u64` | Generate random numbers |
| `uuid` | Generate UUID v4 |

---

## CLI Examples

```bash
ax_rnd bytes 32 --hex

ax_rnd alpha 16

ax_rnd token 32

ax_rnd u64 5

ax_rnd uuid 3
```

---

## Notes

- Deterministic when using fixed seeds
- Portable pure Rust implementation
- Suitable for fast non-cryptographic randomness
- Supports `no_std`

---

## Links

- Crate: https://crates.io/crates/ax-rnd
- Docs: https://docs.rs/ax-rnd
- Rust: https://www.rust-lang.org

---

## License

MIT