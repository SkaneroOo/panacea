use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum PanaceaError {
    #[snafu(display("File '{filename}' does not exist"))]
    FileNotFound { filename: String },
    #[snafu(display("Cannot open file '{filename}'"))]
    CannotOpen { filename: String },
    #[snafu(display("No source file provided"))]
    NoFileProvided
}