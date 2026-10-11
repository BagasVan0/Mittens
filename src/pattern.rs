const ROWS: [&str; 4] = ["1234567890", "qwertyuiop", "asdfghjkl", "zxcvbnm", ];

pub fn is_keyboard_walk(password: &str) -> bool {
    let lower = password.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();

    if chars.len() < 4 {
        return false;
    }

    for row in ROWS {
        let row_chars: Vec<char> = row.chars().collect();

        for window in chars.windows(4) {
            let positions: Vec<Option<usize>> = window
                .iter()
                .map(|c| row_chars.iter().position(|rc| rc == c))
                .collect();

            if positions.iter().all(|p| p.is_some()) {
                let pos: Vec<usize> = positions.into_iter().map(|p| p.unwrap()).collect();
                let ascending = pos.windows(2).all(|w| w[1] == w[0] + 1);
                let descending = pos.windows(2).all(|w| w[0] == w[1] + 1);
                if ascending || descending {
                    return true;
                }
            }
        }
    }
    false
}

pub fn is_sequential(password: &str) -> bool {
    let bytes: Vec<u8> = password.bytes().collect();
    if bytes.len() < 4 {
        return false;
    }

    bytes.windows(4).any(|w| {
        let ascending = w.windows(2).all(|p| p[1] == p[0] + 1);
        let descending = w.windows(2).all(|p| p[0] == p[1] +1);
        ascending || descending
    })
}

pub fn is_repetitive(password: &str) -> bool {
    let chars: Vec<char> = password.chars().collect();
    let n = chars.len();
    if n< 4 {
        return false;
    }

    if chars.windows(4).any(|w| w.iter().all(|&c| c == w[0])) {
        return true;
    }

    for period in 1..=(n / 2) {
        if n % period == 0 {
            let repeated = (period..n).all(|i| chars[i] == chars[i % period]);
            if repeated {
                return true;
            }
        }
    }
    false
}

pub fn detect_weak_pattern(password: &str) -> Option<&'static str> {
    if is_keyboard_walk(password) {
        return Some("keyboard pattern (qwerty/asdf/dll)");
    }
    if is_sequential(password) {
        return Some("sequential characters (abcd/1234)");
    }
    if is_repetitive(password) {
        return Some("reduplicatives (aaaa/abab/123123)");
    }
    None
}