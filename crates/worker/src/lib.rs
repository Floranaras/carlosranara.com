use serde::Deserialize;
use serde_json::json;
use shared::{
    fallback::fallback_stats,
    github::{GitHubLanguage, GitHubStats},
};
use std::collections::{HashMap, HashSet};
use worker::*;

// ── GitHub GraphQL Types ──────────────────────────────────────────────────────

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

fn cors_headers(mut response: Response) -> Result<Response> {
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "GET")?;
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", "public, max-age=86400")?;
    Ok(response)
}

// ── Handler ───────────────────────────────────────────────────────────────────

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if req.method() == Method::Options {
        let mut resp = Response::empty()?;
        let headers = resp.headers_mut();
        headers.set("Access-Control-Allow-Origin", "*")?;
        headers.set("Access-Control-Allow-Methods", "GET, OPTIONS")?;
        return Ok(resp);
    }

    let url = req.url()?;
    let username = url
        .query_pairs()
        .find(|(k, _)| k == "username")
        .map(|(_, v)| v.into_owned())
        .unwrap_or_else(|| "AmaneKai".to_string());

    let token = match env.secret("GITHUB_TOKEN") {
        Ok(s) => s.to_string(),
        Err(_) => return cors_headers(Response::from_json(&fallback_stats())?),
    };

    if token.is_empty() {
        return cors_headers(Response::from_json(&fallback_stats())?);
    }

    let body = json!({
        "query": GRAPHQL_QUERY,
        "variables": { "username": username }
    });

    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_body(Some(body.to_string().into()));

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {token}"))?;
    headers.set("Content-Type", "application/json")?;
    headers.set("User-Agent", "portfolio-worker/1.0")?;
    init.with_headers(headers);

    let gh_req = Request::new_with_init("https://api.github.com/graphql", &init)?;
    let mut gh_resp = Fetch::Request(gh_req).send().await?;

    let gql: GraphQLResponse = match gh_resp.json().await {
        Ok(d) => d,
        Err(_) => return cors_headers(Response::from_json(&fallback_stats())?),
    };

    let user = match gql.data {
        Some(d) => d.user,
        None => return cors_headers(Response::from_json(&fallback_stats())?),
    };

    let total_contributions = user
        .contributions_collection
        .contribution_calendar
        .total_contributions;

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
    cors_headers(Response::from_json(&stats)?)
}
