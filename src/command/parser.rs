use super::Command;

pub struct CommandParser {
    previous_lines: Vec<String>,
}

impl CommandParser {
    pub fn new() -> CommandParser {
        CommandParser {
            previous_lines: vec![],
        }
    }

    pub fn feed_line(&mut self, line: &str) -> FeedResult {
        let line = line.trim();
        let (line, end) = if line.ends_with('\\') {
            (line.trim_end_matches('\\'), false)
        } else {
            (line, true)
        };
        let split_line = line.split_whitespace().map(str::to_string);
        self.previous_lines.extend(split_line);
        if end {
            let result = self.previous_lines.clone();
            self.previous_lines = vec![];
            FeedResult::Command(Command(result))
        } else {
            FeedResult::Continue
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FeedResult {
    Command(Command),
    Continue,
}

#[cfg(test)]
mod tests {
    use crate::command::{
        parser::{CommandParser, FeedResult},
        Command,
    };

    #[test]
    fn test_one_line() {
        let mut parser = CommandParser::new();
        assert_eq!(
            FeedResult::Command(Command(vec!["first".to_string(), "line".to_string()])),
            parser.feed_line("first line\n")
        );
        assert_eq!(
            FeedResult::Command(Command(vec!["second".to_string()])),
            parser.feed_line("second\n")
        );
    }

    #[test]
    fn test_broken_line() {
        let mut parser = CommandParser::new();
        assert_eq!(FeedResult::Continue, parser.feed_line("first\\\n"));
        assert_eq!(
            FeedResult::Command(Command(vec!["first".to_string(), "second".to_string()])),
            parser.feed_line("second\n")
        );
        assert_eq!(
            FeedResult::Command(Command(vec!["third".to_string()])),
            parser.feed_line("third\n")
        );
    }
}
