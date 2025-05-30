# hacks
CLI full of hacks that make my life easier

## Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)
- Cargo (comes with Rust)

## Building

### Quick Build
```bash
cargo build --release
```

### Development Build
```bash
cargo build
```

The binary will be available at:
- Release: `target/release/hacks`
- Development: `target/debug/hacks`

## Development

### Running Tests
```bash
cargo test
```

### Linting
```bash
cargo clippy
```

### Formatting
```bash
cargo fmt
```

### Check Formatting
```bash
cargo fmt --check
```

## Usage

### IP Commands
```bash
# Get external IP address
hacks ip external

# Get internal IP addresses
hacks ip internal
```

### UUID Commands
```bash
# Generate a UUID (default: v7)
hacks uuid new

# Generate a specific UUID version
hacks uuid new --version 4
hacks uuid new --version 1
hacks uuid new --version 7

# Get information about a UUID
hacks uuid info 01971f3a-8606-7a13-aa68-06059c72a37e
```

### Msgpack Commands
```bash
# Decode base64 msgpack from stdin
echo "gaNuYW1lpHRlc3SlYXJyYXmTAQIDpXZhbHVle4w=" | hacks msgpack decode
```

### CSV to Markdown
```bash
# Convert CSV to markdown table
echo -e "name,age\nJohn,25\nJane,30" | hacks csv2md

# Convert CSV with header row
echo -e "name,age\nJohn,25\nJane,30" | hacks csv2md --header
```

## Installation

```bash
cargo install --git https://github.com/robherley/hacks
```

## License

MIT
