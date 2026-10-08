// Start the bundled CLI's browser sign-in and expose only the one-time device code and completion state.
use crate::GitHub;
use serde::Serialize;
use std::{process::Stdio, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::Mutex,
};
#[derive(Clone, Default, Serialize)]
pub struct LoginState {
    pub code: Option<String>,
    pub running: bool,
    pub error: Option<String>,
}
impl GitHub {
    pub async fn login(&self, progress: Arc<Mutex<LoginState>>) -> Result<(), String> {
        let mut state = progress.lock().await;
        if state.running {
            return Ok(());
        }
        *state = LoginState {
            running: true,
            ..Default::default()
        };
        drop(state);
        let cli = self.cli.clone();
        tokio::spawn(async move {
            let result = async {
                let mut child = tokio::process::Command::new(cli)
                    .args([
                        "auth",
                        "login",
                        "--hostname",
                        "github.com",
                        "--web",
                        "--git-protocol",
                        "https",
                        "--skip-ssh-key",
                    ])
                    .env("GH_DEBUG", "")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::piped())
                    .kill_on_drop(true)
                    .spawn()
                    .map_err(|_| "GitHub sign-in could not start")?;
                let mut lines = BufReader::new(
                    child
                        .stderr
                        .take()
                        .ok_or("GitHub sign-in output unavailable")?,
                )
                .lines();
                tokio::time::timeout(Duration::from_secs(900), async {
                    while let Some(line) = lines
                        .next_line()
                        .await
                        .map_err(|_| "GitHub sign-in failed")?
                    {
                        if let Some(code) = line
                            .split("one-time code:")
                            .nth(1)
                            .map(str::trim)
                            .filter(|code| {
                                code.len() == 9
                                    && code.chars().all(|c| {
                                        c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-'
                                    })
                            })
                        {
                            progress.lock().await.code = Some(code.into());
                        }
                    }
                    if child
                        .wait()
                        .await
                        .map_err(|_| "GitHub sign-in failed")?
                        .success()
                    {
                        Ok(())
                    } else {
                        Err("GitHub sign-in was not completed")
                    }
                })
                .await
                .map_err(|_| "GitHub sign-in expired")?
            }
            .await;
            let mut state = progress.lock().await;
            state.running = false;
            state.code = None;
            if let Err(error) = result {
                state.error = Some(error.to_string());
            }
        });
        Ok(())
    }
}
