use std::fmt;
use std::process::exit;

pub trait UnwrapExit<R> {
    fn unwrap_or_exit(self) -> R;
}

impl<R, E: fmt::Display> UnwrapExit<R> for Result<R, E> {
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