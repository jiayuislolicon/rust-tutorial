use crate::errors::AppError;
use std::collections::BTreeMap;
use std::fs;

pub struct Report {
    pub filename: String,
    pub content: String,
}

#[derive(PartialEq, Debug)]
pub enum LineKind {
    Empty,
    Comment,
    Code,
}

#[derive(PartialEq, Debug)]
pub struct LineStats {
    pub empty: usize,
    pub comment: usize,
    pub code: usize,
}

impl Report {
    pub fn new(filename: &str, content: String) -> Report {
        Report {
            filename: filename.to_string(),
            content,
        }
    }

    pub fn summary(&self) -> String {
        format!("{}：共 {} 行", self.filename, self.line_count())
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

    pub fn line_stats(&self) -> LineStats {
        self.content.lines().fold(
            LineStats {
                empty: 0,
                comment: 0,
                code: 0,
            },
            |mut acc, line| {
                match Report::classify_line(line) {
                    LineKind::Empty => acc.empty += 1,
                    LineKind::Comment => acc.comment += 1,
                    LineKind::Code => acc.code += 1,
                }
                acc
            },
        )
    }

    pub fn line_count(&self) -> usize {
        self.content.lines().count()
    }

    pub fn lines_of_kind(&self, kind: LineKind) -> Vec<&str> {
        self.content
            .lines()
            .filter(|line| Report::classify_line(line) == kind)
            .collect()
    }

    pub fn word_counts(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();

        for word in self.content.split_whitespace() {
            *counts.entry(word.to_string()).or_insert(0) += 1;
        }

        return counts;
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
        assert_eq!(Report::classify_line(""), LineKind::Empty);
    }

    #[test]
    fn classify_line_comment() {
        assert_eq!(Report::classify_line("// hi"), LineKind::Comment);
    }

    #[test]
    fn classify_line_code() {
        assert_eq!(Report::classify_line("let x = 1;"), LineKind::Code);
    }

    #[test]
    fn line_stats() {
        let report = Report::new("test.txt", "code\n\n// comment\n".to_string());

        assert_eq!(
            report.line_stats(),
            LineStats {
                empty: 1,
                comment: 1,
                code: 1
            }
        );
    }

    #[test]
    fn lines_of_kind_filters_correctly() {
        let report = Report::new("test.txt", "code\n\n// comment\nmore code\n".to_string());
        let comments = report.lines_of_kind(LineKind::Comment);
        assert_eq!(comments, vec!["// comment"]);

        let code_lines = report.lines_of_kind(LineKind::Code);
        assert_eq!(code_lines, vec!["code", "more code"]);
    }

    #[test]
    fn word_counts() {
        let report = Report::new("test.txt", "code code // comment".to_string());
        let counts_map = report.word_counts();

        assert_eq!(counts_map.get("code"), Some(&2));
        assert_eq!(counts_map.get("nothing"), None);
        assert_eq!(counts_map.len(), 3);
    }

    #[test]
    fn summary_agrees_with_line_stats() {
        let mut report = Report::new("t.txt", "a\nb\nc\n".to_string());
        report.add_title();

        let line_stats = report.line_stats();
        assert_eq!(
            report.line_count(),
            line_stats.empty + line_stats.comment + line_stats.code
        );
    }
}
