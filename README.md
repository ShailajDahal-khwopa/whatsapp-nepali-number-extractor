# WhatsApp Nepali Phone Number Extractor
A Rust CLI tool to extract Nepali phone numbers from WhatsApp text exports. The tool handles deduplication and WhatsApp-specific Unicode characters, producing a clean numbers.txt output.

## Features
- Extracts Nepali numbers like +977 9XX-XXXXXXX

- Deduplicates repeated numbers

- Handles WhatsApp hidden characters (LTR/RTL marks, non-breaking spaces, special hyphens)

- Saves output in the same directory as the input file

- Works with large WhatsApp exports efficiently

- Simple CLI usage

## Installation

### 1. Clone the repository
```
git clone <your-repo-url>
cd whatsapp_numbers
```

### 2. Build the binary
**Debug build:**
```
cargo Build
```
Binary will be located at:
```
target/debug/
```

**Release build(recommended):**
```
cargo build --release
```
Binary will be located at:
```
target/release/
```

## Usage
```
./whatsapp_number_extractor <input_file>
```

## Developement
- Requires Rust
- Dependencies [regex](https://crates.io/crates/regex)

## License
MIT License  2025
