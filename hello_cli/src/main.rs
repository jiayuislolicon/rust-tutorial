mod errors;
mod formatter;
mod report;

use errors::AppError;
use std::env;

use formatter::make_formatter;
use report::load_report;

fn run() -> Result<(), AppError> {
    let mut filename: Option<String> = None;
    let mut count_only: bool = false;
    let mut words_count_only: bool = false;
    let mut min: usize = 1;
    let mut args = env::args().skip(1);
    let mut format = String::from("plain");
    let mut grep: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--count-only" => count_only = true,
            "--words" => words_count_only = true,
            "--min" => match args.next() {
                Some(value) => min = value.parse()?,
                None => return Err(AppError::MissingValue("--min".to_string())),
            },
            "--format" => match args.next() {
                Some(value) => format = value,
                None => return Err(AppError::MissingValue("--format".to_string())),
            },
            "--grep" => match args.next() {
                Some(value) => grep = Some(value),
                None => return Err(AppError::MissingValue("--grep".to_string())),
            },
            _ if arg.starts_with("-") => return Err(AppError::UnknownFlag(arg)),
            _ => filename = Some(arg),
        }
    }

    let formatter = make_formatter(&format)
        .ok_or_else(|| AppError::UnknownFlag(format!("--format {}", format)))?;

    let filename = match filename {
        Some(name) => name,
        None => {
            println!("用法：cargo run -- [--count-only] <檔名>");
            return Ok(());
        }
    };

    let report = load_report(&filename)?;

    if let Some(pattern) = &grep {
        for line in report.matching_lines(pattern) {
            println!("{}", line);
        }
        return Ok(());
    }

    if words_count_only {
        let counts = report.word_counts();
        let mut pairs: Vec<_> = counts.iter().collect();

        pairs.retain(|pair| *pair.1 >= min);
        pairs.sort_by(|a, b| b.1.cmp(a.1));

        for (word, count) in pairs {
            println!("{}: {}", word, count);
        }
        return Ok(());
    }

    if count_only {
        println!("{}", report.summary());
    } else {
        println!("{}", formatter.format_report(&report));
    }

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
