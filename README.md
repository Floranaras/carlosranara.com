# carlosranara.com

Personal portfolio — full Rust stack. Frontend compiled to WASM via Leptos, GitHub stats
proxied through a Cloudflare Worker, deployed to Cloudflare Pages.

## Stack

| Layer      | Technology                          |
|------------|-------------------------------------|
| Frontend   | Leptos 0.6 (CSR → WASM)             |
| Styling    | Tailwind CSS (pre-compiled)         |
| Router     | leptos_router                       |
| API        | Cloudflare Worker (Rust → WASM)     |
| Local Dev  | Axum (GitHub stats proxy)           |
| Deployment | Cloudflare Pages + Workers          |
| Build      | Trunk                               |

## Project Structure

```
carlosranara.com/
├── crates/
│   ├── app/                  # Leptos frontend (compiles to WASM)
│   │   └── src/
│   │       ├── lib.rs        # App root, router, mount()
│   │       ├── config.rs     # WORKER_URL, GITHUB_USERNAME
│   │       ├── components/   # Header, Footer
│   │       ├── data/         # projects.rs, timeline.rs
│   │       └── pages/
│   │           ├── home/     # hero.rs, about.rs, featured.rs
│   │           ├── projects.rs
│   │           ├── journey.rs
│   │           └── contact.rs
│   ├── api/                  # Axum local dev server (port 3001)
│   ├── worker/               # Cloudflare Worker (GitHub stats proxy)
│   └── shared/               # Shared types and logic
│       └── src/
│           ├── github.rs     # GitHubStats, GitHubLanguage
│           ├── fallback.rs   # Fallback data when API is unavailable
│           └── assets.rs     # Asset enum — compile-time path safety
├── public/                   # Static assets (gifs, screenshots, CV)
├── style/
│   └── output.css            # Pre-compiled Tailwind CSS
├── Cargo.toml                # Workspace root
├── Trunk.toml                # Frontend build config
├── wrangler.toml             # Cloudflare Worker config
├── build.sh                  # Cloudflare Pages build script
├── rustfmt.toml              # max_width = 100
└── leptosfmt.toml            # max_width = 100 for view! macros
```

## Local Development

### Prerequisites

- Rust + `wasm32-unknown-unknown` target
- `cargo install trunk`
- `cargo install leptosfmt`
- A GitHub personal access token (fine-grained, read-only public repos)

### Setup

```bash
git clone https://github.com/Floranaras/carlosranara.com.git
cd carlosranara.com
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install leptosfmt
```

### Running

```bash
# Terminal 1 — local GitHub stats API
GITHUB_TOKEN=your_token cargo run -p api

# Terminal 2 — frontend with hot reload
trunk serve
```

Open `http://localhost:8080`.

### Formatting

```bash
cargo fmt && leptosfmt crates/app/src/**/*.rs
```

## Deployment

### Cloudflare Worker (GitHub Stats API)

```bash
npm install -g wrangler
wrangler login
wrangler secret put GITHUB_TOKEN
wrangler deploy
```

### Cloudflare Pages (Frontend)

Connected to this repo's `main` branch via Cloudflare Pages dashboard.

| Setting          | Value              |
|------------------|--------------------|
| Build command    | `bash build.sh`    |
| Output directory | `dist`             |
| Branch           | `main`             |

Every push to `main` triggers an automatic deploy.

## Architecture Decisions

**Why Leptos over React?**
The entire stack compiles to WASM — no JavaScript runtime, no Node.js toolchain in
production. The frontend is a single `.wasm` binary served as a static file.

**Why a Cloudflare Worker for GitHub stats?**
The GitHub GraphQL API requires a token. Embedding a token in WASM is insecure since
WASM is inspectable. The Worker proxies the request server-side keeping the token in
Cloudflare's secret store.

**Why pre-compiled Tailwind CSS?**
Tailwind v4's JIT scanner doesn't understand Leptos view! macro syntax. The CSS is
compiled once from the React prototype and committed directly. New utility classes are
appended manually.

**Why `shared` crate?**
GitHubStats, GitHubLanguage, fallback data, and the Asset enum are used across
app, api, and worker. Defining them once in shared eliminates duplication and
gives a single place to add unit tests for data integrity.

## Code Standards

- No unconstrained generics without purpose
- Explicit return types on all public functions
- Max 100 characters per line (enforced by rustfmt + leptosfmt)
- Named exports, no pub use * except in mod.rs barrel files
- view! sub-components extracted when nesting exceeds 3 levels

## License

MIT
