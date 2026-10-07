// Prepare an app-owned local plugin catalog and construct supported desktop host links.
use crate::connection::{TEST_CHOICE, TEST_REASON, private_write};
use serde_json::json;
use std::path::{Path, PathBuf};
pub const PLUGIN_NAME: &str = "personal-memory-engine";
pub const MARKETPLACE_NAME: &str = "personal-memory-engine-desktop";
pub const PACKAGE_VERSION: &str = "0.1.1";
const PACKAGE_FILES: &[&str] = &[
    "plugin.json",
    "mcp.json",
    ".codex-plugin/plugin.json",
    ".mcp.json",
    "skills/record-decisions/SKILL.md",
    "assets/logo.png",
    "LICENSE",
    "NOTICE",
    "scripts/start-mcp.sh",
];
fn private_directory(path: &Path) -> Result<(), String> {
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(path).map_err(|_| {
        "The bundled connector could not be prepared. Check local storage access.".to_string()
    })
}
pub fn catalog_path(data_dir: &Path) -> PathBuf {
    data_dir
        .join("worker-plugins")
        .join(PACKAGE_VERSION)
        .join(".agents/plugins/marketplace.json")
}
pub fn prepare(source: &Path, helper: &Path, data_dir: &Path) -> Result<PathBuf, String> {
    let helper = helper
        .canonicalize()
        .map_err(|_| "Recording helper is missing. Reinstall the app.".to_string())?;
    if !helper.is_file() || !data_dir.is_absolute() {
        return Err("The installed app's local paths are unavailable.".into());
    }
    // Validate all required resources before creating a catalog the host might discover.
    let files = PACKAGE_FILES
        .iter()
        .map(|relative| {
            let path = source.join(relative);
            let metadata = std::fs::symlink_metadata(&path)
                .map_err(|_| "Bundled connector is incomplete. Reinstall the app.".to_string())?;
            if !metadata.is_file() {
                return Err("Bundled connector contains an unsupported file.".into());
            }
            Ok((
                *relative,
                std::fs::read(path).map_err(|_| "Bundled connector cannot be read.".to_string())?,
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    for (relative, bytes) in &files {
        if *relative == "plugin.json" || *relative == ".codex-plugin/plugin.json" {
            let manifest: serde_json::Value = serde_json::from_slice(bytes)
                .map_err(|_| "Bundled connector manifest is invalid.".to_string())?;
            if manifest["name"] != PLUGIN_NAME || manifest["version"] != PACKAGE_VERSION {
                return Err("Bundled connector version does not match the app.".into());
            }
        }
    }
    let catalog = catalog_path(data_dir);
    let root = data_dir.join("worker-plugins").join(PACKAGE_VERSION);
    let plugin = root.join("plugins").join(PLUGIN_NAME);
    for (relative, bytes) in files {
        let target = plugin.join(relative);
        private_directory(target.parent().ok_or("Connector directory is missing")?)?;
        private_write(&target, &bytes)?;
    }
    // Installed copies read a stable app-owned launcher. Reconnecting after an app move updates
    // this launcher without depending on a cached manifest being reinstalled by the host.
    let launcher = data_dir.join("worker-plugins/start-installed-mcp.sh");
    let quote = |value: &str| format!("'{}'", value.replace('\'', "'\\''"));
    let script = format!(
        "#!/bin/bash\nset -euo pipefail\nexec {} --data-dir {}\n",
        quote(&helper.to_string_lossy()),
        quote(&data_dir.to_string_lossy())
    );
    private_write(&launcher, script.as_bytes())?;
    let server = json!({"mcpServers":{"personal-memory":{"type":"stdio","command":"/bin/bash","args":[launcher]}}});
    let bytes = serde_json::to_vec_pretty(&server).map_err(|error| error.to_string())?;
    private_write(&plugin.join("mcp.json"), &bytes)?;
    private_write(&plugin.join(".mcp.json"), &bytes)?;
    private_directory(catalog.parent().ok_or("Catalog directory is missing")?)?;
    let marketplace = json!({"name":MARKETPLACE_NAME,"interface":{"displayName":"Personal Memory Engine · installed app"},"plugins":[{"name":PLUGIN_NAME,"source":{"source":"local","path":"./plugins/personal-memory-engine"},"policy":{"installation":"AVAILABLE","authentication":"ON_INSTALL"},"category":"Productivity"}]});
    private_write(
        &catalog,
        &serde_json::to_vec_pretty(&marketplace).map_err(|error| error.to_string())?,
    )?;
    Ok(catalog)
}
pub fn plugin_link(catalog: &Path) -> Result<String, String> {
    if !catalog.is_absolute() {
        return Err("Local connector catalog must have an absolute path.".into());
    }
    let mut url = url::Url::parse(&format!("codex://plugins/{PLUGIN_NAME}"))
        .map_err(|error| error.to_string())?;
    url.query_pairs_mut()
        .append_pair("marketplacePath", &catalog.to_string_lossy());
    Ok(url.into())
}
pub fn test_prompt(request_id: &str) -> String {
    format!(
        "[@Personal Memory Engine](plugin://{PLUGIN_NAME}@{MARKETPLACE_NAME})\n\nThis is an explicit synthetic setup decision: I choose a blue cover for my demo notebook because I prefer blue. Please record this one choice using record_decision with request_id \"{request_id}\", user_confirmed true, chosen_option \"{TEST_CHOICE}\", and rationale \"{TEST_REASON}\". Use your actual worker name. I have not stated any rejected alternatives or source evidence; omit them. This is a new decision, not a revision. Confirm only after the tool succeeds. Do not record any other content from this setup chat."
    )
}
pub fn test_link(request_id: &str) -> String {
    let mut url = url::Url::parse("codex://new").expect("fixed desktop link is valid");
    url.query_pairs_mut()
        .append_pair("prompt", &test_prompt(request_id));
    url.into()
}
