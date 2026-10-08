// Register this app's MCP using the desktop host's bundled CLI without replacing other servers.
use serde::Serialize;
use serde_json::Value;
use std::path::{Path, PathBuf};
use tokio::process::Command;

pub const SERVER_NAME: &str = "personal-memory-engine";
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HostState {
    Unavailable,
    Missing,
    Registered,
    Disabled,
    Conflict,
}
pub struct DesktopHost {
    cli: PathBuf,
    config_home: Option<PathBuf>,
}
impl DesktopHost {
    pub async fn installed() -> Option<Self> {
        crate::host_discovery::find_cli().await.map(|cli| Self {
            cli,
            config_home: None,
        })
    }
    // Explicit test configuration never changes the owner's host or launches a model.
    pub fn isolated(cli: PathBuf, config_home: PathBuf) -> Self {
        Self {
            cli,
            config_home: Some(config_home),
        }
    }
    async fn run(&self, args: &[&std::ffi::OsStr]) -> Result<std::process::Output, String> {
        let mut command = Command::new(&self.cli);
        command.args(args).kill_on_drop(true);
        if let Some(home) = &self.config_home {
            command.env("CODEX_HOME", home).current_dir(home);
        } else if let Some(home) = dirs::home_dir() {
            command.current_dir(home);
        }
        tokio::time::timeout(std::time::Duration::from_secs(10), command.output())
            .await
            .map_err(|_| "ChatGPT connection setup timed out. Try Connect again.".to_string())?
            .map_err(|_| {
                "ChatGPT's connection manager could not start. Update its desktop app.".to_string()
            })
    }
    pub async fn state(&self, data_dir: &Path) -> Result<HostState, String> {
        let output = self
            .run(&[
                "mcp".as_ref(),
                "get".as_ref(),
                SERVER_NAME.as_ref(),
                "--json".as_ref(),
            ])
            .await?;
        if !output.status.success() {
            // A failed config read must never be treated as permission to replace a server.
            if String::from_utf8_lossy(&output.stderr)
                .contains(&format!("No MCP server named '{SERVER_NAME}' found"))
            {
                return Ok(HostState::Missing);
            }
            return Err("ChatGPT's MCP settings could not be read. Check its MCP settings before reconnecting.".into());
        }
        let server: Value = serde_json::from_slice(&output.stdout)
            .map_err(|_| "ChatGPT returned an unsupported connection format.".to_string())?;
        Ok(classify(&server, data_dir))
    }
    pub async fn connect(&self, data_dir: &Path) -> Result<(), String> {
        match self.state(data_dir).await? {
            HostState::Registered => return Ok(()),
            HostState::Conflict => return Err("A different MCP already uses the name personal-memory-engine. Rename it in ChatGPT's MCP settings before connecting; it has not been changed.".into()),
            HostState::Disabled => return Err("Enable Personal Memory Engine and its recording tools in ChatGPT's MCP settings, then reconnect.".into()),
            HostState::Unavailable => return Err("Install or update ChatGPT desktop before connecting.".into()),
            HostState::Missing => {}
        }
        let launcher = data_dir.join("worker-plugins/start-installed-mcp.sh");
        let output = self
            .run(&[
                "mcp".as_ref(),
                "add".as_ref(),
                SERVER_NAME.as_ref(),
                "--".as_ref(),
                "/bin/bash".as_ref(),
                launcher.as_os_str(),
            ])
            .await?;
        if !output.status.success() || self.state(data_dir).await? != HostState::Registered {
            return Err(
                "The recording server could not be registered. Your saved decisions are unchanged."
                    .into(),
            );
        }
        Ok(())
    }
}
fn classify(server: &Value, data_dir: &Path) -> HostState {
    let transport = &server["transport"];
    let launcher = data_dir.join("worker-plugins/start-installed-mcp.sh");
    let matching = server["name"] == SERVER_NAME
        && transport["type"] == "stdio"
        && transport["command"] == "/bin/bash"
        && transport["args"] == serde_json::json!([launcher])
        && transport["env"]
            .as_object()
            .is_none_or(|env| env.is_empty())
        && transport["env_vars"]
            .as_array()
            .is_none_or(|vars| vars.is_empty())
        && transport["cwd"].is_null();
    if !matching {
        return HostState::Conflict;
    }
    let required = ["record_decision", "list_decisions", "decision_history"];
    let excluded = server["disabled_tools"].as_array().is_some_and(|tools| {
        required
            .iter()
            .any(|name| tools.iter().any(|tool| tool == name))
    }) || server["enabled_tools"].as_array().is_some_and(|tools| {
        required
            .iter()
            .any(|name| !tools.iter().any(|tool| tool == name))
    });
    if server["enabled"] != true || excluded {
        HostState::Disabled
    } else {
        HostState::Registered
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conflicting_and_restricted_servers_are_not_ready() {
        let dir = Path::new("/private/test memory");
        let mut config = serde_json::json!({"name":SERVER_NAME,"enabled":true,"transport":{"type":"stdio","command":"/bin/bash","args":[dir.join("worker-plugins/start-installed-mcp.sh")],"env":null,"env_vars":[],"cwd":null}});
        assert_eq!(classify(&config, dir), HostState::Registered);
        config["disabled_tools"] = serde_json::json!(["record_decision"]);
        assert_eq!(classify(&config, dir), HostState::Disabled);
        config["transport"]["command"] = serde_json::json!("another-worker");
        assert_eq!(classify(&config, dir), HostState::Conflict);
    }
}
