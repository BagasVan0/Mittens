**Mittens By Kalikoo**

Mittens is a Rust-based CLI password strength checker that uses **hashcat** as its core testing engine. Combining dictionary attacks, mask attacks, structural pattern detection, leetspeak normalization, and entropy scoring to evaluate how strong a password really is.

>  **Defensive tool, not an ATTACK tool.** Mittens is built to help anyone check the strength of their own password before using it — much like the password strength meters found in many apps, just as a CLI version with deeper analysis. Mittens is not intended, and should not be used, to attempt to crack someone else's password without permission.

---

## How It Works

Every password entered goes through several checks in sequence (cascading) — as soon as one layer finds a weakness, that becomes the final result:

1. **Dictionary attack** (hashcat) — matches the password's hash against real breach wordlists (rockyou.txt, SecLists)
2. **Mask attack** (hashcat) — fast brute-force for short patterns that might not be in any wordlist
3. **Pattern detection** — detects keyboard walks (`qwerty`, `asdf`), sequential characters (`abcd`, `1234`), and repetition (`aaaa`, `abab`)
4. **Leetspeak normalization** — normalizes common substitutions (`@`→`a`, `0`→`o`, etc.) and re-checks against the dictionary, so `P@ssw0rd` still gets caught even though it's not a literal match
5. **Entropy scoring** — if the password survives every layer above, a score is calculated based on length and character variety

The reasoning behind this design: simply "not being in a breach database" doesn't mean a password is strong. Passwords typed carelessly — keyboard patterns, repetition — can still be guessed through structure alone, not just database matching. That's why Mittens doesn't stop at a single type of check.

---

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or later)
- [hashcat](https://hashcat.net/hashcat/) 

---

## Wordlist Setup

Mittens needs wordlists containing real breached passwords to run dictionary attacks. Wordlists are **not bundled in this repo** (they're large, and best downloaded from their original sources). p.s. you'll need to set them up manually before running Mittens.

### 1. Create a `wordlists/` folder at the project root

### 2. Download rockyou.txt

This wordlist contains ~14 million real passwords from the 2009 RockYou data breach (see the Credits section for context). Common sources:

- Via the SecLists repo (look for `rockyou.txt.tar.gz` under `Passwords/Leaked-Databases/`, then `tar -xzf` after downloading)
- Bundled by default on Kali Linux at `/usr/share/wordlists/rockyou.txt.gz`

### 3. Download an additional wordlist from SecLists

Open the `Passwords/Common-Credentials` folder in the SecLists repo (https://github.com/danielmiessler/SecLists), pick one of the wordlist files there (Please check what's actually available, since filenames can change over time)

(Adjust the filename in `src/main.rs`'s config if you save it under a different name.)

>  **Note for people new to security/networking:** downloading breach wordlists like this is an extremely common and legal practice in cybersecurity — used by pentesters, researchers, and developers of defensive tools (like password strength checkers) to test how guessable a password is. This is **different** from accessing or redistributing private breach data that hasn't been publicly released, or using it to try to log into someone else's accounts without permission — that's illegal and outside the scope of this tool.

---

## Project Structure

```
mittens/
├── Cargo.toml
├── src/
│   ├── main.rs        # entry point & cascading flow
│   ├── hashcat.rs      # hashcat wrapper (dictionary & mask attack)
│   ├── pattern.rs       # keyboard walk, sequential, repetitive detection
│   ├── leet.rs          # leetspeak normalization
│   ├── entropy.rs       # entropy calculation
│   └── learn.rs         # archive of passwords that passed every layer
└── wordlists/            # (not committed, see Wordlist Setup above)
```

---

## Credits

Mittens wouldn't work without wordlists from the following projects:

- **SecLists** (https://github.com/danielmiessler/SecLists) — maintained by **Daniel Miessler** and **Jason Haddix**, with contributions from **g0tmi1k**. SecLists is the most comprehensive publicly available collection of security assessment wordlists, covering passwords, usernames, URLs, and much more. See their CONTRIBUTORS.md for the full list of contributors.

- **rockyou.txt** — this wordlist originates from the 2009 RockYou data breach, a company that stored user passwords in clear text. It isn't the work of a developer who published it voluntarily — it's breach victim data that later spread widely through the security community as research and testing material, and is now one of the most widely referenced wordlists in password security, redistributed through both SecLists and Kali Linux.

- **hashcat** (https://hashcat.net/hashcat/) — the password cracking/testing engine at the core of Mittens' logic.

---

## Disclaimer

1. Mittens is built for educational purposes and checking the strength of your own passwords. Do not use it to attempt to crack accounts or systems belonging to others without explicit permission — that's illegal in nearly every jurisdiction.

2. I might update this tools from time to time. But when? Who knows. Only God knows...
