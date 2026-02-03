# samhithe.dev - Portfolio Monorepo

A performant, content-driven portfolio platform comprising:
- Next.js frontend for public pages and MDX content
- Rust/Axum API engineered for sub-millisecond reads with buffered writes
- SvelteKit admin dashboard for managing projects and technical logs

The stack emphasizes fast page loads, minimal latency API responses, and a pragmatic authoring workflow.

## Architecture Overview

```
Frontend (Next.js, MDX)           Admin (SvelteKit)
        |                               |
        +------------- HTTP(S) ---------+
                        |
                  Rust API (Axum)
                        |
          In-Memory Store + View Buffer
                        |
                        +--> SQLite (primary, batched ~30m)
                        +--> Turso/libSQL (remote sync)
```

Key properties
- In-memory first reads (<1ms) with periodic buffered writes
- Dual persistence: local SQLite (authoritative) + Turso (remote backup/sync)
- Admin API protected via an `X-Admin-Password` header
- CORS enabled; structured logging and type-safe models

## Repository Layout

```
samhithe.dev/
├─ frontend/                # Next.js 14 (App Router), Tailwind, Velite (MDX)
│  ├─ app/                  # Pages/layouts
│  ├─ components/           # UI components
│  ├─ content/              # MDX posts and projects
│  └─ next.config.mjs       # Velite build hook integration
├─ backend/                 # Rust Axum API
│  ├─ src/
│  │  ├─ handlers/          # projects, logs, admin, system
│  │  ├─ db/                # local SQLite + Turso sync
│  │  ├─ middleware/        # admin auth
│  │  ├─ models.rs          # data types
│  │  └─ main.rs            # router, state, background sync
│  └─ Cargo.toml
└─ admin/                   # SvelteKit admin dashboard (Vite)
   └─ src/
      ├─ routes/            # login, projects, logs
      └─ lib/               # axios client, stores, components
```

## Quick Start

Prerequisites
- Node.js 20+
- Rust (stable)
- Turso account/token for remote sync (optional for local dev)

Clone
```bash
git clone <repo-url>
cd samhithe.dev
```

Run the backend (API)
```bash
cd backend
# If .env.example is unavailable, create .env with:
# ADMIN_PASSWORD=your-secure-password
# DATABASE_URL=file:portfolio.db
# TURSO_URL=libsql://your-db.turso.io
# TURSO_AUTH_TOKEN=your-turso-token
cargo run
# API: http://0.0.0.0:3000
```

Run the frontend (site)
```bash
cd frontend
npm install
# Use a non-conflicting port if the API runs on 3000
npm run dev -- -p 3001
# Site: http://localhost:3001
```

Run the admin dashboard
```bash
cd admin
npm install
npm run dev
# Admin: http://localhost:5173 (Vite default)
# The admin targets the API at http://localhost:3000 (see admin/src/lib/api.ts)
```

Tip: Start the backend first, then frontend and admin in separate terminals.

## Configuration

Backend environment (.env)
- `ADMIN_PASSWORD` — required for admin routes (sent as `X-Admin-Password`)
- `DATABASE_URL` — SQLite path, e.g. `file:portfolio.db`
- `TURSO_URL` — Turso/libSQL Database URL
- `TURSO_AUTH_TOKEN` — Turso auth token

Admin API base URL
- Defaults to `http://localhost:3000` in `admin/src/lib/api.ts`. Update for production.

Frontend content pipeline
- MDX content is compiled at build time via Velite (see `frontend/velite.config.ts`).

## API Overview (backend)

Public
- `GET /projects` — list projects (includes README content, `view_count`)
- `GET /projects/:id` — get project (increments view buffer)
- `GET /logs` — list logs
- `GET /logs/:id` — get log (increments view buffer)
- `GET /stats` — system stats (uptime, memory, engine, buffer sizes)

Admin (requires header `X-Admin-Password: <ADMIN_PASSWORD>`)
- `POST /admin/projects` - create project (auto-fetch README from GitHub)
- `PUT /admin/projects/:id` — update project (title/repo/demo/priority/readme)
- `DELETE /admin/projects/:id` — delete project
- `POST /admin/projects/:id/refresh-readme` - re-fetch README from GitHub
- `POST /admin/logs` — create log
- `PUT /admin/logs/:id` — update log
- `DELETE /admin/logs/:id` — delete log
- `POST /admin/sync` — force immediate view count sync

Example: create project
```bash
curl -X POST http://localhost:3000/admin/projects \
  -H "X-Admin-Password: your-password" \
  -H "Content-Type: application/json" \
  -d '{"title":"My Project","repo_url":"https://github.com/user/repo","demo_url":"https://demo.example.com"}'
```

## Content Model (frontend)

Projects (frontend/content/projects/*.mdx)
```mdx
---
title: Project Name
problem: What problem does it solve?
stack: [Tech1, Tech2]
hurdle: Biggest challenge
tags: [tag1, tag2]
link: https://example.com
github_link: https://github.com/username/repo
featured: true
---

# Project details...
```

Posts (frontend/content/posts/*.mdx)
```mdx
---
title: Post Title
date: YYYY-MM-DD
tags: [tag1, tag2]
preview: Brief description
---

# Post content...
```

## Development

Backend
```bash
cargo watch -x run     # hot reload (install cargo-watch)
cargo test             # tests
cargo fmt              # format
cargo clippy           # lint
```

Frontend
```bash
npm run dev
npm run build
npm run start
npm run lint
```

Admin
```bash
npm run dev
npm run build
npm run preview
npm run check
```

## Performance Characteristics

- Read latency: sub-millisecond (in-memory access)
- Buffered writes: batched every 30 minutes to SQLite and Turso, with retry
- Concurrency: Axum + Tokio, async I/O throughout
- Logging: structured logs via `tracing` with environment filtering

## Production Checklist

- Configure strong `ADMIN_PASSWORD` and rotate credentials periodically
- Set appropriate CORS/origin allowlists
- Place the API behind a reverse proxy with TLS termination
- Provision Turso and validate connectivity; schedule backups for SQLite snapshotting
- Configure metrics/alerts for sync failures and error rates
- Pin container or binary versions; enable health checks

## Licensing

This repository does not currently include a root license file. The backend component references MIT in its documentation. If you intend to distribute this code, add a root `LICENSE` that reflects your preferred terms and update component READMEs accordingly.
