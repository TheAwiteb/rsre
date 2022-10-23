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
