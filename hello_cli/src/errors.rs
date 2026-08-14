use std::fmt;

pub enum AppError {
    ReadFailed(std::io::Error),
    Empty,
    UnknownFlag(String),
    MissingValue(String),
    BadNumber(std::num::ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ReadFailed(e) => write!(f, "讀不到檔案：{}", e),
            AppError::Empty => write!(f, "空檔案"),
            AppError::UnknownFlag(flag) => write!(f, "未知旗標：{}", flag),
            AppError::MissingValue(flag) => write!(f, "{} 後面少了一個數字", flag),
            AppError::BadNumber(e) => write!(f, "數字格式錯誤：{}", e),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::ReadFailed(e)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> AppError {
        AppError::BadNumber(e)
    }
}
