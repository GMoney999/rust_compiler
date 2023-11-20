

pub fn remove_comments(contents: &str) -> String {
    let mut result = String::new();
    let mut in_comment = false;

    for line in contents.lines() {
        let mut processed_line = String::new();
        let mut chars = line.chars().peekable();

        while let Some(&ch) = chars.peek() {
            if in_comment {
                if ch == '*' && chars.nth(1) == Some(')') {
                    in_comment = false;
                }
            } else {
                if ch == '(' && chars.nth(1) == Some('*') {
                    in_comment = true;
                } else {
                    processed_line.push(ch);
                }
            }
            chars.next();
        }

        if !in_comment && !processed_line.trim().is_empty() {
            result.push_str(&processed_line);
            result.push('\n');
        }
    }

    result
}


pub fn remove_excess_whitespaces(contents: &str) -> String {
    contents
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>().join(" "))
        .collect::<Vec<String>>()
        .join("\n")
}

