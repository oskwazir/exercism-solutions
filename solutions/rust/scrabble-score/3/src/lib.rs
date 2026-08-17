const LETTER_VALUES: [u64; 26] = [
    1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, // A-M
    1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10, // N-Z
];

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| LETTER_VALUES[(c.to_ascii_uppercase() as u8 - b'A') as usize])
        .sum()
}
