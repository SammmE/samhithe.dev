# API Documentation

This document describes the REST API for the `samhithe.dev` backend.

## Base URL
The API runs at the host and port defined in the server configuration (by default `http://localhost:<PORT>`).

## Authentication
All routes under `/admin/*` require a Firebase JWT to be passed in the `Authorization` header:

```http
Authorization: Bearer <FIREBASE_JWT_TOKEN>
```

## Public Routes

### Health Check
Check the health of the backend server.

- **URL:** `/health`
- **Method:** `GET`
- **Response:**
  ```json
  {
    "status": "ok"
  }
  ```

### Projects
List all projects.

- **URL:** `/projects`
- **Method:** `GET`
- **Response:** Array of [Project](#project) objects.

### Articles
List all published articles.

- **URL:** `/articles`
- **Method:** `GET`
- **Response:** Array of [ArticleMeta](#articlemeta) objects.

### Single Article
Get a single article by its ID.

- **URL:** `/articles/{id}`
- **Method:** `GET`
- **Response:** An [Article](#article) object.

### Record Hit
Record a view/hit for a given article ID. This route is rate-limited (60 per second, burst size 10) and uses the client's IP address (hashed) to prevent duplicate counting.

- **URL:** `/hit/{id}`
- **Method:** `POST`
- **Response:**
  ```json
  {
    "counted": true // true if this is a new hit, false if it was already counted for this IP
  }
  ```

## Admin Routes (Requires Authentication)

### Create or Update Article
Create a new article or update an existing one. If an article with the given ID already exists, it will be updated.

- **URL:** `/admin/articles`
- **Method:** `POST`
- **Body:** [AdminArticleInput](#adminarticleinput)
- **Response:** The created/updated [ArticleMeta](#articlemeta) object. (Status 201 Created)

### Get Statistics
Get overall site statistics.

- **URL:** `/admin/stats`
- **Method:** `GET`
- **Response:** [StatsResponse](#statsresponse)

### Get Chart Data
Get time-series data for chart visualization.

- **URL:** `/admin/charts`
- **Method:** `GET`
- **Response:** Array of [ChartPoint](#chartpoint) objects.

### Create Project
Create a new project.

- **URL:** `/admin/projects`
- **Method:** `POST`
- **Body:** [ProjectInput](#projectinput)
- **Response:** The created [Project](#project) object. (Status 201 Created)

### Update Project
Update an existing project.

- **URL:** `/admin/projects/{id}`
- **Method:** `PATCH`
- **Body:** [ProjectPatch](#projectpatch)
- **Response:** The updated [Project](#project) object.

## Models

### `Project`
```json
{
  "id": "string",
  "name": "string",
  "description": "string",
  "demo_link": "string | null",
  "repo_link": "string | null",
  "readme_type": "url | raw",
  "readme_content": "string",
  "health_status": "unknown | healthy | broken",
  "last_health_check": "string (ISO 8601 datetime) | null"
}
```

### `ProjectInput`
```json
{
  "id": "string",
  "name": "string",
  "description": "string",
  "demo_link": "string | null",
  "repo_link": "string | null",
  "readme_type": "url | raw",
  "readme_content": "string"
}
```

### `ProjectPatch`
```json
{
  "name": "string | null",
  "description": "string | null",
  "demo_link": "string | null",
  "repo_link": "string | null",
  "readme_type": "url | raw | null",
  "readme_content": "string | null"
}
```

### `ArticleMeta`
```json
{
  "id": "string",
  "title": "string",
  "description": "string",
  "date": "string (ISO 8601 datetime)",
  "word_count": 0,
  "image_count": 0,
  "heading_count": 0,
  "views": 0,
  "is_published": true
}
```

### `Article`
Combines `ArticleMeta` fields with content:
```json
{
  "id": "string",
  "title": "string",
  "description": "string",
  "date": "string (ISO 8601 datetime)",
  "word_count": 0,
  "image_count": 0,
  "heading_count": 0,
  "views": 0,
  "is_published": true,
  "abstract_markdown": "string",
  "content": "string"
}
```

### `AdminArticleInput`
```json
{
  "id": "string",
  "title": "string",
  "description": "string",
  "abstract_markdown": "string",
  "content": "string",
  "date": "string (ISO 8601 datetime) | null",
  "is_published": true
}
```

### `StatsResponse`
```json
{
  "global_views": 0,
  "best_performer": { /* ArticleMeta | null */ },
  "trending": 0,
  "broken_projects": 0
}
```

### `ChartPoint`
```json
{
  "date": "string",
  "views": 0
}
```
