// Verify frozen selections, restart-safe publication bookkeeping and unchanged private decision history.
use memory_engine::{CaptureDecision, Engine};
use memory_local_runtime::{SqliteStore, sharing::ShareStore};
fn capture(option: &str, key: &str) -> CaptureDecision {
    serde_json::from_value(serde_json::json!({"request_id":key,"user_confirmed":true,"chosen_option":option,"worker":"Isolated sharing test","user_statement":"PRIVATE-CONVERSATION"})).unwrap()
}
#[test]
fn preview_is_frozen_and_publication_bookkeeping_does_not_change_memory() {
    let dir = tempfile::tempdir().unwrap();
    let store = SqliteStore::open(&dir.path().join("memory.sqlite3")).unwrap();
    let engine = Engine::new(store.clone());
    let selected = engine
        .record(capture("Selected choice", "selected"))
        .unwrap();
    engine
        .record(capture("PRIVATE-UNSELECTED", "secret"))
        .unwrap();
    let shares = ShareStore::new(dir.path());
    let mut preview = shares
        .prepare(
            &store,
            std::slice::from_ref(&selected.decision_id),
            "Synthetic snapshot",
            false,
            false,
        )
        .unwrap();
    let mut revision = capture("Later local choice", "revision");
    revision.decision_id = Some(selected.decision_id.clone());
    revision.supersedes_version_id = Some(selected.version_id);
    engine.record(revision).unwrap();
    preview.status = "pending".into();
    preview.repository = Some("owner/shares".into());
    preview.url = Some(format!(
        "https://owner.github.io/shares/shares/{}/",
        preview.id
    ));
    shares.update(preview.clone()).unwrap();
    let reopened = ShareStore::new(dir.path());
    let frozen = reopened.get(&preview.id).unwrap();
    assert_eq!(
        frozen.snapshot.cards[0].versions[0].chosen_option,
        "Selected choice"
    );
    assert!(
        !serde_json::to_string(&frozen.snapshot)
            .unwrap()
            .contains("PRIVATE-")
    );
    assert_eq!(engine.list().unwrap().len(), 2);
    assert_eq!(engine.history(&selected.decision_id).unwrap().len(), 2);
    let mut changed = frozen.clone();
    changed.snapshot.title = "Different unreviewed content".into();
    assert!(reopened.update(changed).is_err());
    preview.status = "live".into();
    reopened.update(preview.clone()).unwrap();
    preview.status = "withdrawal_pending".into();
    reopened.update(preview.clone()).unwrap();
    preview.status = "withdrawn".into();
    reopened.update(preview.clone()).unwrap();
    assert_eq!(engine.history(&selected.decision_id).unwrap().len(), 2);
    assert_eq!(reopened.entries().unwrap().len(), 1);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            std::fs::metadata(dir.path().join("memory.publications.json"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
}
