# hacks
CLI full of hacks that make my life easier

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

Build from source:
```bash
cargo build --release
```

The binary will be available at `target/release/hacks`.
