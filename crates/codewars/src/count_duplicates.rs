use std::collections::HashMap;

pub fn count_duplicates(text: &str) -> u32 {
    let mut counts = HashMap::new();
    text.chars().flat_map(|c| c.to_lowercase()).for_each(|c| {
        *counts.entry(c).or_insert(0) += 1;
    });
    counts.values().filter(|&&x| x > 1).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
