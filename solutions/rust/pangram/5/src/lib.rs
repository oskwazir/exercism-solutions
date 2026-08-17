/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .fold(0u32, |acc, c| acc | 1 << (c.to_ascii_lowercase() as u32 - 'a' as u32))
        == 0x3FFFFFF
}
