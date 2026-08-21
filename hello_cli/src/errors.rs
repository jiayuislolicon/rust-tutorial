use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("讀不到檔案：{0}")]
    ReadFailed(#[from] std::io::Error),

    #[error("空檔案")]
    Empty,

    #[error("未知旗標：{0}")]
    UnknownFlag(String),

    #[error("數字格式錯誤：{0}")]
    BadNumber(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_empty_and_unknown_flag() {
        assert_eq!(AppError::Empty.to_string(), "空檔案");
        assert_eq!(
            AppError::UnknownFlag("--xml".to_string()).to_string(),
            "未知旗標：--xml"
        );
    }

    #[test]
    fn io_error_converts_via_into() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
        let app: AppError = io.into();
        assert!(matches!(app, AppError::ReadFailed(_)));
        assert!(app.to_string().starts_with("讀不到檔案："));
    }
}
