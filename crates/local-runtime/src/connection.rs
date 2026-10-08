// Coordinate owner controls and worker access without changing append-only decision records.
use memory_engine::DecisionVersion;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub const TEST_CHOICE: &str = "A blue cover for my demo notebook";
pub const TEST_REASON: &str = "I prefer blue";
#[derive(Clone)]
pub struct ConnectionControl {
    state_path: PathBuf,
    lock_path: PathBuf,
}
#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TestReceipt {
    pub version_id: String,
    pub recorded_at: String,
    pub worker: String,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectionStatus {
    pub schema_version: u32,
    pub enabled: bool,
    pub test_request_id: Option<String>,
    pub receipt: Option<TestReceipt>,
}
impl Default for ConnectionStatus {
    fn default() -> Self {
        Self {
            schema_version: 1,
            enabled: true,
            test_request_id: None,
            receipt: None,
        }
    }
}
pub struct WorkerAccess {
    _lock: File,
    control: ConnectionControl,
    state: ConnectionStatus,
}
fn io_error(_: impl std::fmt::Display) -> String {
    "Worker setup could not be read or saved. Check local storage access.".into()
}
pub(crate) fn private_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let temporary = path.with_extension(format!("{}.tmp", Uuid::new_v4()));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let result = (|| {
        let mut file = options.open(&temporary).map_err(io_error)?;
        file.write_all(bytes).map_err(io_error)?;
        file.sync_all().map_err(io_error)?;
        std::fs::rename(&temporary, path).map_err(io_error)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(temporary);
    }
    result
}
impl ConnectionControl {
    pub fn for_database(database: &Path) -> Self {
        Self {
            state_path: database.with_extension("connection.json"),
            lock_path: database.with_extension("connection.lock"),
        }
    }
    fn locked(&self) -> Result<WorkerAccess, String> {
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let file = options.open(&self.lock_path).map_err(io_error)?;
        file.lock().map_err(io_error)?;
        let state: ConnectionStatus = match std::fs::read(&self.state_path) {
            Ok(bytes) => serde_json::from_slice(&bytes).map_err(io_error)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                ConnectionStatus::default()
            }
            Err(error) => return Err(io_error(error)),
        };
        if state.schema_version != 1 {
            return Err("Worker setup requires a newer app. No worker access was granted.".into());
        }
        Ok(WorkerAccess {
            _lock: file,
            control: self.clone(),
            state,
        })
    }
    pub fn status(&self) -> Result<ConnectionStatus, String> {
        Ok(self.locked()?.state.clone())
    }
    pub fn worker_access(&self) -> Result<WorkerAccess, String> {
        let access = self.locked()?;
        if !access.state.enabled {
            return Err("Worker access is paused in Personal Memory Engine. Ask the owner to reconnect in the app.".into());
        }
        Ok(access)
    }
    pub fn resume(&self) -> Result<(), String> {
        let mut access = self.locked()?;
        access.state.enabled = true;
        access.save()
    }
    pub fn pause(&self) -> Result<(), String> {
        let mut access = self.locked()?;
        access.state.enabled = false;
        access.state.test_request_id = None;
        access.state.receipt = None;
        access.save()
    }
    pub fn begin_test(&self) -> Result<String, String> {
        let mut access = self.worker_access()?;
        let request_id = format!("onboarding-{}", Uuid::new_v4());
        access.state.test_request_id = Some(request_id.clone());
        access.state.receipt = None;
        access.save()?;
        Ok(request_id)
    }
}
impl WorkerAccess {
    fn save(&self) -> Result<(), String> {
        private_write(
            &self.control.state_path,
            &serde_json::to_vec(&self.state).map_err(io_error)?,
        )
    }
    pub fn note_record(&mut self, record: &DecisionVersion) -> Result<(), String> {
        let input = &record.submission;
        if self.state.test_request_id.as_deref() == Some(&input.request_id)
            && input.chosen_option == TEST_CHOICE
            && input.rationale.as_deref() == Some(TEST_REASON)
            && input.supersedes_version_id.is_none()
        {
            self.state.receipt = Some(TestReceipt {
                version_id: record.version_id.clone(),
                recorded_at: record.recorded_at.clone(),
                worker: input.worker.clone(),
            });
            self.save()?;
        }
        Ok(())
    }
}
