// Deliver the owner desktop around reusable decision operations and separate adapters.
mod sharing;
mod worker_setup;
use memory_engine::{DecisionVersion, Engine};
use memory_local_runtime::{SqliteStore, paths::default_data_dir};
use serde::Serialize;
use sharing::{
    connect_github, github_account, open_github_signin, open_snapshot, prepare_snapshot,
    publish_snapshot, sharing_status, verify_snapshot, withdraw_snapshot,
};
use std::path::PathBuf;
use tauri::Manager;
use tauri_plugin_opener::OpenerExt;
use worker_setup::{connect_chatgpt, connection_status, pause_recording, start_connection_test};
struct DesktopState {
    store: SqliteStore,
    data_dir: PathBuf,
    helper: PathBuf,
    plugin_source: PathBuf,
    shares: memory_local_runtime::sharing::ShareStore,
    github: memory_github_publication::GitHub,
    github_login: std::sync::Arc<tokio::sync::Mutex<memory_github_publication::login::LoginState>>,
    publication_busy: tokio::sync::Mutex<()>,
}
#[derive(Serialize)]
struct LocalStatus {
    data_dir: String,
    database_path: String,
    helper_path: String,
    helper_available: bool,
    last_tool_use: Option<String>,
    app_version: &'static str,
}
#[tauri::command]
async fn local_status(state: tauri::State<'_, DesktopState>) -> Result<LocalStatus, String> {
    let store = state.store.clone();
    let data_dir = state.data_dir.clone();
    let helper = state.helper.clone();
    tauri::async_runtime::spawn_blocking(move || {
        Ok(LocalStatus {
            database_path: data_dir
                .join("memory.sqlite3")
                .to_string_lossy()
                .into_owned(),
            data_dir: data_dir.to_string_lossy().into_owned(),
            helper_available: helper.is_file(),
            helper_path: helper.to_string_lossy().into_owned(),
            last_tool_use: store.last_mcp_use().map_err(|error| error.to_string())?,
            app_version: env!("CARGO_PKG_VERSION"),
        })
    })
    .await
    .map_err(|_| "Local status could not be read".to_string())?
}
#[tauri::command]
async fn list_decisions(
    state: tauri::State<'_, DesktopState>,
) -> Result<Vec<DecisionVersion>, String> {
    let store = state.store.clone();
    tauri::async_runtime::spawn_blocking(move || {
        Engine::new(store).list().map_err(|error| error.to_string())
    })
    .await
    .map_err(|_| "Decisions could not be read".to_string())?
}
#[tauri::command]
async fn decision_history(
    decision_id: String,
    state: tauri::State<'_, DesktopState>,
) -> Result<Vec<DecisionVersion>, String> {
    let store = state.store.clone();
    tauri::async_runtime::spawn_blocking(move || {
        Engine::new(store)
            .history(&decision_id)
            .map_err(|error| error.to_string())
    })
    .await
    .map_err(|_| "History could not be read".to_string())?
}
#[tauri::command]
fn open_connection_docs(app: tauri::AppHandle) -> Result<(), String> {
    // A fixed help destination keeps arbitrary URL opening out of the interface boundary.
    app.opener()
        .open_url("https://github.com/david3xu/personal-memory-engine/blob/074f841099aa11cb490d58dbaba542fc02d2c955/docs/user-guides/install-and-connect.md", None::<&str>)
        .map_err(|error| error.to_string())
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // An explicit override is only for isolated development/testing; installs use OS storage.
            let data_dir = match std::env::var_os("PERSONAL_MEMORY_DATA_DIR") {
                Some(path) => PathBuf::from(path),
                None => default_data_dir()?,
            };
            let store = SqliteStore::open(&data_dir.join("memory.sqlite3"))?;
            let helper = std::env::current_exe()?.with_file_name(if cfg!(windows) {
                "memory-mcp.exe"
            } else {
                "memory-mcp"
            });
            let shares = memory_local_runtime::sharing::ShareStore::new(&data_dir);
            let github_cli = app
                .path()
                .resolve("github-cli/gh", tauri::path::BaseDirectory::Resource)?;
            app.manage(DesktopState {
                shares,
                github: memory_github_publication::GitHub::new(github_cli),
                github_login: Default::default(),
                publication_busy: Default::default(),
                store,
                data_dir,
                helper,
                plugin_source: app
                    .path()
                    .resolve("plugin-source", tauri::path::BaseDirectory::Resource)?,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            local_status,
            list_decisions,
            decision_history,
            open_connection_docs,
            sharing_status,
            github_account,
            connect_github,
            open_github_signin,
            prepare_snapshot,
            publish_snapshot,
            verify_snapshot,
            withdraw_snapshot,
            open_snapshot,
            connection_status,
            connect_chatgpt,
            start_connection_test,
            pause_recording
        ])
        .run(tauri::generate_context!())
        .expect("Personal Memory Engine could not open; check local storage access");
}
