use axum::{extract::Query, http::Method, response::Json, routing::get, Router};
use serde::Deserialize;
use shared::{fallback::fallback_stats, github::{GitHubLanguage, GitHubStats}};
use std::collections::{HashMap, HashSet};
use tower_http::cors::{Any, CorsLayer};

// ── GraphQL Types ─────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct StatsQuery {
    username: Option<String>,
}

#[derive(Deserialize)]
struct GraphQLResponse {
    data: Option<GraphQLData>,
}

#[derive(Deserialize)]
struct GraphQLData {
    user: GitHubUser,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GitHubUser {
    contributions_collection: ContributionsCollection,
    repositories: RepositoryConnection,
    public_repositories: RepositoryConnection,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContributionsCollection {
    contribution_calendar: ContributionCalendar,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContributionCalendar {
    total_contributions: u32,
}

#[derive(Deserialize)]
struct RepositoryConnection {
    nodes: Vec<Repository>,
}

#[derive(Deserialize)]
struct Repository {
    name: String,
    languages: LanguageConnection,
}

#[derive(Deserialize)]
struct LanguageConnection {
    edges: Vec<LanguageEdge>,
}

#[derive(Deserialize)]
struct LanguageEdge {
    size: u64,
    node: LanguageNode,
}

#[derive(Deserialize)]
struct LanguageNode {
    name: String,
}

// ── Constants ─────────────────────────────────────────────────────────────────

const LANGUAGE_COLORS: &[&str] = &[
    "rose-pine-foam",
    "rose-pine-gold",
    "rose-pine-iris",
    "rose-pine-rose",
    "primary",
    "rose-pine-love",
    "rose-pine-pine",
    "rose-pine-subtle",
];

const GRAPHQL_QUERY: &str = "
query($username: String!) {
  user(login: $username) {
    contributionsCollection {
      contributionCalendar { totalContributions }
    }
    repositories(
      first: 100,
      ownerAffiliations: [OWNER, COLLABORATOR, ORGANIZATION_MEMBER],
      privacy: PRIVATE
    ) {
      nodes {
        name
        languages(first: 10, orderBy: {field: SIZE, direction: DESC}) {
          edges { size node { name } }
        }
      }
    }
    publicRepositories: repositories(
      first: 100,
      ownerAffiliations: [OWNER, COLLABORATOR, ORGANIZATION_MEMBER],
      privacy: PUBLIC
    ) {
      nodes {
        name
        languages(first: 10, orderBy: {field: SIZE, direction: DESC}) {
          edges { size node { name } }
        }
      }
    }
  }
}";

// ── Handler ───────────────────────────────────────────────────────────────────

async fn github_stats(Query(params): Query<StatsQuery>) -> Json<serde_json::Value> {
    let username = params.username.unwrap_or_else(|| "AmaneKai".into());
    let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();

    if token.is_empty() {
        return Json(serde_json::to_value(fallback_stats()).unwrap());
    }

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "query": GRAPHQL_QUERY,
        "variables": { "username": username }
    });

    let resp = match client
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .header("User-Agent", "portfolio-api/1.0")
        .json(&body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => return Json(serde_json::to_value(fallback_stats()).unwrap()),
    };

    let gql: GraphQLResponse = match resp.json().await {
        Ok(d) => d,
        Err(_) => return Json(serde_json::to_value(fallback_stats()).unwrap()),
    };

    let user = match gql.data {
        Some(d) => d.user,
        None => return Json(serde_json::to_value(fallback_stats()).unwrap()),
    };

    let total_contributions =
        user.contributions_collection.contribution_calendar.total_contributions;

    let mut seen: HashSet<String> = HashSet::new();
    let mut lang_bytes: HashMap<String, u64> = HashMap::new();

    for repo in user
        .repositories
        .nodes
        .iter()
        .chain(user.public_repositories.nodes.iter())
    {
        if !seen.insert(repo.name.clone()) {
            continue;
        }
        for edge in &repo.languages.edges {
            *lang_bytes.entry(edge.node.name.clone()).or_insert(0) += edge.size;
        }
    }

    let total_repos = seen.len() as u32;
    let total_bytes: u64 = lang_bytes.values().sum();

    let mut lang_list: Vec<(String, u64)> = lang_bytes.into_iter().collect();
    lang_list.sort_by(|a, b| b.1.cmp(&a.1));

    let languages: Vec<GitHubLanguage> = lang_list
        .into_iter()
        .filter_map(|(name, bytes)| {
            let pct = ((bytes as f64 / total_bytes as f64) * 100.0).round() as u32;
            if pct == 0 {
                return None;
            }
            Some((name, pct))
        })
        .enumerate()
        .map(|(i, (name, percentage))| GitHubLanguage {
            name,
            percentage,
            color: LANGUAGE_COLORS[i % LANGUAGE_COLORS.len()].into(),
        })
        .collect();

    let stats = GitHubStats { total_repos, total_contributions, languages };
    Json(serde_json::to_value(stats).unwrap())
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/github-stats", get(github_stats))
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".into());
    let addr = format!("0.0.0.0:{port}");

    println!("🦀 API server running at http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
