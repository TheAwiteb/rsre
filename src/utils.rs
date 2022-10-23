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

use crate::debug as de;
use crate::errors::{Error, Result as RsreResult};
use std::path::PathBuf;

/// Reanem the path
pub fn rename(path: PathBuf, new_name: String) -> RsreResult<()> {
    // path is already exists, was checked in `Actions::run` instance function
    let new_path = path.with_file_name(new_name);
    if new_path.exists() {
        Err(Error::FileSystem(format!(
            "`{}` already exists",
            new_path.display()
        )))
    } else {
        de! {
            println!("Renaming `{}` to `{}`", path.display(), new_path.display())
        };
        std::fs::rename(path, new_path).map_err(|err| Error::FileSystem(err.to_string()))?;
        Ok(())
    }
}
