//! Database management for MateriaTrack

use crate::error::{DatabaseError, Result};
use crate::models::{Entry, EntryId, EntryWithDetails, Project, ProjectId, Task, TaskId};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension, Row};
use std::path::Path;
use std::sync::{Arc, Mutex};

const SCHEMA_VERSION: i32 = 1;

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn =
            Connection::open(path).map_err(|e| DatabaseError::ConnectionFailed(e.to_string()))?;

        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
            .map_err(|e| DatabaseError::ConnectionFailed(e.to_string()))?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };

        db.migrate()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| DatabaseError::ConnectionFailed(e.to_string()))?;

        conn.execute_batch("PRAGMA foreign_keys=ON;")
            .map_err(|e| DatabaseError::ConnectionFailed(e.to_string()))?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };

        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER PRIMARY KEY)",
            [],
        )
        .map_err(|e| DatabaseError::MigrationFailed(e.to_string()))?;

        let current_version: i32 = conn
            .query_row("SELECT version FROM schema_version LIMIT 1", [], |row| {
                row.get(0)
            })
            .unwrap_or(0);

        if current_version < 1 {
            conn.execute_batch(
                r#"
                CREATE TABLE IF NOT EXISTS projects (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL UNIQUE,
                    color TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS tasks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    project_id INTEGER NOT NULL,
                    name TEXT NOT NULL,
                    git_repo TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
                    UNIQUE(project_id, name)
                );

                CREATE TABLE IF NOT EXISTS entries (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    project_id INTEGER NOT NULL,
                    task_id INTEGER NOT NULL,
                    start_time TEXT NOT NULL,
                    end_time TEXT,
                    notes TEXT,
                    git_commits TEXT,
                    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
                    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
                );

                CREATE TABLE IF NOT EXISTS active_tracking (
                    id INTEGER PRIMARY KEY CHECK (id = 1),
                    entry_id INTEGER NOT NULL,
                    FOREIGN KEY (entry_id) REFERENCES entries(id) ON DELETE CASCADE
                );

                CREATE INDEX IF NOT EXISTS idx_entries_project ON entries(project_id);
                CREATE INDEX IF NOT EXISTS idx_entries_task ON entries(task_id);
                CREATE INDEX IF NOT EXISTS idx_entries_start ON entries(start_time);
                CREATE INDEX IF NOT EXISTS idx_tasks_project ON tasks(project_id);
                "#,
            )
            .map_err(|e| DatabaseError::MigrationFailed(e.to_string()))?;
        }

        conn.execute(
            "INSERT OR REPLACE INTO schema_version (version) VALUES (?1)",
            params![SCHEMA_VERSION],
        )
        .map_err(|e| DatabaseError::MigrationFailed(e.to_string()))?;

        Ok(())
    }

    pub fn create_project(&self, project: &mut Project) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO projects (name, color, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                project.name,
                project.color,
                project.created_at.to_rfc3339(),
                project.updated_at.to_rfc3339()
            ],
        )?;
        project.id = conn.last_insert_rowid();
        Ok(())
    }

    pub fn get_project(&self, id: ProjectId) -> Result<Option<Project>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, color, created_at, updated_at FROM projects WHERE id = ?1",
            params![id],
            row_to_project,
        )
        .optional()
        .map_err(Into::into)
    }

    pub fn get_project_by_name(&self, name: &str) -> Result<Option<Project>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, color, created_at, updated_at FROM projects WHERE name = ?1",
            params![name],
            row_to_project,
        )
        .optional()
        .map_err(Into::into)
    }

    pub fn list_projects(&self) -> Result<Vec<Project>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, color, created_at, updated_at FROM projects ORDER BY name",
        )?;
        let projects = stmt
            .query_map([], row_to_project)?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(projects)
    }

    pub fn update_project(&self, project: &Project) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let updated = conn.execute(
            "UPDATE projects SET name = ?1, color = ?2, updated_at = ?3 WHERE id = ?4",
            params![
                project.name,
                project.color,
                Utc::now().to_rfc3339(),
                project.id
            ],
        )?;

        if updated == 0 {
            return Err(DatabaseError::NotFound(format!("Project {}", project.id)).into());
        }
        Ok(())
    }

    pub fn delete_project(&self, id: ProjectId) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let deleted = conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;

        if deleted == 0 {
            return Err(DatabaseError::NotFound(format!("Project {}", id)).into());
        }
        Ok(())
    }

    pub fn create_task(&self, task: &mut Task) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tasks (project_id, name, git_repo, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task.project_id,
                task.name,
                task.git_repo,
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339()
            ],
        )?;
        task.id = conn.last_insert_rowid();
        Ok(())
    }

    pub fn get_task(&self, id: TaskId) -> Result<Option<Task>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, project_id, name, git_repo, created_at, updated_at FROM tasks WHERE id = ?1",
            params![id],
            row_to_task,
        )
        .optional()
        .map_err(Into::into)
    }

    pub fn get_task_by_name(&self, project_id: ProjectId, name: &str) -> Result<Option<Task>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, project_id, name, git_repo, created_at, updated_at FROM tasks WHERE project_id = ?1 AND name = ?2",
            params![project_id, name],
            row_to_task,
        )
        .optional()
        .map_err(Into::into)
    }

    pub fn list_tasks(&self, project_id: ProjectId) -> Result<Vec<Task>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, name, git_repo, created_at, updated_at FROM tasks WHERE project_id = ?1 ORDER BY name",
        )?;
        let tasks = stmt
            .query_map(params![project_id], row_to_task)?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(tasks)
    }

    pub fn list_all_tasks(&self) -> Result<Vec<Task>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, name, git_repo, created_at, updated_at FROM tasks ORDER BY name",
        )?;
        let tasks = stmt
            .query_map([], row_to_task)?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(tasks)
    }

    pub fn update_task(&self, task: &Task) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let updated = conn.execute(
            "UPDATE tasks SET name = ?1, git_repo = ?2, updated_at = ?3 WHERE id = ?4",
            params![task.name, task.git_repo, Utc::now().to_rfc3339(), task.id],
        )?;

        if updated == 0 {
            return Err(DatabaseError::NotFound(format!("Task {}", task.id)).into());
        }
        Ok(())
    }

    pub fn delete_task(&self, id: TaskId) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let deleted = conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;

        if deleted == 0 {
            return Err(DatabaseError::NotFound(format!("Task {}", id)).into());
        }
        Ok(())
    }

    pub fn create_entry(&self, entry: &mut Entry) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let git_commits = serde_json::to_string(&entry.git_commits)?;

        conn.execute(
            "INSERT INTO entries (project_id, task_id, start_time, end_time, notes, git_commits) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                entry.project_id,
                entry.task_id,
                entry.start.to_rfc3339(),
                entry.end.map(|e| e.to_rfc3339()),
                entry.notes,
                git_commits
            ],
        )?;
        entry.id = conn.last_insert_rowid();
        Ok(())
    }

    pub fn get_entry(&self, id: EntryId) -> Result<Option<Entry>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, project_id, task_id, start_time, end_time, notes, git_commits FROM entries WHERE id = ?1",
            params![id],
            row_to_entry,
        )
        .optional()
        .map_err(Into::into)
    }

    pub fn update_entry(&self, entry: &Entry) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let git_commits = serde_json::to_string(&entry.git_commits)?;

        let updated = conn.execute(
            "UPDATE entries SET start_time = ?1, end_time = ?2, notes = ?3, git_commits = ?4 WHERE id = ?5",
            params![
                entry.start.to_rfc3339(),
                entry.end.map(|e| e.to_rfc3339()),
                entry.notes,
                git_commits,
                entry.id
            ],
        )?;

        if updated == 0 {
            return Err(DatabaseError::NotFound(format!("Entry {}", entry.id)).into());
        }
        Ok(())
    }

    pub fn delete_entry(&self, id: EntryId) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let deleted = conn.execute("DELETE FROM entries WHERE id = ?1", params![id])?;

        if deleted == 0 {
            return Err(DatabaseError::NotFound(format!("Entry {}", id)).into());
        }
        Ok(())
    }

    pub fn list_entries(&self, since: Option<DateTime<Utc>>) -> Result<Vec<Entry>> {
        let conn = self.conn.lock().unwrap();

        if let Some(since) = since {
            let mut stmt = conn.prepare(
                "SELECT id, project_id, task_id, start_time, end_time, notes, git_commits 
                 FROM entries WHERE start_time >= ?1 ORDER BY start_time DESC",
            )?;
            let entries: Vec<Entry> = stmt
                .query_map(params![since.to_rfc3339()], row_to_entry)?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(entries)
        } else {
            let mut stmt = conn.prepare(
                "SELECT id, project_id, task_id, start_time, end_time, notes, git_commits 
                 FROM entries ORDER BY start_time DESC",
            )?;
            let entries: Vec<Entry> = stmt
                .query_map([], row_to_entry)?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(entries)
        }
    }

    pub fn list_entries_with_details(
        &self,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<EntryWithDetails>> {
        let conn = self.conn.lock().unwrap();

        let query = r#"
            SELECT e.id, e.project_id, e.task_id, e.start_time, e.end_time, e.notes, e.git_commits,
                   p.name as project_name, p.color as project_color, t.name as task_name
            FROM entries e
            JOIN projects p ON e.project_id = p.id
            JOIN tasks t ON e.task_id = t.id
        "#;

        if let Some(since) = since {
            let mut stmt = conn.prepare(&format!(
                "{} WHERE e.start_time >= ?1 ORDER BY e.start_time DESC",
                query
            ))?;
            let entries: Vec<EntryWithDetails> = stmt
                .query_map(params![since.to_rfc3339()], row_to_entry_with_details)?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(entries)
        } else {
            let mut stmt = conn.prepare(&format!("{} ORDER BY e.start_time DESC", query))?;
            let entries: Vec<EntryWithDetails> = stmt
                .query_map([], row_to_entry_with_details)?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(entries)
        }
    }

    pub fn set_active_tracking(&self, entry_id: EntryId) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO active_tracking (id, entry_id) VALUES (1, ?1)",
            params![entry_id],
        )?;
        Ok(())
    }

    pub fn get_active_tracking(&self) -> Result<Option<Entry>> {
        let conn = self.conn.lock().unwrap();

        let entry_id: Option<EntryId> = conn
            .query_row(
                "SELECT entry_id FROM active_tracking WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .optional()?;

        drop(conn);

        if let Some(id) = entry_id {
            self.get_entry(id)
        } else {
            Ok(None)
        }
    }

    pub fn clear_active_tracking(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM active_tracking WHERE id = 1", [])?;
        Ok(())
    }

    pub fn get_or_create_project(&self, name: &str) -> Result<Project> {
        if let Some(project) = self.get_project_by_name(name)? {
            return Ok(project);
        }

        let mut project = Project::new(name);
        self.create_project(&mut project)?;
        Ok(project)
    }

    pub fn get_or_create_task(&self, project_id: ProjectId, name: &str) -> Result<Task> {
        if let Some(task) = self.get_task_by_name(project_id, name)? {
            return Ok(task);
        }

        let mut task = Task::new(project_id, name);
        self.create_task(&mut task)?;
        Ok(task)
    }
}

