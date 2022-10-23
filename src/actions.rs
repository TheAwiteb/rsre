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

use crate::errors::Result as RsreResult;
use crate::utils::rename;
#[cfg(feature = "debug")]
use std::fmt::Debug;
use std::path::PathBuf;

const USAGE: &str = r#"
USAGE:
    rsre FILE/DIRECTORY NEW_FULL_NAME

OPTIONS:
    -h, --help     Print help information
    -V, --version  Print version information"#;

#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Actions {
    Help,
    Version,
    Rename { path: PathBuf, new_name: String },
}

impl Actions {
    // Run the action
    pub fn run(self) -> RsreResult<()> {
        match self {
            Self::Help => Ok(println!("{USAGE}")),
            Self::Version => Ok(println!("Rsre {}", env!("CARGO_PKG_VERSION"))),
            Self::Rename { path, new_name } => rename(path, new_name),
        }
    }
}
