/// Indent each line of the input string with the given indent string.
///
/// This function takes a multi-line string and prepends each line with the specified indentation
/// string. It's useful for formatting text output with consistent indentation, such as when
/// generating code or displaying hierarchical data.
pub(crate) fn indent_lines(input: &str, indent: &str) -> String {
    input
        .lines()
        .map(|line| format!("{indent}{line}"))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_indent_lines() {
        let input = "hello\nworld".to_string();
        let indent = "  ";
        let expected = "  hello\n  world";
        assert_eq!(indent_lines(&input, indent), expected);
    }
}