fn parse_datetime(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn row_to_project(row: &Row) -> rusqlite::Result<Project> {
    Ok(Project {
        id: row.get(0)?,
        name: row.get(1)?,
        color: row.get(2)?,
        created_at: parse_datetime(&row.get::<_, String>(3)?),
        updated_at: parse_datetime(&row.get::<_, String>(4)?),
    })
}

fn row_to_task(row: &Row) -> rusqlite::Result<Task> {
    Ok(Task {
        id: row.get(0)?,
        project_id: row.get(1)?,
        name: row.get(2)?,
        git_repo: row.get(3)?,
        created_at: parse_datetime(&row.get::<_, String>(4)?),
        updated_at: parse_datetime(&row.get::<_, String>(5)?),
    })
}

fn row_to_entry(row: &Row) -> rusqlite::Result<Entry> {
    let end_time: Option<String> = row.get(4)?;
    let git_commits_str: String = row.get(6)?;
    let git_commits: Vec<String> =
        serde_json::from_str(&git_commits_str).unwrap_or_else(|_| Vec::new());

    Ok(Entry {
        id: row.get(0)?,
        project_id: row.get(1)?,
        task_id: row.get(2)?,
        start: parse_datetime(&row.get::<_, String>(3)?),
        end: end_time.map(|s| parse_datetime(&s)),
        notes: row.get(5)?,
        git_commits,
    })
}

fn row_to_entry_with_details(row: &Row) -> rusqlite::Result<EntryWithDetails> {
    let entry = row_to_entry(row)?;
    Ok(EntryWithDetails {
        entry,
        project_name: row.get(7)?,
        project_color: row.get(8)?,
        task_name: row.get(9)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = Database::open_in_memory().unwrap();
        let projects = db.list_projects().unwrap();
        assert!(projects.is_empty());
    }

    #[test]
    fn test_project_crud() {
        let db = Database::open_in_memory().unwrap();

        let mut project = Project::new("TestProject");
        db.create_project(&mut project).unwrap();
        assert!(project.id > 0);

        let fetched = db.get_project(project.id).unwrap().unwrap();
        assert_eq!(fetched.name, "TestProject");

        let projects = db.list_projects().unwrap();
        assert_eq!(projects.len(), 1);

        db.delete_project(project.id).unwrap();
        assert!(db.get_project(project.id).unwrap().is_none());
    }

    #[test]
    fn test_task_crud() {
        let db = Database::open_in_memory().unwrap();

        let mut project = Project::new("TestProject");
        db.create_project(&mut project).unwrap();

        let mut task = Task::new(project.id, "TestTask");
        db.create_task(&mut task).unwrap();
        assert!(task.id > 0);

        let fetched = db.get_task(task.id).unwrap().unwrap();
        assert_eq!(fetched.name, "TestTask");

        let tasks = db.list_tasks(project.id).unwrap();
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn test_entry_tracking() {
        let db = Database::open_in_memory().unwrap();

        let mut project = Project::new("TestProject");
        db.create_project(&mut project).unwrap();

        let mut task = Task::new(project.id, "TestTask");
        db.create_task(&mut task).unwrap();

        let mut entry = Entry::new(project.id, task.id);
        db.create_entry(&mut entry).unwrap();

        db.set_active_tracking(entry.id).unwrap();
        let active = db.get_active_tracking().unwrap().unwrap();
        assert_eq!(active.id, entry.id);

        db.clear_active_tracking().unwrap();
        assert!(db.get_active_tracking().unwrap().is_none());
    }
}
