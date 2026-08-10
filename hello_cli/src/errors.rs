use std::fmt;

pub enum AppError {
    ReadFailed(std::io::Error),
    Empty,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ReadFailed(e) => write!(f, "讀不到檔案：{}", e),
            AppError::Empty => write!(f, "空檔案"),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::ReadFailed(e)
    }
}
