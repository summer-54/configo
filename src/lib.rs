use colored::Colorize as _;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

pub trait Config: Default + Serialize + for<'de> Deserialize<'de> {
    const NAME: &'static str;
    fn load(dir: impl AsRef<std::path::Path>) -> impl Future<Output = Self> {
        async move {
            let path = dir.as_ref().join(Self::NAME).with_extension("yaml");
            if !tokio::fs::try_exists(&*path).await.unwrap() {
                let this = Self::default();
                let mut file = tokio::fs::File::create(&*path).await.unwrap();
                file.write_all(serde_yml::to_string(&this).unwrap().as_bytes())
                    .await
                    .unwrap();
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
