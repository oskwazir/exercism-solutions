fn capitalize(s: &str) -> String {
    let mut prev: Option<char> = None;
    let mut caps = String::new();
    for c in s.chars() {
        if prev == None {
            caps.push(c);
        }
        if prev != None && prev.unwrap().is_lowercase() && c.is_uppercase() {
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
        .map(|word| capitalize(word))
        .collect::<String>()
}
