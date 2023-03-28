use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PubDevJson {
    pub version: String,
    pub pubspec: Pubspec,
    pub archive_url: String,
    pub archive_sha256: String,
    pub published: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pubspec {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub issue_tracker: Option<String>,
    pub repository: Option<String>,
    pub environment: Option<Environment>,
    pub dependencies: Option<BTreeMap<String, PubDependency>>,
    pub dev_dependencies: Option<BTreeMap<String, PubDependency>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PubDependency {
    String(String),
    Path { path: String },
    Sdk { sdk: String },
    Git { git: GitInfo },
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitInfo {
    pub url: String,
    #[serde(rename = "ref")]
    pub ref_name: Option<String>,
    pub path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    pub sdk: Option<String>,
    pub flutter: Option<String>,
}
