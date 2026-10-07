// Expose local adapters while leaving decision semantics in memory-engine.
pub mod paths;
pub mod storage;
pub use storage::SqliteStore;
