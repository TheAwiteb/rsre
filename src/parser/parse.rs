use crate::actions::Actions;
use crate::errors::{Error, Result as RsreResult};
use std::path::PathBuf;

pub fn parse(args: Vec<String>) -> RsreResult<Actions> {
    if args.is_empty() {
        Ok(Actions::Help)
    } else {
        let mut args = args.iter().map(String::as_str);
        let first = args.next().expect("Was checked above");

        match (first, first.chars().next().expect("it's not empty")) {
            ("--version" | "-V", _) => Ok(Actions::Version),
            ("--help" | "-h", _) => Ok(Actions::Help),
            (_, '-') => Err(Error::Parser(format!("Invalid flag `{first}`"))),
            _ => {
                let path = PathBuf::from(first);
                if path.exists() {
                    if let Some(new_name) = args.next() {
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
