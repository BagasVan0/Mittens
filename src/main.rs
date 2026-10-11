mod pattern;
mod entropy;
mod hashcat;
mod learn;
mod leet;

use std::io::{self, Write};
use std::path::Path;

const WORDLIST: &str = "wordlists/rockyou.txt";
const SECLISTS_DARKWEB: &str = "wordlists/darkweb2017_top-10000.txt";
const SECLISTS_NCSC: &str = "wordlists/100k-most-used-passwords-NCSC.txt";
const TIMEOUT_SECS: u64 = 15;

const SOURCES: &[&str] = &[WORDLIST, SECLISTS_DARKWEB, SECLISTS_NCSC];

fn main() {
    println!("Mittens here! ◥▶‿◀◤");

    loop {
        print!("Please input your password here >> ");
        io::stdout().flush().unwrap();

        let mut password = String::new();
        if io::stdin().read_line(&mut password).unwrap() == 0 {
            break;
        }

        let password = password.trim();
        if password.is_empty() {
            break;
        }

        let hash = hashcat::md5_hash(password);
        if hashcat::dictionary_attack(&hash, WORDLIST, TIMEOUT_SECS) {
            println!("Weak wle! (There's your's on the list!)");
            continue;
        }

        if Path::new(learn::learned_file_path()).exists() {
            if hashcat::dictionary_attack(&hash, learn::learned_file_path(), TIMEOUT_SECS) {
                println!("Weak wle! Also thanks for your contribution!");
                continue;
            }
        }

        if hashcat::mask_attack(&hash, password.len(), TIMEOUT_SECS) {
            println!("Weak wle!");
            continue;
        }

        if let Some(reason) = pattern::detect_weak_pattern(password) {
            println!("Weak wle! ({})", reason);
            continue;
        }

        let normalized = leet::normalize(password);
        if normalized != password {
            let norm_hash = hashcat::md5_hash(&normalized);
            if hashcat::dictionary_attack(&norm_hash, WORDLIST, TIMEOUT_SECS) {
                println!("Weak wle! Cringe");
                continue;
            }
            if let Some(_reason) = pattern::detect_weak_pattern(&normalized) {
                println!("Weak wle! Are you an edgy??");
                continue;
            }
        }

        let bits = entropy::entropy_bits(password);
        let verdict = entropy::classify(bits);
        println!("{} (entropy ~{:.1} bit)", verdict, bits);

        learn::save_learned(password);
    }

    println!("bye! (˵◝ ⩊ ◜˵マ");
}