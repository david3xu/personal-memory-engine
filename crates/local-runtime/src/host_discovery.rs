// Locate a compatible desktop connection manager without depending on one installation directory.
use std::path::{Path, PathBuf};
use tokio::process::Command;

fn executable(path: &Path) -> bool {
    if !path.is_absolute() || !path.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .is_ok_and(|meta| meta.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    true
}
fn cli_in_app(app: &Path) -> Option<PathBuf> {
    let cli = app.join("Contents/Resources/codex-cli/bin/codex");
    executable(&cli).then_some(cli)
}
fn standard_apps(home: Option<&Path>) -> Vec<PathBuf> {
    let mut roots = vec![PathBuf::from("/Applications")];
    if let Some(home) = home {
        roots.push(home.join("Applications"));
    }
    roots
        .into_iter()
        .flat_map(|root| [root.join("ChatGPT.app"), root.join("Codex.app")])
        .collect()
}
fn indexed_cli(output: &[u8]) -> Option<PathBuf> {
    output
        .split(|byte| *byte == 0)
        .filter_map(|bytes| std::str::from_utf8(bytes).ok())
        .find_map(|path| cli_in_app(Path::new(path)))
}
pub async fn find_cli() -> Option<PathBuf> {
    // Explicit overrides fail closed; a stale override must not select another installation.
    if let Some(path) = std::env::var_os("PERSONAL_MEMORY_HOST_CLI") {
        let path = PathBuf::from(path);
        return executable(&path).then_some(path);
    }
    if let Some(cli) = standard_apps(dirs::home_dir().as_deref())
        .iter()
        .find_map(|app| cli_in_app(app))
    {
        return Some(cli);
    }
    if !cfg!(target_os = "macos") {
        return None;
    }
    // This bundle ID was verified on the compatible installed desktop host. Other ChatGPT
    // bundles are not assumed to have the same MCP manager. Spotlight may be unavailable.
    let output = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        Command::new("/usr/bin/mdfind")
            .args(["-0", "kMDItemCFBundleIdentifier == 'com.openai.codex'"])
            .kill_on_drop(true)
            .output(),
    )
    .await
    .ok()?
    .ok()?;
    output
        .status
        .success()
        .then(|| indexed_cli(&output.stdout))
        .flatten()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relocated_indexed_app_requires_an_executable_helper() {
        let dir = tempfile::tempdir().unwrap();
        let app = dir.path().join("owner's custom folder/Renamed desktop.app");
        let cli = app.join("Contents/Resources/codex-cli/bin/codex");
        std::fs::create_dir_all(cli.parent().unwrap()).unwrap();
        std::fs::write(&cli, "synthetic connection manager").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&cli, std::fs::Permissions::from_mode(0o600)).unwrap();
            assert!(cli_in_app(&app).is_none());
            std::fs::set_permissions(&cli, std::fs::Permissions::from_mode(0o700)).unwrap();
        }
        let inventory = format!("/missing/app\0{}\0", app.display());
        assert_eq!(indexed_cli(inventory.as_bytes()), Some(cli));
        assert!(!executable(Path::new("relative/connection-manager")));
    }
    #[test]
    fn standard_locations_include_per_user_and_renamed_host() {
        let apps = standard_apps(Some(Path::new("/synthetic owner")));
        assert!(apps.contains(&PathBuf::from("/Applications/Codex.app")));
        assert!(apps.contains(&PathBuf::from("/synthetic owner/Applications/ChatGPT.app")));
    }
}
