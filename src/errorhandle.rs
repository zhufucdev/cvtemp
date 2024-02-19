use std::process::exit;
use crate::cv;

pub trait UnwrapExit<R> {
    fn unwrap_or_exit(self) -> R;
}

impl<R> UnwrapExit<R> for Result<R, cv::Error> {
    fn unwrap_or_exit(self) -> R {
        match self {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error {}", e);
                exit(1);
            }
        }
    }
}