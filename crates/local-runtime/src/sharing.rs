// Freeze selected public projections and persist their separate publication lifecycle without changing memory.
use crate::{SqliteStore, connection::private_write};
use memory_engine::Engine;
use memory_public_snapshot::{Snapshot, project};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use uuid::Uuid;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    pub id: String,
    pub snapshot: Snapshot,
    pub repository: Option<String>,
    pub url: Option<String>,
    pub commit: Option<String>,
    pub status: String,
}
#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Ledger {
    schema_version: u32,
    repository: String,
    entries: Vec<Publication>,
}
#[derive(Clone)]
pub struct ShareStore {
    path: PathBuf,
    mutex: Arc<Mutex<()>>,
}
impl ShareStore {
    pub fn new(data_dir: &Path) -> Self {
        Self {
            path: data_dir.join("memory.publications.json"),
            mutex: Arc::new(Mutex::new(())),
        }
    }
    fn file_lock(&self) -> Result<std::fs::File, String> {
        let mut options = std::fs::OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let file = options
            .open(self.path.with_extension("lock"))
            .map_err(|_| "Publication lock unavailable")?;
        file.lock().map_err(|_| "Publication lock unavailable")?;
        Ok(file)
    }
    fn read(&self) -> Result<Ledger, String> {
        match std::fs::read(&self.path) {
            Ok(bytes) => {
                let ledger:Ledger=serde_json::from_slice(&bytes).map_err(|_| "Publication settings are unreadable. Your private decisions have not changed.")?;
                if ledger.schema_version != 1 {
                    return Err("Publication settings require a newer app.".into());
                }
                Ok(ledger)
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Ledger {
                schema_version: 1,
                ..Default::default()
            }),
            Err(_) => Err("Publication settings could not be read.".into()),
        }
    }
    fn save(&self, ledger: &Ledger) -> Result<(), String> {
        private_write(
            &self.path,
            &serde_json::to_vec(ledger).map_err(|_| "Publication state could not be saved")?,
        )
    }
    pub fn repository(&self) -> Result<String, String> {
        let _guard = self
            .mutex
            .lock()
            .map_err(|_| "Publication state unavailable")?;
        let _file = self.file_lock()?;
        Ok(self.read()?.repository)
    }
    pub fn entries(&self) -> Result<Vec<Publication>, String> {
        let _guard = self
            .mutex
            .lock()
            .map_err(|_| "Publication state unavailable")?;
        let _file = self.file_lock()?;
        Ok(self
            .read()?
            .entries
            .into_iter()
            .filter(|entry| entry.status != "draft")
            .collect())
    }
    pub fn prepare(
        &self,
        store: &SqliteStore,
        ids: &[String],
        title: &str,
        history: bool,
        evidence: bool,
    ) -> Result<Publication, String> {
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        if unique.len() != ids.len() || ids.is_empty() || ids.len() > 50 {
            return Err("Choose 1–50 distinct decisions.".into());
        }
        // Only explicit selected IDs are read, never a full-store export.
        let selected = ids
            .iter()
            .map(|id| {
                Engine::new(store.clone())
                    .history(id)
                    .map_err(|error| error.to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let snapshot = project(title, &selected, history, evidence)?;
        let entry = Publication {
            id: Uuid::new_v4().to_string(),
            snapshot,
            repository: None,
            url: None,
            commit: None,
            status: "draft".into(),
        };
        let _guard = self
            .mutex
            .lock()
            .map_err(|_| "Publication state unavailable")?;
        let _file = self.file_lock()?;
        let mut ledger = self.read()?;
        // Keep prepared drafts bounded; submitted/live copies are never silently removed.
        ledger.entries.retain(|entry| entry.status != "draft");
        ledger.entries.push(entry.clone());
        self.save(&ledger)?;
        Ok(entry)
    }
    pub fn get(&self, id: &str) -> Result<Publication, String> {
        let _guard = self
            .mutex
            .lock()
            .map_err(|_| "Publication state unavailable")?;
        let _file = self.file_lock()?;
        self.read()?
            .entries
            .into_iter()
            .find(|entry| entry.id == id)
            .ok_or("Preview has expired. Prepare it again.".into())
    }
    pub fn update(&self, entry: Publication) -> Result<(), String> {
        let _guard = self
            .mutex
            .lock()
            .map_err(|_| "Publication state unavailable")?;
        let _file = self.file_lock()?;
        let mut ledger = self.read()?;
        let old = ledger
            .entries
            .iter_mut()
            .find(|old| old.id == entry.id)
            .ok_or("Publication not found")?;
        if old.snapshot != entry.snapshot
            || old
                .repository
                .as_ref()
                .is_some_and(|repository| entry.repository.as_ref() != Some(repository))
            || old
                .url
                .as_ref()
                .is_some_and(|url| entry.url.as_ref() != Some(url))
        {
            return Err(
                "The public preview and its destination are frozen. Prepare a new snapshot.".into(),
            );
        }
        let allowed = matches!(
            (old.status.as_str(), entry.status.as_str()),
            ("draft", "pending")
                | ("pending", "pending" | "live" | "withdrawal_pending")
                | ("live", "live" | "withdrawal_pending")
                | ("withdrawal_pending", "withdrawal_pending" | "withdrawn")
                | ("withdrawn", "withdrawn")
        );
        if !allowed {
            return Err("Publication state changed. Refresh before continuing.".into());
        }
        *old = entry.clone();
        if let Some(repository) = entry.repository {
            ledger.repository = repository;
        }
        self.save(&ledger)
    }
}
