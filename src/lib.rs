use colored::Colorize as _;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    FileSystem(std::io::Error),
    Serialization(serde_yml::Error),
    Deserialization(serde_yml::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Error");
        match self {
            Self::FileSystem(error) => {
                s.field("FileSystem", error);
            }
            Self::Serialization(error) => {
                s.field("Serialization", error);
            }
            Self::Deserialization(error) => {
                s.field("Deserialization", error);
            }
        }
        s.finish()
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Config: Default + Serialize + for<'de> Deserialize<'de> {
    const NAME: &'static str;
    fn load(dir: impl AsRef<std::path::Path>) -> impl Future<Output = Result<Self>> {
        async move {
            let path = dir.as_ref().join(Self::NAME).with_extension("yaml");
            if !tokio::fs::try_exists(&*path)
                .await
                .map_err(Error::FileSystem)?
            {
                let this = Self::default();
                let mut file = tokio::fs::File::create(&*path)
                    .await
                    .map_err(Error::FileSystem)?;
                file.write_all(
                    serde_yml::to_string(&this)
                        .map_err(Error::Serialization)?
                        .as_bytes(),
                )
                .await
                .map_err(Error::FileSystem)?;
                log::warn!(
                    "'{}' config not found by path: {}",
                    Self::NAME.bold().cyan(),
                    path.display()
                );
                log::debug!(
                    "'{}' config was automaticly created by path: {}",
                    Self::NAME.bold().cyan(),
                    path.display()
                );

                Ok(this)
            } else {
                let this = serde_yml::from_str(
                    &tokio::fs::read_to_string(&*path)
                        .await
                        .map_err(Error::FileSystem)?,
                )
                .map_err(Error::Deserialization)?;
                log::trace!("'{}' config was loaded", Self::NAME.bold().cyan());
                Ok(this)
            }
        }
    }
}
