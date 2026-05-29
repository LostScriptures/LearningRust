pub fn parse(code: &str) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    let mut num: i32 = 0;

    code.chars().for_each(|op| match op {
        'i' => num = num.wrapping_add(1),
        'd' => num = num.wrapping_sub(1),
        's' => num = num.pow(2),
        'o' => out.push(num),
        _ => {}
    });

    out
}

#[cfg(test)]
mod example_programs {
    use super::parse;

    #[test]
    fn example_iiisdoso() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
    }

    #[test]
    fn example_iiisxxxdoso() {
        assert_eq!(parse("iiisxxxdoso"), vec![8, 64]);
    }
}
