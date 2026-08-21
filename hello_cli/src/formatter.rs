use crate::report::Report;
use serde::Serialize;

#[derive(Serialize)]
struct ReportJson<'a> {
    filename: &'a str,
    lines: usize,
    empty: usize,
    comment: usize,
    code: usize,
}

pub trait Formatter {
    fn format_report(&self, report: &Report) -> String;
}

pub struct PlainFormatter;

impl Formatter for PlainFormatter {
    fn format_report(&self, report: &Report) -> String {
        let stats = report.line_stats();
        format!("{}\n{}\n{}", report.summary(), report.content, stats)
    }
}

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format_report(&self, report: &Report) -> String {
        let stats = report.line_stats();
        let json = ReportJson {
            filename: &report.filename,
            lines: report.line_count(),
            empty: stats.empty,
            comment: stats.comment,
            code: stats.code,
        };
        serde_json::to_string_pretty(&json).expect("ReportJson should always serialize")
    }
}

pub fn make_formatter(format: &str) -> Option<Box<dyn Formatter>> {
    match format {
        "plain" => Some(Box::new(PlainFormatter)),
        "json" => Some(Box::new(JsonFormatter)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::Report;

    #[test]
    fn plain_formatter_includes_summary() {
        let report = Report::new("test.rs", "let x = 1;\n// hi\n".to_string());
        let output = PlainFormatter.format_report(&report);
        assert!(output.contains("test.rs"));
        assert!(output.contains("2 行"));
    }

    #[test]
    fn json_formatter_produces_valid_structure() {
        let report = Report::new("test.rs", "let x = 1;\n// hi\n\n".to_string());
        let output = JsonFormatter.format_report(&report);
        let value: serde_json::Value =
            serde_json::from_str(&output).expect("output should be valid JSON");
        assert_eq!(value["filename"], "test.rs");
        assert_eq!(value["lines"], 3);
        assert_eq!(value["empty"], 1);
        assert_eq!(value["comment"], 1);
        assert_eq!(value["code"], 1);
    }

    #[test]
    fn make_formatter_returns_none_for_unknown() {
        assert!(make_formatter("xml").is_none());
    }

    #[test]
    fn make_formatter_returns_some_for_known() {
        assert!(make_formatter("plain").is_some());
        assert!(make_formatter("json").is_some());
    }
}
