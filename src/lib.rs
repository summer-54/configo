use colored::Colorize as _;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncWriteExt, BufWriter};

pub trait Config: Default + Serialize + for<'de> Deserialize<'de> {
    const NAME: &'static str;
    fn load(dir: &str) -> impl Future<Output = Self> {
        async move {
            let path = format!("{dir}/{}.yaml", Self::NAME).into_boxed_str();
            if !tokio::fs::try_exists(&*path).await.unwrap() {
                let this = Self::default();
                let mut file = tokio::fs::File::create(&*path).await.unwrap();
                file.write_all(serde_yml::to_string(&this).unwrap().as_bytes())
                    .await
                    .unwrap();
                log::warn!(
                    "'{}' config not found by path: {path}",
                    Self::NAME.bold().cyan()
                );
                log::debug!(
                    "'{}' config was automaticly created by path: {path}",
                    Self::NAME.bold().cyan()
                );

                this
            } else {
                let this =
                    serde_yml::from_str(&tokio::fs::read_to_string(&*path).await.unwrap()).unwrap();
                log::trace!("'{}' config was loaded", Self::NAME.bold().cyan());
                this
            }
        }
    }
}
