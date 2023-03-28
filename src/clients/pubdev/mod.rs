pub mod pubspec_yaml;

use async_trait::async_trait;
use tokio::fs;

use self::pubspec_yaml::{PubDevJson, Pubspec};

use super::ReadFromFile;

pub struct PubDevClient {
    pub base_url: String,
}

impl PubDevClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://pub.dev/api".to_string(),
        }
    }

    pub fn with_base_url(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn get_package(
        &self,
        package_name: &str,
        version: &str,
    ) -> Result<PubDevJson, reqwest::Error> {
        let url = format!(
            "{}/packages/{}/versions/{}",
            self.base_url, package_name, version
        );
        let response = reqwest::get(&url).await?.json::<PubDevJson>().await?;
        Ok(response)
    }
}

#[async_trait]
impl ReadFromFile for PubDevClient {
    type T = Pubspec;

    async fn from_file(&self, file: &str) -> anyhow::Result<Self::T> {
        let file = fs::read_to_string(file).await?;
        let json = serde_yaml::from_str::<Pubspec>(file.as_str())?;
        Ok(json)
    }
}
