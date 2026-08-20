use crate::report::Report;

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
        format!(
            "{{
  \"filename\": \"{}\",
  \"lines\": {},
  \"empty\": {},
  \"comment\": {},
  \"code\": {}
}}",
            report.filename,
            report.line_count(),
            stats.empty,
            stats.comment,
            stats.code,
        )
    }
}

pub fn make_formatter(format: &str) -> Option<Box<dyn Formatter>> {
    match format {
        "plain" => Some(Box::new(PlainFormatter)),
        "json" => Some(Box::new(JsonFormatter)),
        _ => None,
    }
}
