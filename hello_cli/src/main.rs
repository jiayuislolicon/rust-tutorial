mod errors;
mod report;

use errors::AppError;
use std::env;

use report::load_report;

fn run() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();

    let filename = match args.get(1) {
        Some(name) => name,
        None => {
            println!("用法：cargo run -- <檔名>");
            return Ok(());
        }
    };

    let mut report = load_report(filename)?;
    report.add_title();

    println!("{}", report.summary());
    println!("{}", report.content);

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
