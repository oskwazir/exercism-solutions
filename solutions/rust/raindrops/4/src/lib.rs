

pub fn raindrops(n: u32) -> String {
    let sound = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter(|(factor, _)| n % factor == 0)
        .map(|(_, word)| *word)
        .collect::<String>();

    if sound.is_empty() { n.to_string() } else { sound }
}
