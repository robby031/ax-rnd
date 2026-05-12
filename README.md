# ax-rnd

AxRng is a fast, small random number generator (rng) library and CLI tool written in Rust.

[![ax-rnd](https://img.shields.io/crates/v/ax-rnd.svg)](https://crates.io/crates/ax-rnd)
[![docs-ax-rnd](https://docs.rs/ax-rnd/badge.svg?label=docs-ax-rnd)](https://docs.rs/ax-rnd)
[![no_std](https://img.shields.io/badge/no__std-supported-success)](#)
[![rust](https://img.shields.io/badge/rust-1.85%2B-orange)](https://www.rust-lang.org)
[![license](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/licenses/MIT)

**Features:**

- **Fast:** Single-call and bulk operations
- **Small:** Single `u64` state (32 bytes with alignment)
- **Pure Rust:** No external dependencies
- **CLI Tool:** Installable via `cargo install ax-rnd`

## Performance

### Benchmark Results (Release Mode)

**Single-call (`next_u64`):**
| rng | ps/iter | vs fastrand |
|-----|---------|-------------|
| **ax-rnd** | **317** | **0.9% faster** |
| fastrand | 321 | baseline |
| rand_small | 692 | 2.2x slower |

**Bulk fill (`fill_bytes/1MB`):**
| rng | GiB/s | vs fastrand |
|-----|-------|-------------|
| **ax-rnd** | **37.68** | **0.1% faster** |
| fastrand | 37.64 | baseline |
| rand_small | 10.74 | 3.5x slower |

**Algorithm:** Custom variant based on Golden Ratio constant with XOR-first-then-multiply pattern.

## Installation

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
ax-rnd = "0.1.0"
```

### As a CLI Tool

```bash
cargo install ax-rnd
```

## Library API

### Basic Usage

#### Convenience API (like fastrand)

For quick random generation without managing state:

```rust
use ax_rnd::{u8, u16, u32, u64, bool, f32, f64, alphanumeric, base64url, fill};

// Generate random numbers directly
let x = u32();
let y = u64();
let b = bool();

// Generate random floats
let f = f32();
let d = f64();

// Generate random strings
let token = alphanumeric(32);  // base62 alphanumeric
let url_token = base64url(32);  // base64url (URL-safe)

// Fill buffers
let mut buf = [0u8; 1024];
fill(&mut buf);
```

#### Deterministic API (with seed control)

For production use where you need deterministic behavior:

```rust
use ax_rnd::{rng, AxRng};

// Create RNG with seed
let mut rng = rng(12345);

// Generate random values
let x = rng.next_u64();
let y = rng.next_u32();
let b = rng.next_bool();
let f = rng.next_f64();
```

### Alphanumeric Strings

```rust
use ax_rnd::rng;

let mut rng = rng(42);

// Base62 (A-Z, a-z, 0-9) - 62 characters
let alpha = rng.next_alphanumeric(32);
let token = rng.alpha(32);  // alias

// Base64url (A-Z, a-z, 0-9, -, _) - 64 characters
let url_token = rng.next_base64url(32);
let token = rng.token(32);  // alias
```

### Bounded Random Numbers

```rust
use ax_rnd::rng;

let mut rng = rng(42);

// Random number in [0, upper)
let x = rng.bounded_u64(100);
let y = rng.bounded_u32(50);
```

### Bulk Operations

```rust
use ax_rnd::rng;

let mut rng = rng(42);

// Fill byte buffer
let mut buf = [0u8; 1024];
rng.fill_bytes(&mut buf);

// Fill u64 buffer
let mut u64_buf = [0u64; 16];
rng.fill_u64(&mut u64_buf);

// Fill u32 buffer
let mut u32_buf = [0u32; 32];
rng.fill_u32(&mut u32_buf);
```

### State Management

```rust
use ax_rnd::AxRng;

let mut rng = AxRng::new(12345);

// Get current state
let state = rng.state(); // [u64; 1]

// Restore from state
let mut restored = AxRng::from_raw(state[0]);

// Split for independent streams
let mut other = rng.split();

// Reseed
rng.reseed(54321);
```

### Top-level Functions

```rust
use ax_rnd;

// One-off random values (creates new rng each time)
let x = ax_rnd::random_u64(42);
let y = ax_rnd::random_u32(42);
let b = ax_rnd::random_bool(42);
let f = ax_rnd::random_f64(42);

// One-off bounded
let z = ax_rnd::bounded_u64(42, 100);

// One-off strings
let alpha = ax_rnd::alpha(42, 32);
let token = ax_rnd::token(42, 32);
```

## CLI Usage

### Commands

```bash
ax_rnd <command> [args]
```

| Command                        | Description                         |
| ------------------------------ | ----------------------------------- |
| `bytes [count] [--hex] [seed]` | Generate random bytes               |
| `alpha [len] [seed]`           | Generate alphanumeric (base62)      |
| `token [len] [seed]`           | Generate URL-safe token (base64url) |
| `shuffle [file] [seed]`        | Shuffle lines from stdin or file    |
| `u64 [count] [seed]`           | Generate random u64 numbers         |
| `uuid [count] [seed]`          | Generate UUID v4                    |

### Examples

```bash
# Generate 64 random bytes (binary output)
ax_rnd bytes 64 > random.bin

# Generate 32 bytes as hex string
ax_rnd bytes 32 --hex
# 836233f222448066354d9340859e3743

# Generate 16-char alphanumeric string
ax_rnd alpha 16
# SEV3G6JYHei5YFhe

# Generate 32-char URL-safe token
ax_rnd token 32
# 5J8hK3mP9qR2sT4uV6wX8yZ0

# Generate 5 random u64 numbers
ax_rnd u64 5
# 12345678901234567890
# 98765432109876543210
# ...

# Generate 3 UUID v4
ax_rnd uuid 3
# a1b2c3d4-e5f6-7890-abcd-ef1234567890
# ...

# Shuffle lines from file
echo -e "a\nb\nc\nd" | ax_rnd shuffle
# c
# a
# d
# b

# Shuffle lines from file with seed
ax_rnd shuffle file.txt

# Use specific seed for reproducibility
ax_rnd bytes 32 12345
```

### Options

- `count/len` - Number of bytes/chars/numbers (default: 32)
- `--hex` - Output bytes as hex string
- `seed` - rng seed (default: current timestamp)
  - Use `time` for current timestamp
  - Use numeric value for fixed seed

## Testing

### Unit Tests

```bash
cargo test
```

**Test Coverage:**

- Deterministic sequences
- Different seeds produce different streams
- Fill operations
- Bounded range correctness
- Float range correctness
- State roundtrip
- Split functionality
- Alphanumeric character sets
- Base64url character sets
- Statistical quality tests (avalanche, bit distribution, stream correlation)

**Results:** 28 tests passing

### Statistical Quality Tests

The library includes statistical quality tests in `tests/statistical.rs`:

- Bit histogram uniformity
- Avalanche effect
- Stream correlation
- Low byte uniformity
- PractRand stream quality (8MB sample)

**Results:** All statistical tests passing

### Benchmarking

```bash
cargo run --example manual_bench --release
```

## Algorithm

**Core Algorithm:** Custom variant based on Golden Ratio constant (0x9E3779B97F4A7C15)

**Pattern:**

```rust
state += GR;              // Golden ratio increment
xored = state ^ GR;       // XOR first
math = state * xored;     // Multiply (128-bit)
return (math as u64) ^ ((math >> 64) as u64);  // Fold
```

## License

MIT
