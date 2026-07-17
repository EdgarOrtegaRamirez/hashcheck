# AGENTS.md

This file provides guidance for AI agents working with this codebase.

## Project Overview

`hashcheck` is a Rust CLI tool for computing, verifying, and generating file checksums.
It supports SHA-256, SHA-512, SHA-1, MD5, and BLAKE2b algorithms.

## Code Structure

```
src/
  main.rs       — Entry point, calls cli::run()
  cli.rs        — Clap CLI definition and command handlers
  algorithm.rs  — Algorithm enum and display implementation
  hasher.rs     — File hashing implementation
  output.rs     — CSV writer for tabular output
  verifier.rs   — Checksum file parsing and verification
tests/
  integration.rs — Integration tests using assert_cmd
```

## Key Commands

- `hashcheck hash <file> [-a <algo>] [-f <format>]` — Hash a file
- `hashcheck dir <dir> [-r]` — Hash all files in a directory
- `hashcheck genfile <dir> [-o <file>] [-r]` — Generate checksum file
- `hashcheck verify <checksum-file> [--base <dir>]` — Verify checksums
- `hashcheck compare <file1> <file2>` — Compare two files

## Adding a New Algorithm

1. Add variant to `Algorithm` enum in `src/algorithm.rs`
2. Add the `Hasher` trait impl or use the existing macro
3. Update `Algorithm::from_hex_len()` in `algorithm.rs`
4. Add test in `tests/integration.rs`

## Dependencies

- `clap` — CLI parsing (derive mode)
- `sha2`, `sha1`, `md-5`, `blake2` — Hash algorithms
- `anyhow` — Error handling
- `walkdir` — Directory traversal
- `colored` — Terminal coloring
- `serde_json` — JSON output
- `csv` — CSV output/writer

## Testing

```bash
cargo test        # run all tests
cargo test test_hash_file  # run specific test
```