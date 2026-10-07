pub mod connection;
// Expose local adapters while leaving decision semantics in memory-engine.
pub mod mcp;
pub mod paths;
pub mod storage;
pub use storage::SqliteStore;
pub mod worker_package;
