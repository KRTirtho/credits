use async_trait::async_trait;

pub mod cargo;
pub mod npm;
pub mod pubdev;

#[async_trait]
pub trait ReadFromFile {
    type T;
    async fn from_file(&self, file: &str) -> anyhow::Result<Self::T>;
}
