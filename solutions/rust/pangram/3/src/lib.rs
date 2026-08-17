
/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let lower = sentence.to_ascii_lowercase();
    ('a'..='z').all(|c| lower.contains(c))
}
