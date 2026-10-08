// Standalone stdio entry point; workers start this packaged helper for their session.
use clap::Parser;
use memory_local_runtime::{SqliteStore, mcp::MemoryMcp, paths::default_data_dir};
use rmcp::ServiceExt;
use std::path::PathBuf;
#[derive(Parser)]
#[command(version, about = "Local-first decision memory MCP server (stdio)")]
struct Arguments {
    /// Application data directory. Must match the owner's desktop app.
    #[arg(long)]
    data_dir: Option<PathBuf>,
}
#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("Personal Memory Engine could not start: {error}");
        std::process::exit(1);
    }
}
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();
    let data_dir = args.data_dir.map(Ok).unwrap_or_else(default_data_dir)?;
    let store = SqliteStore::open(&data_dir.join("memory.sqlite3"))?;
    MemoryMcp::new(store)
        .serve(rmcp::transport::stdio())
        .await?
        .waiting()
        .await?;
    Ok(())
}
