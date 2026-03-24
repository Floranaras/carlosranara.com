use crate::github::{GitHubLanguage, GitHubStats};

pub fn fallback_stats() -> GitHubStats {
    GitHubStats {
        total_repos: 42,
        total_contributions: 1842,
        languages: vec![
            GitHubLanguage {
                name: "Rust".into(),
                percentage: 62,
                color: "rose-pine-foam".into(),
            },
            GitHubLanguage {
                name: "TypeScript".into(),
                percentage: 21,
                color: "rose-pine-gold".into(),
            },
            GitHubLanguage {
                name: "C".into(),
                percentage: 17,
                color: "rose-pine-iris".into(),
            },
        ],
    }
}
