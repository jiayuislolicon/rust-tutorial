mod errors;
mod formatter;
mod report;

use clap::Parser;
use errors::AppError;

use formatter::make_formatter;
use report::load_report;

/// 分析原始碼檔案的小工具
#[derive(Parser)]
struct Cli {
    /// 要分析的檔案
    filename: Option<String>,

    /// 只印行數摘要
    #[arg(long)]
    count_only: bool,

    /// 只印詞頻統計
    #[arg(long)]
    words: bool,

    /// 詞頻篩選門檻
    #[arg(long, default_value_t = 1)]
    min: usize,

    /// 輸出格式 (plain / json)
    #[arg(long, default_value = "plain")]
    format: String,

    /// 搜尋包含此關鍵字的行
    #[arg(long)]
    grep: Option<String>,
}

fn run() -> Result<(), AppError> {
    let cli = Cli::parse();

    let formatter = make_formatter(&cli.format)
        .ok_or_else(|| AppError::UnknownFlag(format!("--format {}", cli.format)))?;

    let filename = match cli.filename {
        Some(name) => name,
        None => {
            println!("用法：cargo run -- [--count-only] <檔名>");
            return Ok(());
        }
    };

    let report = load_report(&filename)?;

    if let Some(pattern) = &cli.grep {
        for line in report.matching_lines(pattern) {
            println!("{}", line);
        }
        return Ok(());
    }

    if cli.words {
        let counts = report.word_counts();
        let mut pairs: Vec<_> = counts.iter().collect();

        pairs.retain(|pair| *pair.1 >= cli.min);
        pairs.sort_by(|a, b| b.1.cmp(a.1));

        for (word, count) in pairs {
            println!("{}: {}", word, count);
        }
        return Ok(());
    }

    if cli.count_only {
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
