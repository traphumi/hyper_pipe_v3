---

# 🔐 Security Policy / นโยบายความปลอดภัย

## 📣 Reporting a Vulnerability

## การรายงานช่องโหว่ความปลอดภัย

**EN:**
If you discover a security vulnerability in hyper_pipe, please report it privately.
Do NOT open a public GitHub issue for security-related bugs.

Contact:

* Email: [traphumi@users.noreply.github.com](mailto:traphumi@users.noreply.github.com) (or your preferred contact)

Please include:

* Description of the vulnerability
* Steps to reproduce
* Proof of concept (if available)

**TH:**
หากคุณพบช่องโหว่ด้านความปลอดภัยใน hyper_pipe
กรุณาแจ้งแบบส่วนตัว และ **ห้ามเปิด Issue แบบสาธารณะ**

ติดต่อได้ที่:

* Email: [traphumi@users.noreply.github.com](mailto:traphumi@users.noreply.github.com) (หรืออีเมลที่คุณต้องการ)

โปรดระบุ:

* รายละเอียดของช่องโหว่
* วิธีทำให้เกิดปัญหา (reproduce)
* ตัวอย่างหรือหลักฐาน (ถ้ามี)

---

## 🔐 Security Model

## โมเดลความปลอดภัย

**EN:**
hyper_pipe is designed under a zero-trust model.

Assumptions:

* The HTTP server may be malicious
* The network may be compromised
* The file may be modified

Trusted components:

* Signed index file (.idx)
* Public key used to verify index
* Cryptographic hash (BLAKE3)

The client verifies:

* Digital signature of the index
* Hash of every downloaded chunk

Any mismatch causes the download to abort.

**TH:**
hyper_pipe ถูกออกแบบตามแนวคิด **zero-trust**

สมมติฐาน:

* HTTP server อาจไม่ปลอดภัย
* เครือข่ายอาจถูกดักฟังหรือแก้ไข
* ไฟล์อาจถูกเปลี่ยนแปลง

สิ่งที่เชื่อถือ:

* ไฟล์ index ที่มีลายเซ็น (.idx)
* public key สำหรับตรวจลายเซ็น
* hash แบบ cryptographic (BLAKE3)

ฝั่ง client จะตรวจ:

* ลายเซ็นดิจิทัลของ index
* hash ของทุก chunk ที่ดาวน์โหลด

หากไม่ตรงกัน → การดาวน์โหลดจะถูกยกเลิกทันที

---

## 🧪 Cryptography

## อัลกอริทึมเข้ารหัสที่ใช้

**EN:**

* Hash: BLAKE3
* Signature: Ed25519

These are modern and well-reviewed cryptographic primitives.

**TH:**

* Hash: BLAKE3
* ลายเซ็น: Ed25519

เป็นอัลกอริทึมสมัยใหม่และผ่านการตรวจสอบในวงการความปลอดภัยแล้ว

---

## ⚠️ Limitations

## ข้อจำกัด

**EN:**
hyper_pipe currently does NOT provide:

* Confidentiality (data is not encrypted)
* Anonymity
* Traffic obfuscation

It only guarantees:

* Integrity
* Authenticity

**TH:**
hyper_pipe ยังไม่รองรับ:

* การเข้ารหัสข้อมูล (confidentiality)
* การไม่เปิดเผยตัวตน (anonymity)
* การซ่อนรูปแบบทราฟฟิก

รับประกันเฉพาะ:

* ความถูกต้องของข้อมูล (integrity)
* ความแท้จริงของข้อมูล (authenticity)

---

## 📦 Supported Versions

## เวอร์ชันที่รองรับ

| Version | Supported |
| ------- | --------- |
| 3.x     | ✅         |
| < 3.0   | ❌         |

| เวอร์ชัน    | รองรับ |
| ----------- | ------ |
| 3.x         | ✅      |
| ต่ำกว่า 3.0 | ❌      |

---

## 🧭 Future Improvements

## แผนพัฒนาในอนาคต

**EN:**
Planned:

* Encrypted chunks
* Authenticated server mode
* Replay protection
* Key rotation

**TH:**
แผนในอนาคต:

* เข้ารหัสข้อมูลแต่ละ chunk
* โหมด server ที่มีการยืนยันตัวตน
* ป้องกัน replay attack
* ระบบเปลี่ยน key (key rotation)

---

## 🧑‍💻 Maintainer

## ผู้ดูแลโครงการ

Maintained by: **traphumi**
hyper_pipe V3
