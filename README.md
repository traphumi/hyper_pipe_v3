# 🚀 hyper_pipe V3

> **Secure, chunk-based file distribution over untrusted HTTP servers**
> Inspired by BitTorrent, IPFS, and secure OTA systems.

`hyper_pipe` คือเครื่องมือส่งไฟล์ผ่าน HTTP แบบ **ไม่เชื่อ server**
โดยใช้:

* ✅ Content-defined chunking
* ✅ Cryptographic hash (BLAKE3)
* ✅ Digital signature (Ed25519)
* ✅ HTTP Range requests

แม้ server จะโกงข้อมูล → client จะตรวจจับได้ทันที

---

## ✨ Features

* 🔹 Content-defined chunking (rolling hash)
* 🔹 Per-chunk hash verification (BLAKE3)
* 🔹 Signed index file (Ed25519)
* 🔹 Zero-trust file transfer
* 🔹 Works over standard HTTP
* 🔹 No custom server required (as long as Range is supported)

---

## 🧠 Architecture

```
        Publisher                          Client
    ─────────────────                 ─────────────────
    file.bin                           index.bin.idx
       │                                     │
       ▼                                     ▼
  chunker.rs                          verify signature
       │                                     │
       ▼                                     ▼
 hash each chunk                    download via Range
       │                                     │
       ▼                                     ▼
 build signed index                 verify chunk hash
       │                                     │
       ▼                                     ▼
  file.bin.idx                     reconstruct file.bin
```

---

## 🔐 Security Model

hyper_pipe assumes:

* ❌ Server is untrusted
* ❌ Network is untrusted
* ❌ File can be corrupted

hyper_pipe trusts:

* ✅ Signed index file
* ✅ Public key
* ✅ Cryptographic hash

If server:

* sends wrong data
* modifies file
* truncates chunks

→ client will detect hash mismatch and abort

---

## ⚠️ HTTP Server Requirement

Server **must support HTTP Range requests** (`206 Partial Content`).

Supported:

* ✅ nginx
* ✅ apache
* ✅ busybox httpd
* ✅ caddy

Not supported:

* ❌ `python -m http.server`

Test with:

```bash
curl -I -H "Range: bytes=0-1023" http://host/file
```

Must return:

```
HTTP/1.1 206 Partial Content
```

---

## 📦 Build

```bash
cargo build --release
```

Binary:

```bash
./target/release/hyper_pipe_v3
```

---

## 🔑 Usage

### 1️⃣ Generate keypair

```bash
./hyper_pipe_v3 keygen
```

Creates:

```
private.key
public.key
```

---

### 2️⃣ Create index

```bash
./hyper_pipe_v3 index test.bin private.key
```

Creates:

```
test.bin.idx
```

---

### 3️⃣ Serve file (example)

```bash
busybox httpd -f -p 8000
```

---

### 4️⃣ Secure download

```bash
./hyper_pipe_v3 download \
  http://127.0.0.1:8000/test.bin \
  test.bin.idx \
  public.key \
  ./out
```

Output:

```
./out/test.bin
```

---

## 🧪 Verification Example

```bash
sha256sum test.bin out/test.bin
```

Expected:

```
same hash
```

---

## 📁 Project Structure

```
src/
 ├── main.rs
 ├── cli.rs
 ├── chunker.rs
 ├── crypto.rs
 ├── index.rs
 ├── downloader.rs
 └── types.rs
```

---

## 🛣️ Roadmap

* [ ] Parallel chunk download
* [ ] Resume support
* [ ] Zstd compression
* [ ] Encrypted chunks
* [ ] Multi-mirror support
* [ ] Progress bar
* [ ] Server mode
* [ ] P2P transport

---

## 🧩 Why hyper_pipe?

hyper_pipe implements the same design pattern as:

* BitTorrent (piece hash)
* IPFS (content addressing)
* Secure OTA systems
* Nix / OSTree

It proves:

> **You can build secure file distribution on top of insecure transport.**

---

## 📜 License

MIT

---

## 🧑‍💻 Author

Developed by **traphumi**
hyper_pipe V3 — 2026

---

ถัดไปพี่ควรทำ 2 อย่างเพื่อ “อัพเป็น V3 จริง”:

---

## ✅ Step 1: เปลี่ยนเวอร์ชันใน Cargo.toml

```toml
[package]
name = "hyper_pipe_v3"
version = "3.0.0"
edition = "2024"
```

---

## ✅ Step 2: Commit + tag

```bash
git add .
git commit -m "Release hyper_pipe v3.0.0: secure chunked file transfer"
git tag v3.0.0
git push origin main --tags
```

---
