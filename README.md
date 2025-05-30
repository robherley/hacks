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
hacks uuid generate

# Generate a specific UUID version
hacks uuid generate --version 4
hacks uuid generate --version 1
hacks uuid generate --version 7

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

### From Source
```bash
git clone https://github.com/robherley/hacks.git
cd hacks
cargo build --release
sudo cp target/release/hacks /usr/local/bin/
```

### Development Installation
```bash
git clone https://github.com/robherley/hacks.git
cd hacks
cargo install --path .
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run the tests: `cargo test`
5. Run linting: `cargo clippy`
6. Format your code: `cargo fmt`
7. Commit your changes (`git commit -m 'Add some amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Adding New Commands

The CLI is structured for easy extensibility. To add a new command:

1. Create a new module in `src/commands/`
2. Add your command logic
3. Export the module in `src/commands/mod.rs`
4. Add the subcommand to the `Commands` enum in `src/main.rs`
5. Add the match arm in the main function

## License

MIT
