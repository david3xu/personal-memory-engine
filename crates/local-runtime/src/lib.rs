// Expose local adapters while leaving decision semantics in memory-engine.
pub mod connection;
pub mod host_connection;
mod host_discovery;
pub mod mcp;
pub mod paths;
pub mod storage;
pub use storage::SqliteStore;
pub mod worker_package;

pub mod sharing;
