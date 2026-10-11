pub fn normalize(password: &str) -> String {
    let replaced: String = password.chars().map(|c| match c {
        '@' => 'a',
        '4' => 'a',
        '3' => 'e',
        '0' => 'o',
        '1' => 'i',
        '!' => 'i',
        '5' => 's',
        '$' => 's',
        '7' => 't',
        _ => c,
    }).collect();
    
    replaced.to_lowercase()
}