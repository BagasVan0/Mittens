use std::fs;
use std::process::Command;

pub fn md5_hash(password: &str) -> String {
    format!("{:x}", md5::compute(password.as_bytes()))
}

pub fn dictionary_attack(hash: &str, wordlist: &str, timeout_secs: u64) -> bool {
    let hash_file = tempfile::NamedTempFile::new().unwrap();
    fs::write(hash_file.path(), format!("{}\n", hash)).unwrap();

    let pot_file = tempfile::NamedTempFile::new().unwrap();

    let status = Command::new("hashcat")
        .arg("-m").arg("0")
        .arg("-a").arg("0")
        .arg(hash_file.path())
        .arg(wordlist)
        .arg("--potfile-path").arg(pot_file.path())
        .arg("--runtime").arg(timeout_secs.to_string())
        .arg("--quiet")
        .status();

    let _ = status;

    fs::metadata(pot_file.path())
        .map(|m| m.len() > 0)
        .unwrap_or(false)
}

pub fn mask_attack(hash: &str, password_len: usize, timeout_secs: u64) -> bool {
    let masks: Vec<&str> = match password_len {
        0..=6 => vec!["?l?l?l?l?d?d", "?u?l?l?l?d?d"],
        7..=8 => vec!["?l?l?l?l?l?d?d?d", "?u?l?l?l?l?d?d?d"],
        _ => vec![],
    };

    for mask in masks {
        let hash_file = tempfile::NamedTempFile::new().unwrap();
        fs::write(hash_file.path(), format!("{}\n", hash)).unwrap();

        let pot_file = tempfile::NamedTempFile::new().unwrap();

        let status = Command::new("hashcat")
            .arg("-m").arg("0")
            .arg("-a").arg("3")
            .arg(hash_file.path())
            .arg(mask)
            .arg("--potfile-path").arg(pot_file.path())
            .arg("--runtime").arg(timeout_secs.to_string())
            .arg("--quiet")
            .status();

        let _ = status;

        if fs::metadata(pot_file.path()).map(|m| m.len() > 0).unwrap_or(false) {
            return true;
        }
    }
    false
}