fn capitalize(s: &str) -> String {
    let mut prev: Option<char> = None;
    let mut caps = String::new();
    for c in s.chars() {
        if prev.is_none() || (prev.unwrap().is_lowercase() && c.is_uppercase()) {
            caps.push(c);
        }
        prev = Some(c);
    }
    caps.to_uppercase()
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-', '_'][..])
        .filter(|s| !s.is_empty())
        .map(capitalize)
        .collect::<String>()
}
