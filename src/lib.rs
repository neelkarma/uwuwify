/// Uwuwifies the provided string slice.
///
/// Returns an owned string with the uwuwified text.
pub fn uwuwify(input: &str) -> String {
    let mut out = String::new();
    let mut prev = None;

    for char in input.chars() {
        match char {
            'L' | 'R' => out.push('W'),
            'l' | 'r' => out.push('w'),
            // No -> Nyo, only works if first letter is M or N
            'O' | 'o' if prev.map_or(false, |c| "MmNn".contains(c)) => out.push_str("yo"),
            _ => out.push(char),
        }

        prev = Some(char);
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fox() {
        assert_eq!(
            uwuwify("The quick brown fox jumps over the lazy dog."),
            "The quick bwown fox jumps ovew the wazy dog.".to_string()
        );
    }

    #[test]
    fn late_for_work() {
        assert_eq!(
            uwuwify("Nooo! I was not late to work!"),
            "Nyooo! I was nyot wate to wowk!".to_string()
        );
    }
}
