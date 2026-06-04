use std::sync::Arc;

use dirtybase_contract::prelude::{ConfigResult, Context, DirtyConfig, TryFromDirtyConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TipConfig {
    pub(crate) base_url: Arc<String>,
    pub(crate) user_validation_url: Arc<String>,
}

#[async_trait::async_trait]
impl TryFromDirtyConfig for TipConfig {
    type Returns = Self;
    async fn from_config(config: &DirtyConfig, _ctx: &Context) -> ConfigResult<Self::Returns> {
        Ok(config
            .load_optional_file("tip.toml", Some("TIP"))
            .build()
            .await?
            .try_deserialize()?)
    }
}

#[cfg(test)]
mod test {
    use dirtybase_contract::prelude::global_context;

    use super::*;

    #[tokio::test]
    async fn foo() {
        let config = DirtyConfig::default();
        let ctx = global_context().await;
        let app_config = TipConfig::from_config(&config, &ctx).await;

        println!("{app_config:#?}");
    }
}
