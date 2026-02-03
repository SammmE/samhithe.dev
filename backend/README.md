# Portfolio API - High-Efficiency Rust Backend

A blazingly fast portfolio API built with Axum and Rust, designed for minimal latency and maximum efficiency.

## 🚀 Features

- **In-Memory First**: All data loaded into memory on startup for <1ms read latency
- **Buffered View Counts**: View counts batched in memory and synced every 30 minutes to save database resources
- **Dual Database**: Local SQLite for speed + Turso for backup/sync
- **Auto GitHub README**: Automatically fetches and stores README content from GitHub repositories
- **Admin API**: Secure admin endpoints with password authentication
- **CORS Enabled**: Ready for frontend integration

## 📊 Architecture

```
┌─────────────┐
│   Request   │
└──────┬──────┘
       │
       ▼
┌─────────────────┐
│  In-Memory Data │  ◄── <1ms latency
│   (HashMap)     │
└─────────────────┘
       │
       │ (view counts buffered)
       ▼
┌─────────────────┐
│  View Buffer    │
│  (every 30min)  │
└─────────────────┘
       │
       ├──► Local SQLite  (primary, fast)
       │
       └──► Turso         (backup, async)
```

## 🔧 Setup

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone and Configure

```bash
git clone <your-repo>
cd samhithe-dev-backend
cp .env.example .env
```

### 3. Set Environment Variables

Edit `.env`:

```env
ADMIN_PASSWORD=your-secure-password-here
DATABASE_URL=file:portfolio.db
TURSO_URL=libsql://your-database.turso.io
TURSO_AUTH_TOKEN=your-turso-token
```

### 4. Build and Run

```bash
# Development
cargo run

# Production (optimized)
cargo build --release
./target/release/samhithe-dev-backend
```

Server starts on `http://0.0.0.0:3000`

## 📡 API Endpoints

### Public Endpoints

#### `GET /projects`
List all projects with their view counts (including buffered views).

**Response:**
```json
[
  {
    "id": 1,
    "title": "Awesome Project",
    "repo_url": "https://github.com/user/repo",
    "readme_content": "# Project README...",
    "demo_url": "https://demo.example.com",
    "view_count": 42,
    "created_at": "2024-01-01T00:00:00Z"
  }
]
```

#### `GET /projects/:id`
Get a single project by ID. **Increments view buffer.**

#### `GET /logs`
List all logs, sorted by creation date (newest first).

**Response:**
```json
[
  {
    "id": 1,
    "content": "Released version 2.0 with new features!",
    "view_count": 15,
    "created_at": "2024-01-01T00:00:00Z"
  }
]
```

#### `GET /logs/:id`
Get a single log by ID. **Increments view buffer.**

#### `GET /stats`
System statistics.

**Response:**
```json
{
  "uptime_seconds": 3600,
  "memory_usage_mb": 45,
  "engine": "Axum + Tokio",
  "persistence": "SQLite (local) + Turso (backup)",
  "buffered_views_size": 23
}
```

### Admin Endpoints

All admin endpoints require the `X-Admin-Password` header with your admin password.

#### `POST /admin/projects`
Create a new project. Automatically fetches README from GitHub.

**Request:**
```json
{
  "title": "My Project",
  "repo_url": "https://github.com/user/repo",
  "demo_url": "https://demo.example.com"  // optional
}
```

**Response:** Created project object

#### `PUT /admin/projects/:id`
Update an existing project.

**Request:**
```json
{
  "title": "Updated Title",           // optional
  "repo_url": "https://...",          // optional
  "demo_url": "https://...",          // optional
  "readme_content": "Custom README"   // optional
}
```

#### `DELETE /admin/projects/:id`
Delete a project.

#### `POST /admin/projects/:id/refresh-readme`
Re-fetch README content from GitHub for a project.

#### `POST /admin/logs`
Create a new log entry.

**Request:**
```json
{
  "content": "Launched new feature XYZ today!"
}
```

#### `PUT /admin/logs/:id`
Update a log entry.

**Request:**
```json
{
  "content": "Updated log content"
}
```

#### `DELETE /admin/logs/:id`
Delete a log entry.

#### `POST /admin/sync`
Force immediate sync of buffered view counts to databases.

## 🔐 Authentication

Admin endpoints require the `X-Admin-Password` header:

```bash
curl -X POST http://localhost:3000/admin/projects \
  -H "X-Admin-Password: your-password" \
  -H "Content-Type: application/json" \
  -d '{"title":"Test","repo_url":"https://github.com/user/repo"}'
```

## 💾 Database Schema

### Projects Table
```sql
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    repo_url TEXT NOT NULL,
    readme_content TEXT NOT NULL,
    demo_url TEXT,
    view_count INTEGER DEFAULT 0,
    created_at TEXT NOT NULL
);
```

