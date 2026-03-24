use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GitHubLanguage {
    pub name: String,
    pub percentage: u32,
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GitHubStats {
    pub total_repos: u32,
    pub total_contributions: u32,
    pub languages: Vec<GitHubLanguage>,
}
