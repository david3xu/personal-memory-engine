// Send bounded GitHub CLI requests without exposing credentials or accepting shell commands.
use serde_json::Value;
use std::{path::PathBuf, process::Stdio, time::Duration};
use tokio::{io::AsyncWriteExt, process::Command};
#[derive(Clone)]
pub struct GitHub {
    pub(crate) cli: PathBuf,
}
impl GitHub {
    pub fn new(cli: PathBuf) -> Self {
        Self { cli }
    }
    pub(crate) async fn api(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<Value>,
    ) -> Result<Value, String> {
        let mut command = Command::new(&self.cli);
        command
            .args([
                "api",
                "--hostname",
                "github.com",
                "--method",
                method,
                endpoint,
            ])
            .env("GH_PROMPT_DISABLED", "1")
            .env("GH_DEBUG", "")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        if body.is_some() {
            command.args(["--input", "-"]);
        }
        let mut child = command
            .spawn()
            .map_err(|_| "The bundled GitHub connector could not start.")?;
        if let Some(body) = body {
            let bytes = serde_json::to_vec(&body)
                .map_err(|_| "The publishing request could not be prepared")?;
            child
                .stdin
                .take()
                .ok_or("Publishing input unavailable")?
                .write_all(&bytes)
                .await
                .map_err(|_| "Publishing input could not be sent")?;
        } else {
            drop(child.stdin.take());
        }
        let output = tokio::time::timeout(Duration::from_secs(30), child.wait_with_output())
            .await
            .map_err(
                |_| "GitHub timed out. Check your network and refresh the shared link status.",
            )?
            .map_err(|_| "GitHub could not be contacted")?;
        if !output.status.success() {
            // Only distinguish a missing resource; never expose arbitrary CLI output or tokens.
            if String::from_utf8_lossy(&output.stderr).contains("HTTP 404") {
                return Err("not_found".into());
            }
            return Err("GitHub rejected the request. Check sign-in, repository permissions and network access.".into());
        }
        if output.stdout.is_empty() {
            return Ok(Value::Null);
        }
        serde_json::from_slice(&output.stdout)
            .map_err(|_| "GitHub returned an unsupported response.".into())
    }
    pub async fn account(&self) -> Result<String, String> {
        self.api("user", "GET", None).await?["login"]
            .as_str()
            .map(String::from)
            .ok_or("GitHub sign-in is unavailable.".into())
    }
}
