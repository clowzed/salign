#[derive(Debug, Default, PartialEq, Eq)]
#[allow(clippy::derive_ord_xor_partial_ord)]
pub struct Line {
    code: String,
    comment: String,
}

impl Line {
    pub fn new(code: String, comment: String) -> Self {
        Self { code, comment }
    }

    /// If the line has no unclosed quotes, and the separator is found, return a `Line` with the code
    /// and comment. Otherwise, return `None`
    ///
    /// Arguments:
    ///
    /// * `line`: &String - The line to parse
    /// * `separator`: The character that separates the code from the comment.
    ///
    pub fn parse(line: &str, separator: char) -> Option<Self> {
        if Self::unclosed_quotes(line).is_empty() {
            if let Some(separator_index) = Self::find_separator(line, separator) {
                let line = Line {
                    code: line[..separator_index].trim_end().into(),
                    comment: line[separator_index + 1..].trim().into(),
                };
                return Some(line);
            } else {
                return Some(Line::new(line.trim_end().into(), String::default()));
            }
        }
        None
    }

    /// If the line has no unclosed quotes, then find the first separator that is not inside quotes
    ///
    /// Arguments:
    ///
    /// * `line`: &String - The line to search for the separator
    /// * `separator`: The character that separates the fields in the CSV file.
    ///
    pub fn find_separator(line: &str, separator: char) -> Option<usize> {
        if Self::unclosed_quotes(line).is_empty() {
            let mut stack = vec![];
            let quotes = vec!['\'', '"'];

            for (char_index, current_char) in line.chars().enumerate() {
                if current_char == separator && stack.is_empty() {
                    return Some(char_index);
                }
                if quotes.contains(&current_char) {
                    if !stack.is_empty() && stack.last().unwrap() == &current_char {
                        stack.pop();
                    } else {
                        stack.push(current_char);
                    }
                }
            }
        }
        None
    }

    /// It takes a string and returns a vector of indexes of unclosed quotes
    ///
    /// Arguments:
    ///
    /// * `line`: &String - The line of code that we're checking for unclosed quotes.
    ///
    /// Returns:
    ///
    /// A vector of usize
    pub fn unclosed_quotes(line: &str) -> Vec<usize> {
        let mut stack = vec![];
        let quotes = ['\'', '"'];

        for (index_of_current_char, current_char) in line.chars().enumerate() {
            if quotes.contains(&current_char) {
                if !stack.is_empty() && stack.last().unwrap_or(&(0, ' ')).1 == current_char {
                    stack.pop();
                } else {
                    stack.push((index_of_current_char, current_char));
                }
            }
        }
        stack.iter().map(|item| item.0).collect()
    }

    pub fn code(&self) -> &String {
        &self.code
    }

    pub fn comment(&self) -> &String {
        &self.comment
    }

    pub fn has_code(&self) -> bool {
        !self.code.is_empty()
    }

    pub fn has_comment(&self) -> bool {
        !self.comment.is_empty()
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.code.len().cmp(&other.code.len())
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod line_tests {
    use crate::line::Line;

    #[test]
    fn test_parse() {
        let empty_line = "".to_string();
        let parsed = Line::parse(&empty_line, ';');
        assert!(parsed.is_some());
        assert_eq!(Line::default(), parsed.unwrap());

        let tabs_and_spaces = "                ".to_string();
        let parsed = Line::parse(&tabs_and_spaces, ';');
        assert!(parsed.is_some());
        assert_eq!(Line::default(), parsed.unwrap());

        let code_only = "  mov ax, dx ".to_string();
        let parsed = Line::parse(&code_only, ';');
        assert!(parsed.is_some());
        assert_eq!(
            Line::new("  mov ax, dx".into(), String::default()),
            parsed.unwrap()
        );

        let comment_only = "    ; comment    ";
        let parsed = Line::parse(&comment_only, ';');
        assert!(parsed.is_some());
        assert_eq!(
            Line::new(String::default(), "comment".into()),
            parsed.unwrap()
        );

        let comment_with_code = "  mov ax, dx ;  comment".to_string();
        let parsed = Line::parse(&comment_with_code, ';');
        assert!(parsed.is_some());
        assert_eq!(
            Line::new("  mov ax, dx".into(), "comment".into()),
            parsed.unwrap()
        );

        let unclosed_quotes = "mov ' ax, dx; comment".to_string();
        assert!(Line::parse(&unclosed_quotes, ';').is_none());

        let separator_in_closed_quotes = "mov ';' ax, dx ; comment".to_string();
        let parsed = Line::parse(&separator_in_closed_quotes, ';');
        assert!(parsed.is_some());
        assert_eq!(
            Line::parse(&separator_in_closed_quotes, ';').unwrap(),
            Line::new("mov ';' ax, dx".into(), "comment".into())
        );
    }
}
