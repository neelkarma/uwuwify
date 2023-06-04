/// Uwuwifies the provided string slice.
///
/// Returns an owned string with the uwuwified text.
pub fn uwuwify(input: &str) -> String {
    if input.len() == 0 {
        return String::new();
    }

    let mut out = String::new();
    let mut prev = input.chars().nth(0).unwrap();

    out.push(prev);

    for char in input.chars().skip(1) {
        match (prev, char) {
            (_, 'L' | 'R') => out.push('W'),
            (_, 'l' | 'r') => out.push('w'),
            ('M' | 'm' | 'N' | 'n', 'O' | 'o') => out.push_str("yo"),
            _ => out.push(char),
        }

        prev = char;
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
