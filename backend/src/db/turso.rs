use std::collections::HashMap;

pub async fn sync_project_views(db: &libsql::Database, views: &HashMap<u32, u32>) -> Result<(), Box<dyn std::error::Error>> {
    let conn = db.connect()?;
    
    for (id, count) in views {
        let query = format!(
            "UPDATE projects SET view_count = view_count + {} WHERE id = {}",
            count, id
        );
        conn.execute(&query, ()).await?;
    }
    
    Ok(())
}

pub async fn sync_log_views(db: &libsql::Database, views: &HashMap<u32, u32>) -> Result<(), Box<dyn std::error::Error>> {
    let conn = db.connect()?;
    
    for (id, count) in views {
        let query = format!(
            "UPDATE logs SET view_count = view_count + {} WHERE id = {}",
            count, id
        );
        conn.execute(&query, ()).await?;
    }
    
    Ok(())
}

pub async fn sync_project(db: &libsql::Database, id: u32, title: &str, repo_url: &str, demo_url: Option<&str>, view_count: u32, priority: u8, created_at: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = db.connect()?;
    
    // Note: README content is NOT synced to Turso to keep the database lightweight.
    // READMEs are only stored in local SQLite and sent through API requests.
    let query = format!(
        "INSERT OR REPLACE INTO projects (id, title, repo_url, readme_content, demo_url, view_count, priority, created_at) VALUES ({}, '{}', '{}', '', {}, {}, {}, '{}')",
        id,
        title.replace("'", "''"),
        repo_url.replace("'", "''"),
        demo_url.map(|s| format!("'{}'", s.replace("'", "''"))).unwrap_or_else(|| "NULL".to_string()),
        view_count,
        priority,
        created_at
    );
    
    conn.execute(&query, ()).await?;
    Ok(())
}

pub async fn delete_project_turso(db: &libsql::Database, id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let conn = db.connect()?;
    let query = format!("DELETE FROM projects WHERE id = {}", id);
    conn.execute(&query, ()).await?;
    Ok(())
}

pub async fn sync_log(db: &libsql::Database, id: u32, content: &str, view_count: u32, created_at: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = db.connect()?;
    
    let query = format!(
        "INSERT OR REPLACE INTO logs (id, content, view_count, created_at) VALUES ({}, '{}', {}, '{}')",
        id,
        content.replace("'", "''"),
        view_count,
        created_at
    );
    
    conn.execute(&query, ()).await?;
    Ok(())
}

pub async fn delete_log_turso(db: &libsql::Database, id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let conn = db.connect()?;
    let query = format!("DELETE FROM logs WHERE id = {}", id);
    conn.execute(&query, ()).await?;
    Ok(())
}
