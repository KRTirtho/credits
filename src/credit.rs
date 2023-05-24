use crate::clients::{
    npm::package_json::{PackageJson, PackageJsonDep},
    pubdev::pubspec_yaml::Pubspec,
};

use async_trait::async_trait;
use octocrab::{self, params::repos::Commitish};
use reqwest::Url;
use serde::Deserialize;

#[async_trait]
trait AsyncFrom<T>: Sized {
    async fn from(value: T) -> Self;
}

#[derive(Deserialize, Debug, Clone)]
struct GitlabResponse {
    description: String,
}

pub struct Credit {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
}

async fn get_github_description(url: String) -> Option<String> {
    let owner = url.split("/").nth(3).unwrap();
    let repo = url.split("/").nth(4).unwrap();
    octocrab::instance()
        .repos(owner, repo)
        .get()
        .await
        .ok()
        .map(|repo| repo.description)
        .unwrap_or(None)
}

async fn get_gitlab_description(url: String) -> Option<String> {
    let owner = url.split("/").nth(3).unwrap();
    let repo = url.split("/").nth(4).unwrap();

    let url = format!("https://gitlab.com/api/v4/projects/{}%2F{}", owner, repo);

    let res = reqwest::get(url)
        .await
        .unwrap()
        .json::<GitlabResponse>()
        .await;

    res.ok().map(|repo| repo.description)
}

pub async fn get_description_for_url(url: String) -> Option<String> {
    let host = Url::parse(url.as_str())
        .unwrap()
        .host_str()
        .unwrap()
        .to_string();

    if host == "github.com" {
        get_github_description(url).await
    } else if host.contains("gitlab") {
        get_gitlab_description(url).await
    } else {
        None
    }
}

impl Credit {
    pub fn new(title: &str, repo: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            url: repo.to_string(),
            description: Some(description.to_string()),
        }
    }
}

#[async_trait]
impl AsyncFrom<Pubspec> for Credit {
    async fn from(pubspec: Pubspec) -> Self {
        let url = pubspec
            .repository
            .unwrap_or_else(|| pubspec.homepage.expect("No homepage or repository found"));
        Self {
            title: pubspec.name,
            url: url.clone(),
            description: match pubspec.description {
                Some(description) => Some(description),
                None => get_description_for_url(url).await,
            },
        }
    }
}

#[async_trait]
impl AsyncFrom<PackageJson> for Credit {
    async fn from(package_json: PackageJson) -> Self {
        let url = package_json
            .repository
            .map(|repo| repo.url)
            .unwrap_or_else(|| {
                package_json
                    .homepage
                    .expect("No homepage or repository found")
            });
        Self {
            title: package_json.name,
            url: url.clone(),
            description: match package_json.description {
                Some(description) => Some(description),
                None => get_description_for_url(url).await,
            },
        }
    }
}

impl Credit {
    async fn from(dependency: String) -> () {
        let version = semver::VersionReq::parse(dependency.as_str());
        let url = Url::parse(
            dependency
                .replace("git+https://", "https://")
                .replace("git://", "https://")
                .as_str(),
        );

        let is_git = dependency.starts_with("git") && url.is_ok();
        let is_npm = version.is_ok();

        if is_git {
            let url = url.unwrap();
            let host = url.host_str().unwrap().to_string();
            let owner = url.path_segments().unwrap().nth(1).unwrap().to_string();
            let repo = url.path_segments().unwrap().nth(2).unwrap().to_string();

            let is_github = host == "github.com";
            let is_gitlab = host.contains("gitlab");

            if is_github {
                let package_json = octocrab::instance()
                    .repos(owner, repo)
                    .raw_file(Commitish("master".to_string()), "package.json")
                    .await
                    .unwrap()
                    .json::<PackageJson>()
                    .await
                    .unwrap();

                let url = package_json
                    .repository
                    .map(|repo| repo.url)
                    .unwrap_or_else(|| {
                        package_json
                            .homepage
                            .expect("No homepage or repository found")
                    });
                let credit = Self {
                    title: package_json.name,
                    url: url.clone(),
                    description: match package_json.description {
                        Some(description) => Some(description),
                        None => get_description_for_url(url).await,
                    },
                };
            }

            else if is_gitlab{
              
            }
        }
    }
}
