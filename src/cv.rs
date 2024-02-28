use std::fmt;
use std::fmt::{Formatter};
use std::path::PathBuf;
use opencv::core::{CV_32FC1, Mat, MatExprResult, MatExprTraitConst, MatTraitConst, min_max_loc, no_array, NORM_MINMAX, normalize, Point, Rect, Scalar};
use opencv::imgcodecs::imwrite;
use opencv::imgproc::{match_template, rectangle, TM_SQDIFF, TM_SQDIFF_NORMED};
use crate::io;

pub struct Templated {
    pub haystack: Mat,
    pub needle: Mat,
}

#[derive(Debug, Clone)]
enum ErrorKind {
    ImageSize,
    Match,
    Normalization,
    MatrixConvert,
    Mark,
    Write,
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    path: Option<PathBuf>,
    message: Option<String>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::ImageSize => {
                write!(f, "validating image size")
            }
            ErrorKind::MatrixConvert => {
                write!(f, "loading into matrix ({})", self.path.as_ref().unwrap().display())
            }
            ErrorKind::Match => {
                write!(f, "matching template because {}", self.message.as_ref().unwrap())
            }
            ErrorKind::Normalization => {
                write!(f, "normalizing because {}", self.message.as_ref().unwrap())
            }
            ErrorKind::Mark => {
                write!(f, "marking the haystack because {}", self.message.as_ref().unwrap())
            }
            ErrorKind::Write => {
                write!(f, "writing ({})", self.path.as_ref().unwrap().display())
            }
        }
    }
}

impl Error {
    pub(self) fn new_file(path: PathBuf, kind: ErrorKind) -> Error {
        return Error { path: Some(path), kind, message: None };
    }
    pub(self) fn new_generic(kind: ErrorKind) -> Error {
        return Error { path: None, kind, message: None };
    }
    pub(self) fn new_messaged(message: String, kind: ErrorKind) -> Error {
        return Error { path: None, kind, message: Some(message) };
    }
}

impl Templated {
    pub fn from_files(haystack: PathBuf, needle: PathBuf) -> Result<Templated, Error> {
        let haystack_mat = match io::to_mat(haystack.clone()) {
            Ok(mat) => mat,
            Err(_) => {
                return Err(Error::new_file(haystack, ErrorKind::MatrixConvert));
            }
        };
        let needle_mat = match io::to_mat(needle.clone()) {
            Ok(mat) => mat,
            Err(_) => {
                return Err(Error::new_file(needle, ErrorKind::MatrixConvert));
            }
        };
        if haystack_mat.rows() < needle_mat.rows() || haystack_mat.cols() < needle_mat.cols() {
            return Err(Error::new_generic(ErrorKind::ImageSize));
        }
        return Ok(Templated { haystack: haystack_mat, needle: needle_mat });
    }
}

pub trait GetPosition {
    fn get_positions(&self, threshold: f32) -> Result<Vec<(Point, f32)>, Error>;
    fn get_best_position(&self, threshold: f32) -> Result<Option<(Point, f32)>, Error>;
}

impl Templated {
    fn normalized_result(&self) -> Result<Mat, Error> {
        let mut result = unsafe {
            Mat::new_rows_cols(self.haystack.rows() - self.needle.rows() + 1, self.haystack.cols() - self.needle.cols() + 1, CV_32FC1)
                .unwrap()
        };

        match match_template(&self.haystack, &self.needle, &mut result, TM_SQDIFF_NORMED, &no_array()) {
            Ok(_) => {
                let mut normalized =
                    unsafe { Mat::new_rows_cols(result.rows(), result.cols(), CV_32FC1).unwrap() };
                normalize(&mut result, &mut normalized, 0.0, 1.0, NORM_MINMAX, -1, &no_array())
                    .and_then(|_| {
                        match Mat::ones(normalized.rows(), normalized.cols(), normalized.typ()).unwrap() - normalized {
                            MatExprResult::Ok(mat) => Ok(mat.to_mat().unwrap()),
                            MatExprResult::Err(e) => Err(e)
                        }
                    })
                    .map_err(|e| { Error::new_messaged(e.message, ErrorKind::Normalization) })
            }
            Err(e) => Err(Error::new_messaged(e.message, ErrorKind::Match))
        }
    }

    pub fn mark(&mut self, position: Point) -> Result<(), Error> {
        rectangle(&mut self.haystack, Rect::new(position.x, position.y, self.needle.cols(), self.needle.rows()), Scalar::all(0.0), 2, 8, 0)
            .map_err(|e| { Error::new_messaged(e.message, ErrorKind::Mark) })
    }

    pub fn write(&self, path: &PathBuf) -> Result<bool, Error> {
        if let Some(filename) = path.to_str() {
            imwrite(filename, &self.haystack, &opencv::core::Vector::<i32>::new())
                .map_err(|_| {
                    Error::new_file(path.clone(), ErrorKind::Write)
                })
        } else {
            Err(Error::new_file(path.clone(), ErrorKind::Write))
        }
    }
}

impl GetPosition for Templated {
    fn get_positions(&self, threshold: f32) -> Result<Vec<(Point, f32)>, Error> {
        self.normalized_result()
            .and_then(|normalized| {
                let mut filtered: Vec<(Point, f32)> = vec![];
                
                for (p, v) in normalized.iter::<f32>().unwrap() {
                    if v > threshold {
                        filtered.push((p, v))
                    }
                }
                Ok(filtered)
            })
    }

    fn get_best_position(&self, threshold: f32) -> Result<Option<(Point, f32)>, Error> {
        self.normalized_result().and_then(|normalized| {
            let mut max_loc = Point::new(-1, -1);
            let mut max_value: f64 = 0.0;
            match min_max_loc(&normalized, None, Some(&mut max_value), None, Some(&mut max_loc), &no_array()) {
                Ok(_) =>
                    Ok({
                        if max_value as f32 > threshold {
                            Some((max_loc, max_value as f32))
                        } else {
                            None
                        }
                    }),
                Err(e) => Err(Error::new_messaged(e.message, ErrorKind::Match))
            }
        })
    }
}