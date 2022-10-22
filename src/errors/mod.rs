mod statuses;
pub use statuses::*;
use std::process::Termination;
pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    FileSystem(String),
    Parser(String),
}

impl Error {
    /// Return the error message of error
    pub fn message(&self) -> &str {
        match self {
            Error::FileSystem(e) => e,
            Error::Parser(e) => e,
        }
    }

    /// Print the message of the error
    pub fn print(&self) {
        println!("Rsre: {}", self.message());
    }
}

// To use `Error` with `Statuses`
impl Termination for Error {
    fn report(self) -> std::process::ExitCode {
        self.print();
        match self {
            Error::FileSystem(_) => 77.into(),
            _ => 1.into(),
        }
    }
}
