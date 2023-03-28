pub mod package_json;
use async_trait::async_trait;
use package_json::PackageJson;
use tokio::fs;

use super::ReadFromFile;

pub struct NpmClient {
    pub base_url: String,
}

impl NpmClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://registry.npmjs.org".to_string(),
        }
    }

    pub fn with_base_url(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn get_package(
        &self,
        package_name: &str
    ) -> Result<PackageJson, reqwest::Error> {
        let url = format!("{}/{}/latest", self.base_url, package_name);
        let resp = reqwest::get(&url)
            .await?
            .json::<PackageJson>()
            .await?;
        Ok(resp)
    }
}

#[async_trait]
impl ReadFromFile for NpmClient {
    type T = PackageJson;

    async fn from_file(&self, file: &str) -> anyhow::Result<Self::T> {
        let file = fs::read_to_string(file).await?;
        let json = serde_json::from_str::<PackageJson>(file.as_str())?;
        Ok(json)
    }
}
