// Render isolated scripted demo records through the same positive projection used by owner sharing.
use memory_engine::DecisionVersion;
use memory_public_snapshot::{project, render};
use std::{env, fs, path::PathBuf};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() != 2 {
        return Err("Usage: render_demo <isolated-records.json> <output-directory>".into());
    }
    let records: Vec<DecisionVersion> = serde_json::from_slice(&fs::read(&args[0])?)?;
    if records.len() != 2
        || records
            .iter()
            .any(|record| record.submission.worker != "Synthetic showcase protocol client")
    {
        return Err("Only the two isolated synthetic showcase versions are accepted".into());
    }
    let snapshot = project(
        "Synthetic travel decision — preserved history",
        &[records],
        true,
        true,
    )?;
    let output = PathBuf::from(&args[1]);
    fs::create_dir_all(&output)?;
    let html = render::html(&snapshot, "synthetic-revision-demo").replace(
        "<main>",
        "<main><p><a href='../'>← Personal Memory Engine</a></p><aside><strong>SYNTHETIC DEMONSTRATION</strong><p>A scripted client recorded and revised a fictional choice through the real MCP helper in isolated storage. This is not personal memory or a live AI transcript. Both history and supplied synthetic evidence are intentionally included.</p></aside>",
    );
    fs::write(output.join("index.html"), html)?;
    fs::write(
        output.join("snapshot.json"),
        serde_json::to_vec_pretty(&snapshot)?,
    )?;
    Ok(())
}
