// Enforce explicit choices and input bounds before delegating to a repository.
use crate::{CaptureDecision, DecisionVersion};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Invalid submission: {0}")]
    InvalidInput(String),
    #[error("The referenced decision or version does not exist")]
    NotFound,
    #[error("{0}")]
    Conflict(String),
    #[error("Local storage is unavailable; no successful save is confirmed")]
    Storage,
}

#[derive(Debug, Clone)]
pub struct ValidatedCapture(CaptureDecision);
impl ValidatedCapture {
    pub fn submission(&self) -> &CaptureDecision {
        &self.0
    }
}

fn text(value: &str, name: &str, limit: usize) -> Result<(), EngineError> {
    if value.trim().is_empty() || value.len() > limit {
        return Err(EngineError::InvalidInput(format!(
            "{name} must contain 1–{limit} bytes of text"
        )));
    }
    Ok(())
}
fn optional(value: &Option<String>, name: &str, limit: usize) -> Result<(), EngineError> {
    if let Some(value) = value {
        text(value, name, limit)?;
    }
    Ok(())
}
impl CaptureDecision {
    pub fn validate(self) -> Result<ValidatedCapture, EngineError> {
        if !self.user_confirmed {
            return Err(EngineError::InvalidInput(
                "An explicit user choice is required".into(),
            ));
        }
        text(&self.request_id, "request_id", 128)?;
        text(&self.chosen_option, "chosen_option", 4096)?;
        text(&self.worker, "worker", 128)?;
        optional(&self.rationale, "rationale", 32768)?;
        optional(&self.user_statement, "user_statement", 32768)?;
        optional(&self.decision_id, "decision_id", 128)?;
        optional(&self.supersedes_version_id, "supersedes_version_id", 128)?;
        if self.decision_id.is_some() != self.supersedes_version_id.is_some() {
            return Err(EngineError::InvalidInput(
                "Supply both revision references, or neither".into(),
            ));
        }
        if self.alternatives.len() > 50 || self.evidence.len() > 50 {
            return Err(EngineError::InvalidInput(
                "At most 50 alternatives and 50 evidence items are allowed".into(),
            ));
        }
        for alt in &self.alternatives {
            text(&alt.option, "alternative", 4096)?;
            optional(&alt.reason, "rejection reason", 32768)?;
        }
        for evidence in &self.evidence {
            text(&evidence.content, "evidence", 32768)?;
            optional(&evidence.reference, "reference", 4096)?;
        }
        Ok(ValidatedCapture(self))
    }
}

pub trait DecisionRepository: Send + Sync {
    fn append(&self, input: ValidatedCapture) -> Result<DecisionVersion, EngineError>;
    fn list(&self) -> Result<Vec<DecisionVersion>, EngineError>;
    fn history(&self, decision_id: &str) -> Result<Vec<DecisionVersion>, EngineError>;
}

pub struct Engine<R: DecisionRepository> {
    repository: R,
}
impl<R: DecisionRepository> Engine<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn record(&self, input: CaptureDecision) -> Result<DecisionVersion, EngineError> {
        self.repository.append(input.validate()?)
    }
    pub fn list(&self) -> Result<Vec<DecisionVersion>, EngineError> {
        self.repository.list()
    }
    pub fn history(&self, id: &str) -> Result<Vec<DecisionVersion>, EngineError> {
        text(id, "decision_id", 128)?;
        self.repository.history(id)
    }
}
