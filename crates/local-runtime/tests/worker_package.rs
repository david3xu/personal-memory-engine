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
    let catalog = worker_package::prepare(&source(), &helper, &data).unwrap();
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
    assert_eq!(portable, legacy);
    let server = &portable["mcpServers"]["personal-memory"];
    let args: Vec<&str> = server["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect();
    let mut command = tokio::process::Command::new(server["command"].as_str().unwrap());
    command
        .args(args)
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
    assert!(prompt.contains("plugin://personal-memory-engine@personal-memory-engine-desktop"));
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
    // Existing installed manifests use the stable launcher; reconnect can repair helper relocation.
    worker_package::prepare(&source(), &helper, &data).unwrap();
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
