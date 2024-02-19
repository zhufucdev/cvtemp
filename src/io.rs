use std::path::PathBuf;
use opencv::core::{Mat, Point};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

pub fn to_mat(pf: PathBuf) -> Result<Mat, ()> {
    match pf.to_str() {
        None => Err(()),
        Some(str) => {
            match imread(str, IMREAD_COLOR) {
                Ok(mat) => Ok(mat),
                Err(_) => Err(())
            }
        }
    }
}

pub trait Print {
    fn println(&self);
}

impl Print for Point {
    fn println(&self) {
        println!("{}\t{}", self.x, self.y);
    }
}

impl Print for (Point, f32) {
    fn println(&self) {
        println!("{}\t{}\t{}", self.0.x, self.0.y, self.1);
    }
}