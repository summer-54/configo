use serde::{Deserialize, Serialize};

pub trait Config: Default + Serialize + for<'de> Deserialize<'de> {
    const NAME: &'static str;
    fn load(dir: &str) -> impl Future<Output = Self> {
        async move {
            let path = format!("{dir}/{}.yaml", Self::NAME).into_boxed_str();
            if !tokio::fs::try_exists(&*path).await.unwrap() {
                let this = Self::default();

                tokio::fs::write(&*path, serde_yml::to_string(&this).unwrap())
                    .await
                    .unwrap();

                log::warn!("'{}' config not found by path: {path}", Self::NAME);
                log::info!(
                    "{} config was automaticly created by path: {path}",
                    Self::NAME
                );

                this
            } else {
                let this =
                    serde_yml::from_str(&tokio::fs::read_to_string(&*path).await.unwrap()).unwrap();
                log::trace!("{} config was loaded", Self::NAME);
                this
            }
        }
    }
}
