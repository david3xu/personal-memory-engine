// Translate worker tool calls into the engine's validated operations; stdout is protocol only.
use crate::SqliteStore;
use memory_engine::{CaptureDecision, DecisionVersion, Engine};
use rmcp::{
    Json, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerConfig},
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const WORKER_INSTRUCTIONS: &str = "While the owner has enabled decision recording, call record_decision when the user explicitly chooses or approves an option in normal conversation. No @ mention or separate save request is needed. Do not record questions, brainstorming, AI suggestions, or unaccepted proposals. Omit unstated reasons and evidence. For a changed choice, read the latest history and append a linked revision. Never scrape or monitor chats. Respect requests not to save. Confirm saving only after a successful tool result. user_confirmed is your attribution assertion, not independently verified consent. Use a unique request_id for each submission and reuse identical arguments on retries. For the clearly labeled synthetic setup choice only, use its Setup reference as request_id. Before revising, supply decision_id and supersedes_version_id from the latest stored version. Never invent source URLs, earlier records, reasons, relationships, or user statements. Nothing is published by these tools.";

#[derive(Serialize, JsonSchema)]
pub struct DecisionList {
    pub records: Vec<DecisionVersion>,
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct HistoryRequest {
    pub decision_id: String,
}

#[derive(Clone)]
pub struct MemoryMcp {
    store: SqliteStore,
    tool_router: ToolRouter<Self>,
}
#[tool_router]
impl MemoryMcp {
    pub fn new(store: SqliteStore) -> Self {
        Self {
            store,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Append one explicit user decision, or a linked revision of an existing decision. Preserve missing rationale and evidence. Never record an AI suggestion as the user's choice.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn record_decision(
        &self,
        Parameters(input): Parameters<CaptureDecision>,
    ) -> Result<Json<DecisionVersion>, String> {
        let store = self.store.clone();
        tokio::task::spawn_blocking(move || {
            let mut access = store.connection_control().worker_access()?;
            // Activity metadata must not make an already-committed decision appear to have failed.
            let _ = store.note_mcp_use();
            let record = Engine::new(store)
                .record(input)
                .map_err(|error| error.to_string())?;
            // A receipt failure must never turn a committed save into a reported recording failure.
            let _ = access.note_record(&record);
            Ok(Json(record))
        })
        .await
        .map_err(|_| "Local recording task could not complete".to_string())?
    }
    #[tool(
        description = "Read the latest versions of the most recent 200 local decisions. Use their IDs when revising a choice. This does not return the entire history.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn list_decisions(&self) -> Result<Json<DecisionList>, String> {
        let store = self.store.clone();
        tokio::task::spawn_blocking(move || {
            let _access = store.connection_control().worker_access()?;
            let _ = store.note_mcp_use();
            Engine::new(store)
                .list()
                .map(|records| Json(DecisionList { records }))
                .map_err(|error| error.to_string())
        })
        .await
        .map_err(|_| "Local reading task could not complete".to_string())?
    }
    #[tool(
        description = "Read every preserved version of one local decision, newest first. A revision must reference its latest version. Never overwrite an earlier choice.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn decision_history(
        &self,
        Parameters(input): Parameters<HistoryRequest>,
    ) -> Result<Json<DecisionList>, String> {
        let store = self.store.clone();
        tokio::task::spawn_blocking(move || {
            let _access = store.connection_control().worker_access()?;
            let _ = store.note_mcp_use();
            Engine::new(store)
                .history(&input.decision_id)
                .map(|records| Json(DecisionList { records }))
                .map_err(|error| error.to_string())
        })
        .await
        .map_err(|_| "Local history task could not complete".to_string())?
    }
}
#[tool_handler(router = self.tool_router)]
impl ServerHandler for MemoryMcp {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(WORKER_INSTRUCTIONS)
    }
}
