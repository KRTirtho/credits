use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub keywords: Option<Vec<String>>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub bugs: Option<PackageJsonBugs>,
    pub license: String,
    pub main: Option<String>,
    pub repository: Option<PackageJsonRepository>,
    pub dependencies: Option<PackageJsonDep>,
    pub dev_dependencies: Option<PackageJsonDep>,
    pub peer_dependencies: Option<PackageJsonDep>,
    pub maintainers: Option<Vec<PackageJsonMaintainer>>,
}

type PackageJsonDep = BTreeMap<String, String>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJsonBugs {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJsonRepository {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub directory: Option<String>,
}




#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJsonMaintainer {
    pub email: String,
    pub name: String,
}
