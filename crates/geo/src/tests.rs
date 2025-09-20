#[cfg(test)]
use super::*;

#[test]
fn test_pos() {
    let a = Pos::X(10);
    assert_eq!(a.get_string(), "X(10)");
}
