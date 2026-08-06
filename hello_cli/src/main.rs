use std::env;
use std::fs;
use std::fmt;

struct Report {
    filename: String,
    content: String,
    line_count: usize,
}

enum LineKind {
    Empty,
    Comment,
    Code
}

enum AppError {
    ReadFailed(std::io::Error),
    Empty
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ReadFailed(e) => write!(f, "讀不到檔案：{}", e),
            AppError::Empty => write!(f, "空檔案"),
        }
    }
}

impl Report {
    fn new(filename: &str, content: String) -> Report {
        let line_count = content.lines().count();


        Report {
            filename: filename.to_string(),
            content,
            line_count,
        }
    }

    fn summary(&self) -> String {
        format!("{}：共 {} 行", self.filename, self.line_count)
    }

    fn add_title(&mut self) {
        let title = format!("=== {} ===\n", self.filename);
        self.content.insert_str(0, &title);
    }

    fn classify_line(line: &str) -> LineKind {
        if line.trim().is_empty() {
            LineKind::Empty
        } else if line.starts_with("//") {
            LineKind::Comment
        } else {
            LineKind::Code
        }
    }

    fn count_kinds(&self) -> (usize, usize, usize) {
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


fn load_report(filename: &str) -> Result<Report, AppError> {
    match fs::read_to_string(filename) {
        Err(e) => Err(AppError::ReadFailed(e)),
        Ok(content) => {
            if content.trim().is_empty() {
                Err(AppError::Empty)
            } else {
                Ok(Report::new(filename, content))
            }

        },
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = match args.get(1) {
        Some(name) => name,
        None => {
            println!("用法：cargo run -- <檔名>");
            return;
        }
    };

    match load_report(filename) {
        Ok(mut report) => {
            report.add_title();

            println!("{}", report.summary());
            println!("{}", report.content);

            let (empty, comment, code) = report.count_kinds();
            println!("空行 {} 行，註解 {} 行，程式碼 {} 行", empty, comment, code);
        },
        Err(error) => println!("{}", error),
    }

}