// Expose bounded owner onboarding actions around the bundled local connector.
use crate::DesktopState;
use memory_local_runtime::{
    connection::ConnectionStatus,
    host_connection::{DesktopHost, HostState},
    worker_package,
};
use serde::Serialize;
use tauri_plugin_opener::OpenerExt;
#[derive(Serialize)]
pub struct WorkerStatus {
    #[serde(flatten)]
    control: ConnectionStatus,
    package_available: bool,
    host_state: HostState,
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
    let host_state = match DesktopHost::installed() {
        Some(host) => host.state(&data_dir).await?,
        None => HostState::Unavailable,
    };
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
            host_state,
        })
    })
    .await
    .map_err(|_| "Worker setup could not be read".to_string())?
}
#[tauri::command]
pub async fn connect_chatgpt(state: tauri::State<'_, DesktopState>) -> Result<(), String> {
    let store = state.store.clone();
    let data_dir = state.data_dir.clone();
    let resources = state.plugin_source.clone();
    let helper = state.helper.clone();
    let host =
        DesktopHost::installed().ok_or("Install or update ChatGPT desktop before connecting.")?;
    let prepare_dir = data_dir.clone();
    tauri::async_runtime::spawn_blocking(move || {
        worker_package::prepare(&resources, &helper, &prepare_dir)
    })
    .await
    .map_err(|_| "The bundled connector could not be prepared".to_string())??;
    host.connect(&data_dir).await?;
    tauri::async_runtime::spawn_blocking(move || store.connection_control().resume())
        .await
        .map_err(|_| "Recording could not be enabled".to_string())??;
    // Registration is complete. Opening a plugin page is not part of the primary connection.
    Ok(())
}
#[tauri::command]
pub async fn start_connection_test(
    app: tauri::AppHandle,
    state: tauri::State<'_, DesktopState>,
) -> Result<(), String> {
    let store = state.store.clone();
    let data_dir = state.data_dir.clone();
    let host = DesktopHost::installed().ok_or("ChatGPT desktop is unavailable.")?;
    if host.state(&data_dir).await? != HostState::Registered {
        return Err("Choose Connect once before testing recording.".into());
    }
    let resources = state.plugin_source.clone();
    let helper = state.helper.clone();
    let link = tauri::async_runtime::spawn_blocking(move || {
        worker_package::prepare(&resources, &helper, &data_dir)?;
        let request_id = store.connection_control().begin_test()?;
        Ok::<_, String>(worker_package::test_link(&request_id))
    })
    .await
    .map_err(|_| "The setup test could not be prepared".to_string())??;
    app.opener().open_url(link, None::<&str>).map_err(|_| "The test chat could not be opened. Copy the test message below into a new local desktop chat. Work support needs a separate test.".to_string())
}
#[tauri::command]
pub async fn pause_recording(state: tauri::State<'_, DesktopState>) -> Result<(), String> {
    let store = state.store.clone();
    tauri::async_runtime::spawn_blocking(move || store.connection_control().pause())
        .await
        .map_err(|_| "Worker access could not be paused".to_string())?
}
