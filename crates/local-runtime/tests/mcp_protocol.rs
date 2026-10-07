// Exercise the real stdio executable through an MCP client, not direct handler calls.
use rmcp::{
    ClientHandler, RoleClient, ServiceExt,
    model::{CallToolRequestParams, CallToolResult},
    service::RunningService,
};
use serde_json::{Value, json};
use std::path::Path;
#[derive(Clone)]
struct Client;
impl ClientHandler for Client {}
async fn connect(path: &Path) -> (RunningService<RoleClient, Client>, tokio::process::Child) {
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_memory-mcp"));
    command
        .arg("--data-dir")
        .arg(path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .kill_on_drop(true);
    let mut child = command.spawn().unwrap();
    let transport = (child.stdout.take().unwrap(), child.stdin.take().unwrap());
    let client = Client.serve(transport).await.unwrap();
    (client, child)
}
async fn call(
    client: &RunningService<RoleClient, Client>,
    name: &str,
    args: Value,
) -> CallToolResult {
    client
        .call_tool(
            CallToolRequestParams::new(name.to_string())
                .with_arguments(args.as_object().unwrap().clone()),
        )
        .await
        .unwrap()
}
#[tokio::test]
async fn stdio_contract_retries_history_and_restart() {
    let dir = tempfile::tempdir().unwrap();
    let (client, mut child) = connect(dir.path()).await;
    let tools = client.list_all_tools().await.unwrap();
    assert_eq!(tools.len(), 3);
    assert!(tools.iter().all(|tool| tool.output_schema.is_some()));
    let input = json!({"request_id":"test-1","user_confirmed":true,"chosen_option":"A synthetic option","worker":"protocol-test"});
    let mut suggestion = input.clone();
    suggestion["user_confirmed"] = json!(false);
    assert_eq!(
        call(&client, "record_decision", suggestion).await.is_error,
        Some(true)
    );
    let first = call(&client, "record_decision", input.clone()).await;
    assert_eq!(first.is_error, Some(false));
    let value = first.structured_content.unwrap();
    assert!(value["submission"].get("rationale").is_none());
    let retry = call(&client, "record_decision", input.clone())
        .await
        .structured_content
        .unwrap();
    assert_eq!(value, retry);
    let mut unknown = input.clone();
    unknown["made_up_field"] = json!(true);
    // Invalid arguments may be rejected at JSON-RPC or tool-result level; either must not save.
    let result = client
        .call_tool(
            CallToolRequestParams::new("record_decision")
                .with_arguments(unknown.as_object().unwrap().clone()),
        )
        .await;
    assert!(result.is_err() || result.unwrap().is_error == Some(true));
    let mut revision = input;
    revision["request_id"] = json!("test-2");
    revision["chosen_option"] = json!("A revised synthetic option");
    revision["decision_id"] = value["decision_id"].clone();
    revision["supersedes_version_id"] = value["version_id"].clone();
    let second = call(&client, "record_decision", revision)
        .await
        .structured_content
        .unwrap();
    assert_ne!(value["version_id"], second["version_id"]);
    client.cancel().await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), child.wait())
        .await
        .unwrap()
        .unwrap();
    let (restarted, mut child) = connect(dir.path()).await;
    let latest = call(&restarted, "list_decisions", json!({}))
        .await
        .structured_content
        .unwrap();
    assert_eq!(latest["records"].as_array().unwrap().len(), 1);
    assert_eq!(latest["records"][0], second);
    let history = call(
        &restarted,
        "decision_history",
        json!({"decision_id":value["decision_id"]}),
    )
    .await
    .structured_content
    .unwrap();
    assert_eq!(history["records"][1], value);
    restarted.cancel().await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), child.wait())
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn onboarding_requires_fresh_saved_choice_and_pause_blocks_running_helpers() {
    use memory_local_runtime::{
        SqliteStore,
        connection::{TEST_CHOICE, TEST_REASON},
    };
    let dir = tempfile::tempdir().unwrap();
    let store = SqliteStore::open(&dir.path().join("memory.sqlite3")).unwrap();
    let control = store.connection_control();
    let (client, mut child) = connect(dir.path()).await;
    let request = control.begin_test().unwrap();
    assert!(
        !call(&client, "list_decisions", json!({}))
            .await
            .is_error
            .unwrap()
    );
    assert!(control.status().unwrap().receipt.is_none());
    let mut input = json!({"request_id":request,"user_confirmed":false,"chosen_option":TEST_CHOICE,"rationale":TEST_REASON,"worker":"synthetic protocol test"});
    assert!(
        call(&client, "record_decision", input.clone())
            .await
            .is_error
            .unwrap()
    );
    assert!(control.status().unwrap().receipt.is_none());
    input["user_confirmed"] = json!(true);
    let first = call(&client, "record_decision", input.clone())
        .await
        .structured_content
        .unwrap();
    assert_eq!(
        control.status().unwrap().receipt.unwrap().version_id,
        first["version_id"].as_str().unwrap()
    );
    let fresh_request = control.begin_test().unwrap();
    call(&client, "record_decision", input.clone()).await;
    assert!(
        control.status().unwrap().receipt.is_none(),
        "A prior exact retry cannot verify a fresh test"
    );
    input["request_id"] = json!(fresh_request);
    let fresh = call(&client, "record_decision", input.clone())
        .await
        .structured_content
        .unwrap();
    assert_eq!(
        control.status().unwrap().receipt.unwrap().version_id,
        fresh["version_id"].as_str().unwrap()
    );
    assert_eq!(
        call(&client, "record_decision", input.clone())
            .await
            .structured_content
            .unwrap(),
        fresh
    );
    control.pause().unwrap();
    assert!(
        call(&client, "list_decisions", json!({}))
            .await
            .is_error
            .unwrap()
    );
    assert!(
        call(
            &client,
            "decision_history",
            json!({"decision_id":first["decision_id"]})
        )
        .await
        .is_error
        .unwrap()
    );
    input["request_id"] = json!("paused-new-choice");
    assert!(
        call(&client, "record_decision", input)
            .await
            .is_error
            .unwrap()
    );
    assert_eq!(
        memory_engine::Engine::new(store.clone())
            .list()
            .unwrap()
            .len(),
        2,
        "Owner access and saved decisions survive pause"
    );
    client.cancel().await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), child.wait())
        .await
        .unwrap()
        .unwrap();
    let (restarted, mut child) = connect(dir.path()).await;
    assert!(
        call(&restarted, "list_decisions", json!({}))
            .await
            .is_error
            .unwrap()
    );
    control.resume().unwrap();
    assert_eq!(
        call(&restarted, "list_decisions", json!({}))
            .await
            .structured_content
            .unwrap()["records"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert!(control.status().unwrap().receipt.is_none());
    restarted.cancel().await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), child.wait())
        .await
        .unwrap()
        .unwrap();
}
