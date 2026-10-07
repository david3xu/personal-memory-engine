// Exercise durable history, retry integrity, stale revisions, and database protection.
use memory_engine::{CaptureDecision, Engine};
use memory_local_runtime::SqliteStore;
use tempfile::TempDir;
fn input(key: &str, choice: &str) -> CaptureDecision {
    serde_json::from_value(serde_json::json!({"request_id":key,"user_confirmed":true,"chosen_option":choice,"worker":"Synthetic integration test"})).unwrap()
}
#[test]
fn restart_and_revisions_preserve_records() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("memory.sqlite3");
    let engine = Engine::new(SqliteStore::open(&path).unwrap());
    let first = engine.record(input("first", "Rust")).unwrap();
    let mut changed = input("second", "TypeScript");
    changed.decision_id = Some(first.decision_id.clone());
    changed.supersedes_version_id = Some(first.version_id.clone());
    let second = engine.record(changed.clone()).unwrap();
    assert_eq!(engine.record(changed).unwrap(), second);
    assert_eq!(engine.record(input("first", "Rust")).unwrap(), first);
    assert!(engine.record(input("first", "Changed content")).is_err());
    let mut stale = input("stale", "Another option");
    stale.decision_id = Some(first.decision_id.clone());
    stale.supersedes_version_id = Some(first.version_id.clone());
    assert!(engine.record(stale).is_err());
    drop(engine);
    let reopened = Engine::new(SqliteStore::open(&path).unwrap());
    assert_eq!(reopened.list().unwrap(), vec![second.clone()]);
    assert_eq!(
        reopened.history(&first.decision_id).unwrap(),
        vec![second, first]
    );
}
#[test]
fn invalid_references_leave_existing_records_unchanged() {
    let dir = TempDir::new().unwrap();
    let engine = Engine::new(SqliteStore::open(&dir.path().join("memory.sqlite3")).unwrap());
    let first = engine.record(input("one", "Rust")).unwrap();
    let other = engine.record(input("two", "SQLite")).unwrap();
    let mut changed = input("wrong", "New choice");
    changed.decision_id = Some(first.decision_id.clone());
    changed.supersedes_version_id = Some(other.version_id.clone());
    assert!(engine.record(changed).is_err());
    assert_eq!(engine.history(&first.decision_id).unwrap(), vec![first]);
}
#[test]
fn concurrent_retries_create_one_version() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("memory.sqlite3");
    let store = SqliteStore::open(&path).unwrap();
    let joins: Vec<_> = (0..4)
        .map(|_| {
            let independent = SqliteStore::open(&path).unwrap();
            std::thread::spawn(move || {
                Engine::new(independent)
                    .record(input("retry", "Rust"))
                    .unwrap()
            })
        })
        .collect();
    let records: Vec<_> = joins.into_iter().map(|join| join.join().unwrap()).collect();
    assert!(records.iter().all(|record| record == &records[0]));
    assert_eq!(Engine::new(store).list().unwrap().len(), 1);
}
#[test]
fn database_rejects_overwrite_and_individual_delete() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("memory.sqlite3");
    let engine = Engine::new(SqliteStore::open(&path).unwrap());
    engine.record(input("one", "Rust")).unwrap();
    let raw = rusqlite::Connection::open(&path).unwrap();
    assert!(
        raw.execute("UPDATE decision_versions SET record_json='{}'", [])
            .is_err()
    );
    assert!(raw.execute("DELETE FROM decision_versions", []).is_err());
}
#[test]
fn future_database_is_not_silently_downgraded() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("memory.sqlite3");
    let raw = rusqlite::Connection::open(&path).unwrap();
    raw.pragma_update(None, "user_version", 99).unwrap();
    drop(raw);
    assert!(SqliteStore::open(&path).is_err());
    let raw = rusqlite::Connection::open(&path).unwrap();
    let version: u32 = raw
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .unwrap();
    assert_eq!(version, 99);
}
