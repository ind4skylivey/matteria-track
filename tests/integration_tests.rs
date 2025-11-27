use materiatrack::Database;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
pub fn test_db_path() -> PathBuf {
    std::env::var("MATERIATRACK_DB")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = std::env::temp_dir();
            path.push(format!("materiatrack_test_{}.db", std::process::id()));
            path
        })
}

#[test]
fn test_database_integration() {
    let db_path = test_db_path();
    // Ensure cleanup from previous runs if any
    let _ = fs::remove_file(&db_path);

    let db = Database::open(&db_path).expect("Failed to open test database");

    // Verify we can perform operations
    let projects = db.list_projects().expect("Failed to list projects");
    assert!(projects.is_empty());

    // Cleanup
    let _ = fs::remove_file(&db_path);
}
