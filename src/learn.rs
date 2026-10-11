use std::fs::OpenOptions;
use std::io::Write;

const MITTENS_ARCHIVE: &str = "treasure.txt";

pub fn save_learned(password: &str) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(MITTENS_ARCHIVE);

    if let Ok(mut f) = file {
        let _ = writeln!(f, "{}", password);
    }
}

pub fn learned_file_path() -> &'static str {
    MITTENS_ARCHIVE
}