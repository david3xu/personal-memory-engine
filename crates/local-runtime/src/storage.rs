// Persist validated append-only versions atomically through the engine repository port.
use chrono::Utc;
use memory_engine::{DecisionRepository, DecisionVersion, EngineError, ValidatedCapture};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use std::{
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteStore {
    connection: Arc<Mutex<Connection>>,
    control: crate::connection::ConnectionControl,
}
impl SqliteStore {
    pub fn open(path: &Path) -> Result<Self, EngineError> {
        if let Some(parent) = path.parent() {
            let mut builder = std::fs::DirBuilder::new();
            builder.recursive(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                builder.mode(0o700);
            }
            builder.create(parent).map_err(|_| EngineError::Storage)?;
        }
        let connection = Connection::open(path).map_err(|_| EngineError::Storage)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
                .map_err(|_| EngineError::Storage)?;
        }
        connection
            .busy_timeout(Duration::from_secs(5))
            .map_err(|_| EngineError::Storage)?;
        let version: u32 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(|_| EngineError::Storage)?;
        if version > 1 {
            return Err(EngineError::Conflict(
                "This database requires a newer application; it was not migrated".into(),
            ));
        }
        connection.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;
        CREATE TABLE IF NOT EXISTS decision_versions (
            sequence INTEGER PRIMARY KEY AUTOINCREMENT,
            version_id TEXT NOT NULL UNIQUE,
            decision_id TEXT NOT NULL,
            previous_version_id TEXT UNIQUE REFERENCES decision_versions(version_id),
            request_id TEXT NOT NULL UNIQUE,
            submission_json TEXT NOT NULL,
            record_json TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS versions_by_decision ON decision_versions(decision_id, sequence);
        CREATE TRIGGER IF NOT EXISTS preserve_version_updates BEFORE UPDATE ON decision_versions
            BEGIN SELECT RAISE(ABORT, 'Decision versions cannot be overwritten'); END;
        CREATE TRIGGER IF NOT EXISTS preserve_version_deletes BEFORE DELETE ON decision_versions
            BEGIN SELECT RAISE(ABORT, 'Decision versions cannot be deleted individually'); END;
        CREATE TABLE IF NOT EXISTS mcp_activity (singleton INTEGER PRIMARY KEY CHECK (singleton=1), last_used_at TEXT NOT NULL);
        PRAGMA user_version=1;").map_err(|_| EngineError::Storage)?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            control: crate::connection::ConnectionControl::for_database(path),
        })
    }
    pub fn connection_control(&self) -> &crate::connection::ConnectionControl {
        &self.control
    }
    pub fn note_mcp_use(&self) -> Result<(), EngineError> {
        self.connection.lock().map_err(|_|EngineError::Storage)?.execute("INSERT INTO mcp_activity VALUES (1,?1) ON CONFLICT(singleton) DO UPDATE SET last_used_at=excluded.last_used_at",[Utc::now().to_rfc3339()]).map_err(|_|EngineError::Storage)?;
        Ok(())
    }
    pub fn last_mcp_use(&self) -> Result<Option<String>, EngineError> {
        self.connection
            .lock()
            .map_err(|_| EngineError::Storage)?
            .query_row(
                "SELECT last_used_at FROM mcp_activity WHERE singleton=1",
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(|_| EngineError::Storage)
    }
}
fn parse(json: String) -> Result<DecisionVersion, EngineError> {
    let record: DecisionVersion = serde_json::from_str(&json).map_err(|_| EngineError::Storage)?;
    if record.schema_version != 1 || record.submission.clone().validate().is_err() {
        return Err(EngineError::Storage);
    }
    Ok(record)
}
impl DecisionRepository for SqliteStore {
    fn append(&self, input: ValidatedCapture) -> Result<DecisionVersion, EngineError> {
        let input = input.submission();
        let submission_json = serde_json::to_string(input).map_err(|_| EngineError::Storage)?;
        let mut connection = self.connection.lock().map_err(|_| EngineError::Storage)?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| EngineError::Storage)?;
        let previous: Option<(String, String)> = transaction
            .query_row(
                "SELECT submission_json, record_json FROM decision_versions WHERE request_id=?1",
                [&input.request_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(|_| EngineError::Storage)?;
        if let Some((existing, record)) = previous {
            if existing != submission_json {
                return Err(EngineError::Conflict(
                    "This request_id was already used for different content; use a new key".into(),
                ));
            }
            return parse(record);
        }
        let decision_id = match (&input.decision_id, &input.supersedes_version_id) {
            (Some(decision_id), Some(version_id)) => {
                let owner: Option<String> = transaction
                    .query_row(
                        "SELECT decision_id FROM decision_versions WHERE version_id=?1",
                        [version_id],
                        |row| row.get(0),
                    )
                    .optional()
                    .map_err(|_| EngineError::Storage)?;
                match owner {
                    None => return Err(EngineError::NotFound),
                    Some(owner) if owner != *decision_id => {
                        return Err(EngineError::InvalidInput(
                            "The prior version belongs to another decision".into(),
                        ));
                    }
                    _ => {}
                }
                let latest: String = transaction.query_row("SELECT version_id FROM decision_versions WHERE decision_id=?1 ORDER BY sequence DESC LIMIT 1", [decision_id], |row|row.get(0)).map_err(|_|EngineError::Storage)?;
                if latest != *version_id {
                    return Err(EngineError::Conflict("The referenced version is no longer current; retrieve history before revising".into()));
                }
                decision_id.clone()
            }
            (None, None) => Uuid::new_v4().to_string(),
            _ => {
                return Err(EngineError::InvalidInput(
                    "Supply both revision references, or neither".into(),
                ));
            }
        };
        let record = DecisionVersion {
            schema_version: 1,
            decision_id,
            version_id: Uuid::new_v4().to_string(),
            recorded_at: Utc::now().to_rfc3339(),
            submission: input.clone(),
        };
        let record_json = serde_json::to_string(&record).map_err(|_| EngineError::Storage)?;
        transaction.execute("INSERT INTO decision_versions(version_id,decision_id,previous_version_id,request_id,submission_json,record_json) VALUES (?1,?2,?3,?4,?5,?6)", params![record.version_id,record.decision_id,input.supersedes_version_id,input.request_id,submission_json,record_json]).map_err(|_|EngineError::Storage)?;
        transaction.commit().map_err(|_| EngineError::Storage)?;
        Ok(record)
    }
    fn list(&self) -> Result<Vec<DecisionVersion>, EngineError> {
        let connection = self.connection.lock().map_err(|_| EngineError::Storage)?;
        let mut query=connection.prepare("SELECT record_json FROM decision_versions v WHERE NOT EXISTS (SELECT 1 FROM decision_versions child WHERE child.previous_version_id=v.version_id) ORDER BY sequence DESC LIMIT 200").map_err(|_|EngineError::Storage)?;
        let rows = query
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|_| EngineError::Storage)?;
        rows.map(|row| parse(row.map_err(|_| EngineError::Storage)?))
            .collect()
    }
    fn history(&self, id: &str) -> Result<Vec<DecisionVersion>, EngineError> {
        let connection = self.connection.lock().map_err(|_| EngineError::Storage)?;
        let mut query=connection.prepare("SELECT record_json FROM decision_versions WHERE decision_id=?1 ORDER BY sequence DESC").map_err(|_|EngineError::Storage)?;
        let rows = query
            .query_map([id], |row| row.get::<_, String>(0))
            .map_err(|_| EngineError::Storage)?;
        let records: Vec<DecisionVersion> = rows
            .map(|row| parse(row.map_err(|_| EngineError::Storage)?))
            .collect::<Result<_, _>>()?;
        if records.is_empty() {
            return Err(EngineError::NotFound);
        }
        Ok(records)
    }
}
