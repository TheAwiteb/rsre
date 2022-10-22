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
        std::fs::rename(path, new_path).map_err(|err| Error::FileSystem(err.to_string()))?;
        Ok(())
    }
}
