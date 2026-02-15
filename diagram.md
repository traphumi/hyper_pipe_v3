# hyper_pipe V3 – Architecture Diagram

This document explains the data flow and trust model of hyper_pipe V3.

---

## 🔁 High-Level Flow

    Publisher                         Client
─────────────────                 ─────────────────
   file.bin                         file.bin.idx
       │                                 │
       ▼                                 ▼
┌────────────┐                  ┌────────────────┐
│  Chunker   │                  │ Signature Check│
└────────────┘                  └────────────────┘
       │                                 │
       ▼                                 ▼
┌────────────┐                  ┌────────────────┐
│   Hasher   │                  │ Range Download │
└────────────┘                  └────────────────┘
       │                                 │
       ▼                                 ▼
┌────────────┐                  ┌────────────────┐
│ Index (.idx)│                  │ Hash Verify    │
└────────────┘                  └────────────────┘
       │                                 │
       ▼                                 ▼
Signed with Ed25519             Reconstruct file.bin

---

## 🔐 Trust Model

[ Untrusted ]
HTTP Server
│
▼
[ Trusted ]

Signed index (.idx)

Public key

Hash algorithm (BLAKE3)


Client trust boundary:


network ❌
server ❌
file ❌
index ✅
key ✅
hash ✅


---

## 🧠 Detailed Pipeline

1. Publisher splits file into chunks (content-defined)
2. Each chunk is hashed (BLAKE3)
3. Metadata is stored in index file:
   - file name
   - file size
   - chunk sizes
   - chunk hashes
4. Index file is signed with private key
5. Client downloads index file
6. Client verifies signature with public key
7. Client downloads chunks using HTTP Range
8. Client verifies hash of every chunk
9. Client reconstructs the file
10. Any mismatch → abort

---

## ⚠️ Failure Modes

| Scenario              | Result |
|-----------------------|--------|
| Server modifies file  | Detected (hash mismatch) |
| Network corruption    | Detected (hash mismatch) |
| Fake index file       | Detected (signature invalid) |
| Truncated chunk       | Detected (hash mismatch) |

---

## 🧩 Design Inspiration

- BitTorrent piece hashing
- IPFS content addressing
- Secure OTA update systems
- Zero-trust network design

---

## 📌 Summary

hyper_pipe V3 implements:
- Integrity
- Authenticity
- Zero-trust file distribution

It does NOT yet implement:
- Confidentiality
- Anonymity
- Obfuscation

These may be added in future versions.
