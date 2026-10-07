// Expose bounded owner onboarding actions around the bundled local connector.
use crate::DesktopState;
use memory_local_runtime::{connection::ConnectionStatus, worker_package};
use serde::Serialize;
use tauri_plugin_opener::OpenerExt;
#[derive(Serialize)]
pub struct WorkerStatus {
    #[serde(flatten)]
    control: ConnectionStatus,
    package_available: bool,
    catalog_prepared: bool,
    test_prompt: Option<String>,
}
#[tauri::command]
pub async fn connection_status(
    state: tauri::State<'_, DesktopState>,
) -> Result<WorkerStatus, String> {
    let store = state.store.clone();
    let data_dir = state.data_dir.clone();
    let resources = state.plugin_source.clone();
    let helper = state.helper.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let control = store.connection_control().status()?;
        let test_prompt = control
            .test_request_id
            .as_deref()
            .map(worker_package::test_prompt);
        Ok(WorkerStatus {
            control,
            test_prompt,
            package_available: helper.is_file()
                && resources.join(".codex-plugin/plugin.json").is_file(),
            catalog_prepared: worker_package::catalog_path(&data_dir).is_file(),
        })
    })
    .await
    .map_err(|_| "Worker setup could not be read".to_string())?
}
#[tauri::command]
pub async fn connect_chatgpt(
    app: tauri::AppHandle,
    state: tauri::State<'_, DesktopState>,
) -> Result<(), String> {
    let store = state.store.clone();
    let data_dir = state.data_dir.clone();
    let resources = state.plugin_source.clone();
    let helper = state.helper.clone();
    let link = tauri::async_runtime::spawn_blocking(move || {
        let catalog = worker_package::prepare(&resources, &helper, &data_dir)?;
        store.connection_control().resume()?;
        worker_package::plugin_link(&catalog)
    })
    .await
    .map_err(|_| "The bundled connector could not be prepared".to_string())??;
    app.opener().open_url(link, None::<&str>).map_err(|_| "ChatGPT could not be opened. Install its desktop app with Work/plugin support, then try Connect ChatGPT again.".to_string())
}
#[tauri::command]
pub async fn start_connection_test(
    app: tauri::AppHandle,
    state: tauri::State<'_, DesktopState>,
) -> Result<(), String> {
    let store = state.store.clone();
    let data_dir = state.data_dir.clone();
    let link = tauri::async_runtime::spawn_blocking(move || {
        if !worker_package::catalog_path(&data_dir).is_file() {
            return Err("Choose Connect ChatGPT and install the connector first.".into());
        }
        let request_id = store.connection_control().begin_test()?;
        Ok::<_, String>(worker_package::test_link(&request_id))
    })
    .await
    .map_err(|_| "The setup test could not be prepared".to_string())??;
    app.opener().open_url(link, None::<&str>).map_err(|_| "The test chat could not be opened. Copy the test message below into a desktop Work chat with Personal Memory Engine enabled.".to_string())
}
#[tauri::command]
pub async fn pause_recording(state: tauri::State<'_, DesktopState>) -> Result<(), String> {
    let store = state.store.clone();
    tauri::async_runtime::spawn_blocking(move || store.connection_control().pause())
        .await
        .map_err(|_| "Worker access could not be paused".to_string())?
}
