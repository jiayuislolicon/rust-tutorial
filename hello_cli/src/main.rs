mod errors;
mod report;

use errors::AppError;
use std::env;

use report::load_report;

fn run() -> Result<(), AppError> {
    let mut filename: Option<String> = None;
    let mut count_only: bool = false;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--count-only" => count_only = true,
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
    report.add_title();

    println!("{}", report.summary());

    if !count_only {
        println!("{}", report.content);
    }

    let (empty, comment, code) = report.count_kinds();
    println!("空行 {} 行，註解 {} 行，程式碼 {} 行", empty, comment, code);

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
