pub fn maskify(cc: &str) -> String {
    let len = cc.len();
    if len <= 4 {
        return cc.to_string();
    }
    let mut buffer = "#".repeat(len - 4);
    buffer.push_str(&cc[len - 4..len]);
    buffer
}

#[cfg(test)]
mod tests {
    use super::maskify;

    #[test]
    fn it_masks_example_strings() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify("11111"), "#1111");
    }
}
