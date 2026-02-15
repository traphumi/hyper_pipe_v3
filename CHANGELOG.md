# Changelog

All notable changes to this project will be documented in this file.

This project follows a simple versioning scheme:
MAJOR.MINOR.PATCH

---

## [3.0.0] - 2026-02-15

### 🚀 Added (New Features)
- Content-defined chunking (rolling hash based)
- Per-chunk integrity verification using BLAKE3
- Signed index file (.idx) using Ed25519
- Zero-trust download model (server is untrusted)
- Modular architecture:
  - chunker.rs
  - crypto.rs
  - index.rs
  - downloader.rs
  - cli.rs
  - types.rs
- Support for HTTP Range requests
- Secure file reconstruction with hash validation

### 🔐 Security
- Digital signature verification for index file
- Hash verification for every downloaded chunk
- Abort on any mismatch (fail-safe)

### 🧪 Verified
- End-to-end test with random binary data
- sha256(test.bin) == sha256(downloaded.bin)

### 🗂 Project Structure
- Refactored from monolithic design (V2) to modular design (V3)
- Clear separation of concerns:
  - Chunking logic
  - Cryptography
  - Indexing
  - Downloading
  - CLI interface

---

## [2.x.x] - Legacy

### Features
- Basic file chunking
- Simple hash-based verification
- Monolithic code structure

### Limitations
- No signed index
- Weaker trust model
- Less modular and harder to extend

---

## Planned for [3.1.0]

- Parallel chunk download
- Resume support
- Progress bar
- Zstd compression per chunk

---

## Planned for [4.0.0]

- Encrypted chunks (confidentiality)
- Multi-mirror download
- P2P transport mode
- Server mode
