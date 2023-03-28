use crate::clients::{npm::package_json::PackageJson, pubdev::pubspec_yaml::Pubspec};

pub struct Credit {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
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

impl From<Pubspec> for Credit {
    fn from(pubspec: Pubspec) -> Self {
        Self {
            title: pubspec.name,
            url: pubspec
                .repository
                .unwrap_or_else(|| pubspec.homepage.expect("No homepage or repository found")),
            description: pubspec.description,
        }
    }
}

impl From<PackageJson> for Credit {
    fn from(package_json: PackageJson) -> Self {
        Self {
            title: package_json.name,
            url: package_json
                .repository
                .map(|repo| repo.url)
                .unwrap_or_else(|| {
                    package_json
                        .homepage
                        .expect("No homepage or repository found")
                }),
            description: package_json.description,
        }
    }
}
