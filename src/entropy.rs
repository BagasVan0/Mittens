pub fn entropy_bits(password: &str) -> f64 {
    let mut pool_size: u32 = 0;

    if password.chars().any(|c| c.is_ascii_lowercase()) {
        pool_size += 26;
    }
    if password.chars().any(|c| c.is_ascii_uppercase()) {
        pool_size += 26;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        pool_size += 10;
    }
    if password.chars().any(|c| !c.is_ascii_alphanumeric()) {
        pool_size += 32;
    }

    let pool_size = pool_size.max(1) as f64;
    password.len() as f64 * pool_size.log2()
}

pub fn classify(bits: f64) -> &'static str {
    if bits < 28.0 {
        "Weak"
    } else if bits < 50.0 {
        "Medium"
    } else {
        "Strong"
    }
}