# ADAT-RS

**Active Directory Attack Tool** — Rust-based GUI that generates ready-to-use commands for attacking Domain Controllers in CTF and authorized penetration testing.

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange) ![License: MIT](https://img.shields.io/badge/License-MIT-blue) ![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

Developed and maintained by **BeyazAdam**.

---

## ✨ Features

- **GUI (egui):** Target IP, domain, credentials and LDAP base in one panel; commands listed by tabs.
- **Null mode:** No credentials (crackmapexec, enum4linux, kerbrute, SMB, LDAP, etc.).
- **External commands:** Nmap, DNS, Kerberos, SMB, LDAP, WinRM, Impacket, RDP, BloodHound, etc.
- **Internal commands:** PowerShell one-liners to run on the DC (credential dumping, privesc, enumeration).
- **Copy to clipboard:** Single command or entire tab with one click.
- **Local repo:** Use `{LOCAL_URL}` when there is no internet (local HTTP server).
- Cross-platform (Windows, Linux, macOS).

---

## 🤔 What is this useful for?

ADAT-RS **generates** commands; it does not run them. You copy the generated commands and run them in your terminal.

- Quickly recall Domain Controller enumeration / exploitation steps in CTF and lab environments
- Generate typical tool commands (crackmapexec, impacket, kerbrute, etc.) in one place during pentests
- Keep null session, external and internal (PowerShell) command categories in a single UI

> This tool **only generates commands**. Use the generated commands only in **authorized** environments.

---

## 📁 Wordlists

**ADAT-RS does not ship wordlist files.** Userlist paths used in generated commands (kerbrute, GetNPUsers, nmap krb5-enum-users, etc.) are set in the **Wordlists** section of the UI.

- **Default paths** (Linux / Kali): Point to a [SecLists](https://github.com/danielmiessler/SecLists) installation:
  - `Userlist:` `/usr/share/seclists/Usernames/Names/names.txt`
  - `Userlist (xato):` `/usr/share/seclists/Usernames/xato-net-10-million-usernames.txt`
- **Installing SecLists (example):**
  ```bash
  git clone --depth 1 https://github.com/danielmiessler/SecLists.git /usr/share/seclists
  ```
- **Windows or custom setup:** Enter your own file paths in the "Wordlists" field in the UI (e.g. `C:\wordlists\users.txt`).

Commands use these paths as-is; you are responsible for the files existing.

---

## 📦 Installation

- [Rust](https://rustup.rs/) 1.70+ required.

```bash
git clone https://github.com/BeyazAdam/adat-rs.git
cd adat-rs
cargo build --release
```

Binary:

```
target/release/adat-rs.exe   # Windows
target/release/adat-rs      # Linux / macOS
```

---

## 🚀 Usage

### Running

```bash
# Windows
.\target\release\adat-rs.exe

# Linux / macOS
./target/release/adat-rs
```

Development: `cargo run`

### UI

1. **Target & credentials (top panel):** Target IP, Domain, optional Username/Password, LDAP base, Local repo (IP/Port), **Wordlists** (userlist paths).
2. **Tabs:** **Null mode** | **External commands** | **Internal commands** — commands are listed with placeholders filled in.
3. **Copy:** **Copy** for a single command; **Copy all** to copy the whole tab to the clipboard.

### Placeholders

| Placeholder       | Description |
|-------------------|-------------|
| `{USER}` / `{NQUSER}` | Username (quoted / unquoted) |
| `{PASS}`          | Password |
| `{DOMAIN}` / `{NQDOMAIN}` | Domain |
| `{IP}` / `{NQIP}` | Target IP |
| `{LDAP}`          | LDAP base |
| `{USERLIST}` / `{USERLIST_XATO}` | Wordlist file paths (configurable) |
| `{LOCAL_URL}`     | Local HTTP repo (e.g. `http://10.10.14.10:8080/`) |

---

## 🧪 Development

```bash
cargo fmt
cargo test
cargo clippy -- -D warnings
```

---

## 📄 License

MIT License © 2026 **BeyazAdam** — see [LICENSE](LICENSE) for details.

---

---

# ADAT-RS (Türkçe)

**Active Directory Attack Tool** — CTF ve yetkili penetrasyon testleri için Domain Controller’a karşı kullanılacak hazır komutları üreten, Rust ile yazılmış GUI uygulaması.

Bu proje **BeyazAdam** tarafından geliştirilmiş ve sürdürülmektedir.

---

## ✨ Özellikler

- **GUI (egui):** Hedef IP, domain, kullanıcı/parola ve LDAP base tek ekrandan; komutlar sekmelere göre listelenir.
- **Null mod:** Kimlik bilgisi olmadan (crackmapexec, enum4linux, kerbrute, SMB, LDAP vb.).
- **Dış komutlar:** Nmap, DNS, Kerberos, SMB, LDAP, WinRM, Impacket, RDP, BloodHound vb.
- **İç komutlar:** DC üzerinde çalışacak PowerShell one-liner’ları (credential dumping, privesc, enumeration).
- **Panoya kopyalama:** Tek komut veya sekmenin tümü tek tıkla.
- **Yerel repo:** İnternet yokken `{LOCAL_URL}` ile yerel HTTP sunucu adresi kullanımı.
- Windows, Linux ve macOS uyumlu.

---

## 🤔 Bu proje ne işe yarar?

ADAT-RS hazır komut **üretir**; komutları sizin yerinize çalıştırmaz. Üretilen komutları kopyalayıp kendi terminalinizde kullanırsınız.

- CTF ve lab ortamlarında DC enumeration / exploitation adımlarını hızlıca hatırlamak
- Pentest sırasında kullanılan araçların (crackmapexec, impacket, kerbrute vb.) tipik komutlarını tek yerden üretmek
- Null session, dış komut ve iç (PowerShell) komut kategorilerini tek arayüzde toplamak

> Bu araç **sadece komut üreticidir**. Üretilen komutları yalnızca **yetkili** ortamlarda kullanın.

---

## 📁 Wordlist’ler

**ADAT-RS ile wordlist dosyası gelmez.** Kerbrute, GetNPUsers, nmap krb5-enum-users gibi komutlarda kullanılan userlist yolları arayüzde **Wordlist’ler** bölümünden ayarlanır.

- **Varsayılan yollar** (Linux / Kali): [SecLists](https://github.com/danielmiessler/SecLists) kurulumuna işaret eder.
- **SecLists kurulumu (örnek):**
  ```bash
  git clone --depth 1 https://github.com/danielmiessler/SecLists.git /usr/share/seclists
  ```
- **Windows veya özel kurulum:** Arayüzdeki “Wordlist’ler” alanına kendi dosya yollarınızı yazın.

Komutlar bu yolları olduğu gibi kullanır; dosyaların var olduğundan siz sorumlusunuz.

---

## 📦 Kurulum

- [Rust](https://rustup.rs/) 1.70+ gerekir.

```bash
git clone https://github.com/BeyazAdam/adat-rs.git
cd adat-rs
cargo build --release
```

Çalıştırılabilir dosya: `target/release/adat-rs.exe` (Windows) veya `target/release/adat-rs` (Linux/macOS).

---

## 🚀 Kullanım

### Çalıştırma

```bash
# Windows
.\target\release\adat-rs.exe

# Linux / macOS
./target/release/adat-rs
```

Geliştirme: `cargo run`

### Arayüz

1. **Hedef ve kimlik bilgileri (üst panel):** Hedef IP, Domain, isteğe bağlı Kullanıcı/Parola, LDAP base, Yerel repo (IP/Port), **Wordlist’ler** (userlist yolları).
2. **Sekmeler:** **Null mod** | **Dış komutlar** | **İç komutlar** — komutlar placeholder’larla doldurulmuş şekilde listelenir.
3. **Kopyalama:** Satırda **Kopyala** ile tek komut; **Tümünü kopyala** ile o sekmenin tüm komutları panoya gider.

---

## 🛠️ Geliştirme

```bash
cargo fmt
cargo test
cargo clippy -- -D warnings
```

---

## 📄 Lisans

MIT Lisansı © 2026 **BeyazAdam** — ayrıntılar için [LICENSE](LICENSE) dosyasına bakın.
