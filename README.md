# hashcheck

A fast, multi-algorithm checksum toolkit CLI written in Rust. Compute, verify, and generate file hashes with support for SHA-256, SHA-512, SHA-1, MD5, and BLAKE2b.

## Features

- **Hash files** — compute checksums for individual files or all files in a directory
- **Verify checksums** — validate files against standard checksum files (SHA256SUMS / MD5SUMS format)
- **Generate checksum files** — create checksum files for directories
- **Compare files** — quickly check if two files have the same hash
- **Multiple output formats** — text, JSON, CSV
- **Multiple algorithms** — SHA-256, SHA-512, SHA-1, MD5, BLAKE2b
- **Stdin support** — pipe data directly into the hash command

## Installation

### From source

```bash
cargo install --path .
```

### Manual build

```bash
git clone https://github.com/EdgarOrtegaRamirez/hashcheck.git
cd hashcheck
cargo build --release
cp target/release/hashcheck /usr/local/bin/
```

## Usage

```bash
# Hash a single file (SHA-256 by default)
hashcheck hash file.txt

# Hash with a specific algorithm
hashcheck hash file.txt -a sha512
hashcheck hash file.txt -a md5
hashcheck hash file.txt -a blake2b

# Hash all files in a directory
hashcheck dir /path/to/dir

# Hash all files recursively
hashcheck dir /path/to/dir -r

# Hash from stdin
echo "hello world" | hashcheck hash -

# Generate a checksum file for a directory
hashcheck genfile /path/to/dir -o checksums.txt

# Generate recursively
hashcheck genfile /path/to/dir -r -o checksums.txt

# Verify a checksum file
hashcheck verify checksums.txt --base /path/to/dir

# Compare two files
hashcheck compare file1.txt file2.txt

# Output in JSON format
hashcheck hash file.txt -f json

# Output in CSV format
hashcheck dir /path/to/dir -f csv
```

## Output Formats

Use `-f` or `--format` to control output:

- `text` (default) — human-readable hash output
- `json` — machine-readable JSON output
- `csv` — CSV format for data processing

## Checksum File Format

`hashcheck` supports the standard checksum file format used by GNU coreutils:

```
<hash>  <filename>
<hash> *<filename>    # binary mode indicator
```

Lines starting with `#` are treated as comments and ignored.

## Build from source

```bash
cargo build        # debug build
cargo build --release  # optimized release build
cargo test         # run tests
```

## License

MIT License — see [LICENSE](LICENSE) for details.