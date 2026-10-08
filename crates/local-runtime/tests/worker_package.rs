// Exercise the bundled connector with relocated paths, real stdio recording and encoded links.
use memory_local_runtime::{SqliteStore, worker_package};
use rmcp::{ClientHandler, ServiceExt, model::CallToolRequestParams};
use serde_json::{Value, json};
use std::path::PathBuf;
#[derive(Clone)]
struct Client;
impl ClientHandler for Client {}
fn source() -> PathBuf {
    std::env::var_os("MEMORY_PACKAGE_SOURCE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../plugins/personal-memory-engine")
        })
}
#[tokio::test]
async fn installed_catalog_records_without_developer_runtime() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path().join("owner's memory & space");
    let store = SqliteStore::open(&data.join("memory.sqlite3")).unwrap();
    let helper = std::env::var_os("MEMORY_PACKAGE_HELPER")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_memory-mcp")));
    let original_helper = dir.path().join("first app/Contents/MacOS/memory-mcp");
    std::fs::create_dir_all(original_helper.parent().unwrap()).unwrap();
    std::fs::copy(&helper, &original_helper).unwrap();
    let catalog = worker_package::prepare(&source(), &original_helper, &data).unwrap();
    let url = url::Url::parse(&worker_package::plugin_link(&catalog).unwrap()).unwrap();
    assert_eq!(
        url.query_pairs()
            .find(|(key, _)| key == "marketplacePath")
            .unwrap()
            .1,
        catalog.to_string_lossy()
    );
    let marketplace: Value = serde_json::from_slice(&std::fs::read(&catalog).unwrap()).unwrap();
    assert_eq!(marketplace["name"], worker_package::MARKETPLACE_NAME);
    let plugin = catalog
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(
            marketplace["plugins"][0]["source"]["path"]
                .as_str()
                .unwrap(),
        );
    assert!(plugin.join(".codex-plugin/plugin.json").is_file());
    assert!(plugin.join("skills/record-decisions/SKILL.md").is_file());
    let portable: Value =
        serde_json::from_slice(&std::fs::read(plugin.join("mcp.json")).unwrap()).unwrap();
    let legacy: Value =
        serde_json::from_slice(&std::fs::read(plugin.join(".mcp.json")).unwrap()).unwrap();
    assert_eq!(portable["mcpServers"], legacy["mcpServers"]);
    assert_eq!(
        portable["$schema"],
        "https://agent-plugins.org/schemas/1.0.0/mcp.schema.json"
    );
    let server = &portable["mcpServers"]["personal-memory"];
    let args: Vec<&str> = server["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect();
    let mut command = tokio::process::Command::new(server["command"].as_str().unwrap());
    command
        .args(&args)
        .env("PATH", "/usr/bin:/bin")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .kill_on_drop(true);
    let mut child = command.spawn().unwrap();
    let client = Client
        .serve((child.stdout.take().unwrap(), child.stdin.take().unwrap()))
        .await
        .unwrap();
    assert_eq!(client.list_all_tools().await.unwrap().len(), 3);
    let request = store.connection_control().begin_test().unwrap();
    let test_url = url::Url::parse(&worker_package::test_link(&request)).unwrap();
    let prompt = test_url
        .query_pairs()
        .find(|(key, _)| key == "prompt")
        .unwrap()
        .1
        .into_owned();
    assert_eq!(prompt, worker_package::test_prompt(&request));
    assert!(prompt.contains(&request));
    assert!(!prompt.contains("plugin://"));
    assert!(!prompt.contains("record_decision"));
    assert!(!prompt.contains("user_confirmed"));
    let result = client.call_tool(CallToolRequestParams::new("record_decision").with_arguments(json!({"request_id":request,"user_confirmed":true,"chosen_option":"A blue cover for my demo notebook","rationale":"I prefer blue","worker":"Isolated bundled connector test"}).as_object().unwrap().clone())).await.unwrap();
    assert_eq!(result.is_error, Some(false));
    assert!(
        store
            .connection_control()
            .status()
            .unwrap()
            .receipt
            .is_some()
    );
    client.cancel().await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), child.wait())
        .await
        .unwrap()
        .unwrap();
    // Move the real helper, remove the original, then use the unchanged cached MCP manifest.
    let moved_helper = dir
        .path()
        .join("moved owner's app & space/Contents/MacOS/memory-mcp");
    std::fs::create_dir_all(moved_helper.parent().unwrap()).unwrap();
    std::fs::rename(&original_helper, &moved_helper).unwrap();
    let before = serde_json::to_value(store.connection_control().status().unwrap()).unwrap();
    worker_package::refresh_if_prepared(&moved_helper, &data).unwrap();
    assert_eq!(
        before,
        serde_json::to_value(store.connection_control().status().unwrap()).unwrap()
    );
    store.connection_control().pause().unwrap();
    let paused_state = serde_json::to_value(store.connection_control().status().unwrap()).unwrap();
    worker_package::refresh_if_prepared(&moved_helper, &data).unwrap();
    assert_eq!(
        paused_state,
        serde_json::to_value(store.connection_control().status().unwrap()).unwrap()
    );
    let mut command = tokio::process::Command::new(server["command"].as_str().unwrap());
    let mut moved = command
        .args(&args)
        .env("PATH", "/usr/bin:/bin")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let client = Client
        .serve((moved.stdout.take().unwrap(), moved.stdin.take().unwrap()))
        .await
        .unwrap();
    let paused = client
        .call_tool(CallToolRequestParams::new("list_decisions"))
        .await
        .unwrap();
    assert_eq!(paused.is_error, Some(true));
    store.connection_control().resume().unwrap();
    let appended = client.call_tool(CallToolRequestParams::new("record_decision").with_arguments(json!({"request_id":"after-app-relocation","user_confirmed":true,"chosen_option":"Synthetic choice after moving the app","worker":"Isolated relocated helper test"}).as_object().unwrap().clone())).await.unwrap();
    assert_eq!(appended.is_error, Some(false));
    let records = client
        .call_tool(CallToolRequestParams::new("list_decisions"))
        .await
        .unwrap();
    assert_eq!(records.is_error, Some(false));
    assert!(
        serde_json::to_string(&records)
            .unwrap()
            .contains("A blue cover for my demo notebook")
    );
    client.cancel().await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), moved.wait())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        std::fs::read(plugin.join("mcp.json")).unwrap(),
        serde_json::to_vec_pretty(&portable).unwrap()
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            std::fs::metadata(&catalog).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
}
#[test]
fn incomplete_resources_do_not_create_a_discoverable_catalog() {
    let dir = tempfile::tempdir().unwrap();
    assert!(
        worker_package::prepare(dir.path(), &std::env::current_exe().unwrap(), dir.path()).is_err()
    );
    assert!(!worker_package::catalog_path(dir.path()).exists());
    assert!(worker_package::plugin_link(std::path::Path::new("relative/catalog.json")).is_err());
}

#[test]
fn automatic_refresh_does_not_create_connections_or_follow_symlinks() {
    let dir = tempfile::tempdir().unwrap();
    worker_package::refresh_if_prepared(&std::env::current_exe().unwrap(), dir.path()).unwrap();
    assert!(!dir.path().join("worker-plugins").exists());
    #[cfg(unix)]
    {
        let launcher = dir.path().join("worker-plugins/start-installed-mcp.sh");
        std::fs::create_dir(launcher.parent().unwrap()).unwrap();
        let unrelated = dir.path().join("unrelated file");
        std::fs::write(&unrelated, "unchanged").unwrap();
        std::os::unix::fs::symlink(&unrelated, &launcher).unwrap();
        assert!(
            worker_package::refresh_if_prepared(&std::env::current_exe().unwrap(), dir.path())
                .is_err()
        );
        assert_eq!(std::fs::read_to_string(unrelated).unwrap(), "unchanged");
    }
}
