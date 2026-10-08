// Expose owner-only preview, GitHub connection and publication lifecycle commands, separate from MCP.
use crate::DesktopState;
use memory_github_publication::{login::LoginState, public_url, verify_live};
use memory_local_runtime::sharing::Publication;
use memory_public_snapshot::render;
use serde::Serialize;
use tauri_plugin_opener::OpenerExt;
#[derive(Serialize)]
pub struct ShareStatus {
    repository: String,
    entries: Vec<Publication>,
    login: LoginState,
}
#[tauri::command]
pub async fn sharing_status(state: tauri::State<'_, DesktopState>) -> Result<ShareStatus, String> {
    Ok(ShareStatus {
        repository: state.shares.repository()?,
        entries: state.shares.entries()?,
        login: state.github_login.lock().await.clone(),
    })
}
#[tauri::command]
pub async fn github_account(state: tauri::State<'_, DesktopState>) -> Result<String, String> {
    state.github.account().await
}
#[tauri::command]
pub async fn connect_github(state: tauri::State<'_, DesktopState>) -> Result<(), String> {
    state.github.login(state.github_login.clone()).await
}
#[tauri::command]
pub fn open_github_signin(app: tauri::AppHandle) -> Result<(), String> {
    app.opener()
        .open_url("https://github.com/login/device", None::<&str>)
        .map_err(|_| "The sign-in page could not be opened".into())
}
#[tauri::command]
pub async fn prepare_snapshot(
    ids: Vec<String>,
    title: String,
    history: bool,
    evidence: bool,
    state: tauri::State<'_, DesktopState>,
) -> Result<Publication, String> {
    let store = state.store.clone();
    let shares = state.shares.clone();
    tauri::async_runtime::spawn_blocking(move || {
        shares.prepare(&store, &ids, &title, history, evidence)
    })
    .await
    .map_err(|_| "The sharing preview could not be prepared")?
}
#[tauri::command]
pub async fn publish_snapshot(
    id: String,
    repository: String,
    state: tauri::State<'_, DesktopState>,
) -> Result<Publication, String> {
    let _guard = state.publication_busy.lock().await;
    let mut entry = state.shares.get(&id)?;
    if entry.status != "draft" && entry.status != "pending" {
        return Err("Prepare a new preview to publish another snapshot.".into());
    }
    if entry
        .repository
        .as_ref()
        .is_some_and(|old| old != &repository)
    {
        return Err(
            "This preview is already assigned to a repository. Prepare a new preview to change it."
                .into(),
        );
    }
    let html = render::html(&entry.snapshot, &entry.id);
    if html.len() > 900_000 {
        return Err("This public preview is too large. Select fewer decisions or versions.".into());
    }
    entry.url = Some(public_url(&repository, &id)?);
    entry.repository = Some(repository.clone());
    entry.status = "pending".into();
    state.shares.update(entry.clone())?;
    // Pending is persisted before network writes so an interrupted attempt can be verified later.
    let submitted = state.github.publish(&repository, &id, &html).await?;
    entry.commit = Some(submitted.commit);
    entry.url = Some(submitted.url);
    state.shares.update(entry.clone())?;
    Ok(entry)
}
#[tauri::command]
pub async fn verify_snapshot(
    id: String,
    state: tauri::State<'_, DesktopState>,
) -> Result<Publication, String> {
    let _guard = state.publication_busy.lock().await;
    let mut entry = state.shares.get(&id)?;
    if entry.status == "pending" || entry.status == "withdrawal_pending" {
        let html = if entry.status == "withdrawal_pending" {
            render::withdrawn(&id)
        } else {
            render::html(&entry.snapshot, &id)
        };
        if verify_live(
            entry.url.as_deref().ok_or("Publication URL unavailable")?,
            &html,
        )
        .await?
        {
            entry.status = if entry.status == "withdrawal_pending" {
                "withdrawn"
            } else {
                "live"
            }
            .into();
            state.shares.update(entry.clone())?;
        }
    }
    Ok(entry)
}
#[tauri::command]
pub async fn withdraw_snapshot(
    id: String,
    state: tauri::State<'_, DesktopState>,
) -> Result<Publication, String> {
    let _guard = state.publication_busy.lock().await;
    let mut entry = state.shares.get(&id)?;
    if !matches!(
        entry.status.as_str(),
        "live" | "pending" | "withdrawal_pending"
    ) {
        return Err("This snapshot has not been published.".into());
    }
    entry.status = "withdrawal_pending".into();
    state.shares.update(entry.clone())?;
    let submitted = state
        .github
        .publish(
            entry
                .repository
                .as_deref()
                .ok_or("Publication repository unavailable")?,
            &id,
            &render::withdrawn(&id),
        )
        .await?;
    entry.commit = Some(submitted.commit);
    state.shares.update(entry.clone())?;
    Ok(entry)
}
#[tauri::command]
pub fn open_snapshot(
    id: String,
    state: tauri::State<'_, DesktopState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let entry = state.shares.get(&id)?;
    if entry.status != "live" {
        return Err("The public page is not yet verified live.".into());
    }
    let url = public_url(
        entry
            .repository
            .as_deref()
            .ok_or("Publication repository unavailable")?,
        &id,
    )?;
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|_| "The public page could not be opened".into())
}
