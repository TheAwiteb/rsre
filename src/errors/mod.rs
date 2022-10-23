// Copyright 2022 Awiteb <https://github.com/TheAwiteb>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
