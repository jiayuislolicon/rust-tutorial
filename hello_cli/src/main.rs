mod errors;
mod report;

use crate::report::LineStats;
use errors::AppError;
use std::env;

use report::load_report;

fn run() -> Result<(), AppError> {
    let mut filename: Option<String> = None;
    let mut count_only: bool = false;
    let mut words_count_only: bool = false;
    let mut min: usize = 1;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--count-only" => count_only = true,
            "--words" => words_count_only = true,
            "--min" => match args.next() {
                Some(value) => min = value.parse()?,
                None => return Err(AppError::MissingValue("--min".to_string())),
            },
            _ if arg.starts_with("-") => return Err(AppError::UnknownFlag(arg)),
            _ => filename = Some(arg),
        }
    }

    let filename = match filename {
        Some(name) => name,
        None => {
            println!("用法：cargo run -- [--count-only] <檔名>");
            return Ok(());
        }
    };

    let mut report = load_report(&filename)?;

    if words_count_only {
        let counts = report.word_counts();
        let mut pairs: Vec<(&String, &usize)> = counts.iter().collect();

        pairs.retain(|pair| *pair.1 >= min);
        pairs.sort_by(|a, b| b.1.cmp(a.1));

        for (word, count) in pairs {
            println!("{}: {}", word, count);
        }
        return Ok(());
    }

    report.add_title();

    println!("{}", report.summary());

    if !count_only {
        println!("{}", report.content);
    }

    let LineStats {
        empty,
        comment,
        code,
    } = report.line_stats();
    println!("空行 {} 行，註解 {} 行，程式碼 {} 行", empty, comment, code);

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
