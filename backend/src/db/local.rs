use crate::models::{Log, Project};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            repo_url TEXT NOT NULL,
            readme_content TEXT NOT NULL,
            demo_url TEXT,
            view_count INTEGER DEFAULT 0,
            priority INTEGER DEFAULT 0,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            view_count INTEGER DEFAULT 0,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn load_all_projects(conn: &Connection) -> Result<HashMap<u32, Project>> {
    let mut stmt = conn.prepare("SELECT id, title, repo_url, readme_content, demo_url, view_count, priority, created_at FROM projects")?;
    let projects = stmt.query_map([], |row| {
        let created_at_str: String = row.get(7)?;
        Ok(Project {
            id: row.get(0)?,
            title: row.get(1)?,
            repo_url: row.get(2)?,
            readme_content: row.get(3)?,
            demo_url: row.get(4)?,
            view_count: row.get(5)?,
            priority: row.get(6)?,
            created_at: created_at_str.parse::<DateTime<Utc>>().unwrap(),
        })
    })?;

    let mut map = HashMap::new();
    for project in projects {
        let p = project?;
        map.insert(p.id, p);
    }
    Ok(map)
}

pub fn load_all_logs(conn: &Connection) -> Result<HashMap<u32, Log>> {
    let mut stmt = conn.prepare("SELECT id, content, view_count, created_at FROM logs")?;
    let logs = stmt.query_map([], |row| {
        let created_at_str: String = row.get(3)?;
        Ok(Log {
            id: row.get(0)?,
            content: row.get(1)?,
            view_count: row.get(2)?,
            created_at: created_at_str.parse::<DateTime<Utc>>().unwrap(),
        })
    })?;

    let mut map = HashMap::new();
    for log in logs {
        let l = log?;
        map.insert(l.id, l);
    }
    Ok(map)
}

pub fn load_logs_page(conn: &Connection, limit: u32, offset: u32) -> Result<Vec<Log>> {
    let mut stmt = conn.prepare(
        "SELECT id, content, view_count, created_at FROM logs ORDER BY created_at DESC LIMIT ? OFFSET ?"
    )?;
    let logs = stmt.query_map(params![limit, offset], |row| {
        let created_at_str: String = row.get(3)?;
        Ok(Log {
            id: row.get(0)?,
            content: row.get(1)?,
            view_count: row.get(2)?,
            created_at: created_at_str.parse::<DateTime<Utc>>().unwrap(),
        })
    })?;

    let mut result = Vec::new();
    for log in logs {
        result.push(log?);
    }
    Ok(result)
}

pub fn insert_project(conn: &Connection, project: &Project) -> Result<()> {
    conn.execute(
        "INSERT INTO projects (id, title, repo_url, readme_content, demo_url, view_count, priority, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            project.id,
            project.title,
            project.repo_url,
            project.readme_content,
            project.demo_url,
            project.view_count,
            project.priority,
            project.created_at.to_rfc3339()
        ],
    )?;
    Ok(())
}

pub fn update_project(conn: &Connection, project: &Project) -> Result<()> {
    conn.execute(
        "UPDATE projects SET title = ?1, repo_url = ?2, readme_content = ?3, demo_url = ?4, view_count = ?5, priority = ?6 WHERE id = ?7",
        params![
            project.title,
            project.repo_url,
            project.readme_content,
            project.demo_url,
            project.view_count,
            project.priority,
            project.id
        ],
    )?;
    Ok(())
}

pub fn delete_project(conn: &Connection, id: u32) -> Result<()> {
    conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn insert_log(conn: &Connection, log: &Log) -> Result<()> {
    conn.execute(
        "INSERT INTO logs (id, content, view_count, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![
            log.id,
            log.content,
            log.view_count,
            log.created_at.to_rfc3339()
        ],
    )?;
    Ok(())
}

pub fn update_log(conn: &Connection, log: &Log) -> Result<()> {
    conn.execute(
        "UPDATE logs SET content = ?1, view_count = ?2 WHERE id = ?3",
        params![log.content, log.view_count, log.id],
    )?;
    Ok(())
}

pub fn delete_log(conn: &Connection, id: u32) -> Result<()> {
    conn.execute("DELETE FROM logs WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn batch_update_project_views(conn: &Connection, views: &HashMap<u32, u32>) -> Result<()> {
    let tx = conn.unchecked_transaction()?;
    for (id, count) in views {
        tx.execute(
            "UPDATE projects SET view_count = view_count + ?1 WHERE id = ?2",
            params![count, id],
        )?;
    }
    tx.commit()?;
    Ok(())
}

pub fn batch_update_log_views(conn: &Connection, views: &HashMap<u32, u32>) -> Result<()> {
    let tx = conn.unchecked_transaction()?;
    for (id, count) in views {
        tx.execute(
            "UPDATE logs SET view_count = view_count + ?1 WHERE id = ?2",
            params![count, id],
        )?;
    }
    tx.commit()?;
    Ok(())
}

pub fn get_next_project_id(conn: &Connection) -> Result<u32> {
    let id: Result<u32> =
        conn.query_row("SELECT COALESCE(MAX(id), 0) + 1 FROM projects", [], |row| {
            row.get(0)
        });
    id
}

pub fn get_next_log_id(conn: &Connection) -> Result<u32> {
    let id: Result<u32> = conn.query_row("SELECT COALESCE(MAX(id), 0) + 1 FROM logs", [], |row| {
        row.get(0)
    });
    id
}
