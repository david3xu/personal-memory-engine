// Verify owner controls fail closed and serialize against in-flight worker access.
use memory_local_runtime::connection::ConnectionControl;
#[test]
fn corrupted_or_future_configuration_does_not_grant_access() {
    let dir = tempfile::tempdir().unwrap();
    let database = dir.path().join("memory.sqlite3");
    let control = ConnectionControl::for_database(&database);
    std::fs::write(database.with_extension("connection.json"), "broken").unwrap();
    assert!(control.worker_access().is_err());
    assert!(control.resume().is_err());
    std::fs::write(
        database.with_extension("connection.json"),
        r#"{"schema_version":2,"enabled":true,"test_request_id":null,"receipt":null}"#,
    )
    .unwrap();
    assert!(control.worker_access().is_err());
}
#[test]
fn pause_waits_for_inflight_access_and_new_helpers_observe_it() {
    let dir = tempfile::tempdir().unwrap();
    let database = dir.path().join("memory.sqlite3");
    let control = ConnectionControl::for_database(&database);
    let access = control.worker_access().unwrap();
    let (send, receive) = std::sync::mpsc::channel();
    let other = control.clone();
    let thread = std::thread::spawn(move || {
        other.pause().unwrap();
        send.send(()).unwrap();
    });
    assert!(
        receive
            .recv_timeout(std::time::Duration::from_millis(100))
            .is_err()
    );
    drop(access);
    receive
        .recv_timeout(std::time::Duration::from_secs(3))
        .unwrap();
    thread.join().unwrap();
    assert!(
        ConnectionControl::for_database(&database)
            .worker_access()
            .is_err()
    );
}
