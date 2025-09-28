/// Takes a query string and a text and returns an `iterator` over all lines containing the query
/// # Example
/// ```
/// use minigrep::search;
/// let query = "to";
/// let content ="To be,\nor not,\nto be";
/// assert_eq!(vec!["to be"], search(query, content).collect::<Vec<&str>>());
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(query))
}

/// Takes a query string and a text and returns an iterator over all lines containing the query
/// no matter what caese they are in
/// # Example
/// ```
/// use minigrep::search_case_insensitive;
/// let query = "to";
/// let content ="To be,\nor not,\nto be";
/// assert_eq!(vec!["To be,", "to be"], search_case_insensitive(query, content).collect::<Vec<&str>>());
/// ```
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents).collect::<Vec<&str>>()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents).collect::<Vec<&str>>()
        );
    }

    #[test]
    fn long_test() {
        let query = "long";
        let contents = "\
This is a long line.
Short.
Another long example.
Not matching.
A long journey begins with a single step.
Long ago, in a distant land.
The long and winding road.
Short sentence.
Long division is a math operation.
A long list of items:
Item 1
Item 2
Item 3
Item 4
Item 5
Item 6
Item 7
Item 8
Item 9
Item 10
Item 11
Item 12
Item 13
Item 14
Item 15
Item 16
Item 17
Item 18
Item 19
Item 20
Item 21
Item 22
Item 23
Item 24
Item 25
Item 26
Item 27
Item 28
Item 29
Item 30
The long-awaited result is here.
Long live Rust!";
        assert_eq!(
            vec![
                "This is a long line.",
                "Another long example.",
                "A long journey begins with a single step.",
                "Long ago, in a distant land.",
                "The long and winding road.",
                "Long division is a math operation.",
                "A long list of items:",
                "The long-awaited result is here.",
                "Long live Rust!"
            ],
            search_case_insensitive(query, contents).collect::<Vec<&str>>()
        );
    }
}
