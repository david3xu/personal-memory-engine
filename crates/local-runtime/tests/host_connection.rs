// Verify real desktop-host registration and tool discovery in an isolated configuration.
use memory_local_runtime::{
    host_connection::{DesktopHost, HostState, SERVER_NAME},
    worker_package,
};
use serde_json::{Value, json};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::test]
#[ignore = "requires MEMORY_HOST_CLI pointing to the installed desktop host; no model is called"]
async fn real_host_registers_discovers_and_preserves_unrelated_settings() {
    let cli =
        PathBuf::from(std::env::var_os("MEMORY_HOST_CLI").expect("MEMORY_HOST_CLI is required"));
    assert!(
        DesktopHost::installed().await.is_some(),
        "installed compatible host is discoverable"
    );
    let dir = tempfile::tempdir().unwrap();
    let config_home = dir.path().join("isolated host");
    std::fs::create_dir(&config_home).unwrap();
    std::fs::write(config_home.join("config.toml"), "# owner's unrelated setting\nmodel_reasoning_effort = \"low\"\n[mcp_servers.unrelated]\ncommand = \"/bin/false\"\nenabled = false\n").unwrap();
    let data = dir.path().join("owner's synthetic memory");
    let source =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../plugins/personal-memory-engine");
    let helper = std::env::var_os("MEMORY_PACKAGE_HELPER")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_memory-mcp")));
    worker_package::prepare(&source, &helper, &data).unwrap();
    let host = DesktopHost::isolated(cli.clone(), config_home.clone());
    assert_eq!(host.state(&data).await.unwrap(), HostState::Missing);
    host.connect(&data).await.unwrap();
    assert_eq!(host.state(&data).await.unwrap(), HostState::Registered);
    let registered = std::fs::read(config_home.join("config.toml")).unwrap();
    let text = String::from_utf8(registered.clone()).unwrap();
    assert!(text.contains("# owner's unrelated setting"));
    assert!(text.contains("[mcp_servers.unrelated]"));
    assert!(text.contains("model_reasoning_effort = \"low\""));
    host.connect(&data).await.unwrap();
    assert_eq!(
        std::fs::read(config_home.join("config.toml")).unwrap(),
        registered
    );
    // The same fixed server name pointed elsewhere must not be replaced.
    let other = dir.path().join("another store");
    assert_eq!(host.state(&other).await.unwrap(), HostState::Conflict);
    assert!(host.connect(&other).await.is_err());
    assert_eq!(
        std::fs::read(config_home.join("config.toml")).unwrap(),
        registered
    );

    let mut child = tokio::process::Command::new(cli)
        .args(["app-server", "--stdio"])
        .env("CODEX_HOME", &config_home)
        .current_dir(&config_home)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    let mut output = BufReader::new(child.stdout.take().unwrap()).lines();
    async fn send(input: &mut tokio::process::ChildStdin, value: Value) {
        input
            .write_all(format!("{value}\n").as_bytes())
            .await
            .unwrap();
    }
    async fn receive(
        output: &mut tokio::io::Lines<BufReader<tokio::process::ChildStdout>>,
        id: u64,
    ) -> Value {
        tokio::time::timeout(std::time::Duration::from_secs(30), async {
            while let Some(line) = output.next_line().await.unwrap() {
                let value: Value = serde_json::from_str(&line).unwrap();
                if value["id"] == id {
                    return value;
                }
            }
            panic!("host closed before replying")
        })
        .await
        .expect("host discovery timed out")
    }
    send(&mut input, json!({"id":1,"method":"initialize","params":{"clientInfo":{"name":"memory_connection_test","version":"0.1.0"},"capabilities":{"experimentalApi":true,"explicitGatewayOauth":true}}})).await;
    assert!(receive(&mut output, 1).await.get("error").is_none());
    send(&mut input, json!({"method":"initialized","params":{}})).await;
    send(&mut input, json!({"id":2,"method":"mcpServerStatus/list","params":{"serverName":SERVER_NAME,"detail":"full"}})).await;
    let response = receive(&mut output, 2).await;
    assert!(
        response.get("error").is_none(),
        "discovery returned an error"
    );
    let servers = response["result"]["data"]
        .as_array()
        .expect("status inventory");
    let memory = servers
        .iter()
        .find(|server| server["name"] == SERVER_NAME)
        .expect("registered memory server");
    let tools = memory["tools"].as_object().expect("tool catalog");
    assert!(tools.keys().any(|name| name.ends_with("record_decision")));
    assert!(tools.keys().any(|name| name.ends_with("list_decisions")));
    assert!(tools.keys().any(|name| name.ends_with("decision_history")));
    println!("Desktop host discovered all three memory tools; no chat or model was started.");
    child.kill().await.unwrap();
    child.wait().await.unwrap();
}
