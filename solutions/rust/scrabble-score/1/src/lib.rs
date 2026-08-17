/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut score: u64 = 0;
    for c in word.to_ascii_uppercase().chars() {
        match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => score += 1,
            'D' | 'G' => score += 2,
            'B' | 'C' | 'M' | 'P' => score += 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => score += 4,
            'K' => score += 5,
            'J' | 'X' => score += 8,
            'Q' | 'Z' => score += 10,
            _ => (),
        }
    }
    score
}
