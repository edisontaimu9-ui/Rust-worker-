# rust-worker-scaffold

Minimal Cloudflare Worker written in Rust (compiled to WASM via `workers-rs`).

## Deploy path (Termux-friendly)

`wrangler`'s build tooling (esbuild, `worker-build`, `workerd`) relies on
prebuilt native binaries that generally don't have Termux/Android builds, so
building or running wrangler directly inside Termux tends to fail.

Instead, this repo builds and deploys entirely in **GitHub Actions**
(`.github/workflows/deploy.yml`), on every push to `main`. Termux only needs
`git`:

```bash
cd rust-worker-scaffold
git init
git add .
git commit -m "Initial Rust Cloudflare Worker scaffold"
git branch -M main
git remote add origin <your-new-repo-url>
git push -u origin main
```

### One-time GitHub setup

1. In your Cloudflare dashboard, create an API token with Workers deploy
   permissions.
2. In the GitHub repo: Settings → Secrets and variables → Actions →
   New repository secret → name it `CLOUDFLARE_API_TOKEN`, paste the token.
3. Push to `main` — the workflow builds the WASM binary and deploys via
   `wrangler` on Ubuntu runners.

No local Rust, wasm target, or wrangler install needed on your phone.

## Routes included

- `GET /` — plain text confirmation
- `GET /health` — JSON health check
- `GET /echo/:msg` — echoes a URL path segment back as JSON
- `POST /api/data` — accepts JSON body, echoes it back (replace with real logic)

## Next steps

- Add KV/D1/R2 bindings in `wrangler.toml` as needed
- Add real handlers in `src/lib.rs`
- Split handlers into modules (`src/handlers/`) as the project grows
