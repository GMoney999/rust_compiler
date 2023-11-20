use crate::lexer::{remove_excess_whitespaces, remove_comments};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_excess_whitespaces() {
        let input = "
            Some    text  with   excess    spaces.
                Another line     with    spaces.
        ";

        // Include leading and trailing newline and spaces in the expected string
        let expected = "\nSome text with excess spaces.\nAnother line with spaces.\n";

        assert_eq!(remove_excess_whitespaces(input), expected);
    }

    #[test]
    fn test_remove_comments() {
        let input = r#"
        program example;
        (* This is a comment *)
        var a, b: integer;
        (* Another comment *)
        begin
        end.
    "#;

        let expected = "\
        program example;\n\
        var a, b: integer;\n\
        begin\n\
        end.";

        let actual = remove_excess_whitespaces(&remove_comments(input));
        assert_eq!(actual, expected);
    }

}


