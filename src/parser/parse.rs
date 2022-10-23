use crate::actions::Actions;
use crate::errors::{Error, Result as RsreResult};
use std::ffi::OsString;
use std::path::PathBuf;

pub fn parse(text: OsString) -> RsreResult<Actions> {
    if text.is_empty() {
        Ok(Actions::Help)
    } else {
        let lossy_text = text.to_string_lossy().to_string();
        let mut feilds = lossy_text.split_ascii_whitespace();
        let first = feilds.next().expect("Was checked above");

        match (first, first.chars().next().expect("it's not empty")) {
            ("--version" | "-V", _) => Ok(Actions::Version),
            ("--help" | "-h", _) => Ok(Actions::Help),
            (_, '-') => Err(Error::Parser(format!("Invalid flag `{first}`"))),
            _ => {
                let path = PathBuf::from(first);
                if path.exists() {
                    if let Some(new_name) = feilds.next() {
                        Ok(Actions::Rename {
                            path,
                            new_name: new_name.to_owned(),
                        })
                    } else {
                        Err(Error::Parser(format!(
                            "Enter the new name of `{}`",
                            path.display()
                        )))
                    }
                } else {
                    Err(Error::FileSystem(format!(
                        "Could not find file/directory `{}`",
                        path.display()
                    )))
                }
            }
        }
    }
}