### Logs Table
```sql
CREATE TABLE logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    view_count INTEGER DEFAULT 0,
    created_at TEXT NOT NULL
);
```

## ⚡ Performance

- **Read Latency**: <1ms (direct memory access)
- **Write Latency**: Immediate in-memory, async to disk
- **View Count Sync**: Every 30 minutes (configurable)
- **Memory Efficient**: Only stores active data in RAM
- **Concurrent**: Handles thousands of simultaneous requests

## 📋 Logging

The API uses structured logging via `tracing` for clean, professional output:

**Log Levels:**
- `INFO` - Server lifecycle, database operations, sync events
- `WARN` - Failed syncs, authentication failures
- `ERROR` - Critical errors, database failures

**Environment Variable:**
```bash
# Set log level (default: INFO)
RUST_LOG=debug cargo run
```

**Example Output:**
```
INFO  Portfolio API starting
INFO  Loaded 5 projects
INFO  Loaded 3 logs
INFO  Connected to Turso
INFO  Server listening on http://0.0.0.0:3000
INFO  Syncing view counts: 3 projects, 2 logs
```

## 🏗️ Project Structure

```
samhithe-dev-backend/
├── src/
│   ├── main.rs              # Server setup, router, background tasks
│   ├── models.rs            # Data structures (Project, Log, etc.)
│   ├── state.rs             # AppState with in-memory data + buffer
│   ├── db/
│   │   ├── local.rs         # Local SQLite operations
│   │   └── turso.rs         # Turso sync operations
│   ├── handlers/
│   │   ├── projects.rs      # Project endpoints
│   │   ├── logs.rs          # Log endpoints
│   │   ├── admin.rs         # Admin CRUD operations
│   │   └── system.rs        # System stats
│   └── middleware/
│       └── auth.rs          # Admin password verification
├── Cargo.toml               # Dependencies
├── .env.example             # Example environment variables
└── README.md                # This file
```

## 🔄 View Count Buffering

View counts are buffered in memory and synced to databases every 30 minutes:

1. **Request arrives** → View buffer incremented (O(1) operation)
2. **Every 30 minutes** → Buffer drained and batch-written to:
   - Local SQLite (immediate)
   - Turso (async, with retry)
3. **On Turso failure** → Views re-buffered for next sync (logged)

This approach reduces database writes by ~1800x while maintaining accuracy.

## 🛠️ Development

### Run in development mode with auto-reload:

```bash
cargo install cargo-watch
cargo watch -x run
```

### Run tests:

```bash
cargo test
```

### Format code:

```bash
cargo fmt
```

### Lint:

```bash
cargo clippy
```

## 📝 Example Usage

### Create a project (admin):
```bash
curl -X POST http://localhost:3000/admin/projects \
  -H "X-Admin-Password: mysecret" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Portfolio Website",
    "repo_url": "https://github.com/username/portfolio",
    "demo_url": "https://portfolio.com"
  }'
```

### Get all projects (public):
```bash
curl http://localhost:3000/projects
```

### View a specific project (increments view count):
```bash
curl http://localhost:3000/projects/1
```

### Create a log entry (admin):
```bash
curl -X POST http://localhost:3000/admin/logs \
  -H "X-Admin-Password: mysecret" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Just deployed a new feature to production!"
  }'
```

### Refresh a project's README (admin):
```bash
curl -X POST http://localhost:3000/admin/projects/1/refresh-readme \
  -H "X-Admin-Password: mysecret"
```

## 🔍 Monitoring

Check system stats:
```bash
curl http://localhost:3000/stats
```

**Response:**
```json
{
  "uptime_seconds": 3600,
  "memory_usage_mb": 45,
  "engine": "Axum + Tokio",
  "persistence": "SQLite (local) + Turso (backup)",
  "buffered_views_size": 23
}
```

Server logs provide detailed operation information:
```
INFO  Syncing view counts: 5 projects, 3 logs
```

## 🚨 Error Handling

- **404 Not Found**: Resource doesn't exist
- **401 Unauthorized**: Missing or invalid admin password
- **400 Bad Request**: Invalid input or GitHub API error
- **500 Internal Server Error**: Database or server error

## 📦 Dependencies

- **axum** - Web framework
- **tokio** - Async runtime
- **rusqlite** - Local SQLite driver
- **libsql** - Turso database client
- **serde** - Serialization
- **chrono** - Date/time handling
- **reqwest** - HTTP client (GitHub API)

## 🤝 Contributing

Contributions welcome! Please ensure:
- Code compiles: `cargo check`
- Tests pass: `cargo test`
- Code is formatted: `cargo fmt`
- No clippy warnings: `cargo clippy`

## 📄 License

MIT

## 🙋 Support

For issues or questions, open an issue on GitHub.

---

Built with ❤️ and Rust 🦀
