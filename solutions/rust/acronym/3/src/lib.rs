pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .fold((String::new(), None::<char>), |(mut acc, prev), c| {
            if c == ' ' || c == '-' || c == '_' {
                return (acc, None);
            }
            if prev.is_none() || prev.is_some_and(|p| p.is_lowercase() && c.is_uppercase()) {
                acc.push(c.to_ascii_uppercase());
            }
            (acc, Some(c))
        })
        .0
}
