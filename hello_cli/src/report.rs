use crate::errors::AppError;
use std::fs;

pub struct Report {
    pub filename: String,
    pub content: String,
    pub line_count: usize,
}

pub enum LineKind {
    Empty,
    Comment,
    Code,
}

impl Report {
    pub fn new(filename: &str, content: String) -> Report {
        let line_count = content.lines().count();

        Report {
            filename: filename.to_string(),
            content,
            line_count,
        }
    }

    pub fn summary(&self) -> String {
        format!("{}：共 {} 行", self.filename, self.line_count)
    }

    pub fn add_title(&mut self) {
        let title = format!("=== {} ===\n", self.filename);
        self.content.insert_str(0, &title);
    }

    pub fn classify_line(line: &str) -> LineKind {
        if line.trim().is_empty() {
            LineKind::Empty
        } else if line.starts_with("//") {
            LineKind::Comment
        } else {
            LineKind::Code
        }
    }

    pub fn count_kinds(&self) -> (usize, usize, usize) {
        let mut empty_count = 0;
        let mut comment_count = 0;
        let mut code_count = 0;

        for line in self.content.lines() {
            match Report::classify_line(line) {
                LineKind::Empty => empty_count += 1,
                LineKind::Comment => comment_count += 1,
                LineKind::Code => code_count += 1,
            }
        }

        (empty_count, comment_count, code_count)
    }
}

pub fn load_report(filename: &str) -> Result<Report, AppError> {
    let content = fs::read_to_string(filename)?;
    if content.trim().is_empty() {
        Err(AppError::Empty)
    } else {
        Ok(Report::new(filename, content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_line_empty() {
        assert!(matches!(Report::classify_line(""), LineKind::Empty));
    }

    #[test]
    fn classify_line_comment() {
        assert!(matches!(Report::classify_line("// hi"), LineKind::Comment));
    }

    #[test]
    fn classify_line_code() {
        assert!(matches!(
            Report::classify_line("let x = 1;"),
            LineKind::Code
        ));
    }

    #[test]
    fn count_kinds() {
        let report = Report::new("test.txt", "code\n\n// comment\n".to_string());
        let counts = report.count_kinds();

        assert_eq!(counts, (1, 1, 1));
    }
}
