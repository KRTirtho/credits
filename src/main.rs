pub mod clients;
pub mod credit;
use clients::{npm::NpmClient, pubdev::PubDevClient, ReadFromFile};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let npm = NpmClient::new();
    let pubdev = PubDevClient::new();

    let json_file = npm.from_file("sample/package.json").await?;
    let yaml_file = pubdev.from_file("sample/pubspec.yaml").await?;
    

    
    Ok(())
}
