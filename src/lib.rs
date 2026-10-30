pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut arr = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            arr.push(line);
        }
    }

    arr
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut arr = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            arr.push(line);
        }
    }

    arr
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
