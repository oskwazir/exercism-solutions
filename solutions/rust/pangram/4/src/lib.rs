/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut seen: u32 = 0;
    for c in sentence.chars().filter(|c| c.is_ascii_alphabetic()) {
        seen |= 1 << (c.to_ascii_lowercase() as u32 - 'a' as u32);
    }
    seen == 0x3FFFFFF // 26 ones
}
