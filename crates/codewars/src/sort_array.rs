pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds = arr.iter().cloned().filter(|x| x % 2 != 0).collect::<Vec<i32>>();
    odds.sort();
    let mut odds_iter = odds.into_iter();
    arr.iter().cloned().map(| x| if x % 2 != 0 {odds_iter.next().unwrap()} else {x}).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}