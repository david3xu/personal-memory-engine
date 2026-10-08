// Project only explicitly selected presentation fields into a portable public snapshot.
pub mod render;
use memory_engine::{Alternative, DecisionVersion, Evidence};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub schema_version: u32,
    pub title: String,
    pub cards: Vec<PublicCard>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicCard {
    pub versions: Vec<PublicVersion>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicVersion {
    pub chosen_option: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
    pub alternatives: Vec<Alternative>,
    pub evidence: Vec<Evidence>,
    pub recorded_at: String,
}
pub fn project(
    title: &str,
    selected: &[Vec<DecisionVersion>],
    history: bool,
    evidence: bool,
) -> Result<Snapshot, String> {
    if title.trim().is_empty()
        || title.len() > 160
        || selected.is_empty()
        || selected.len() > 50
        || selected.iter().any(Vec::is_empty)
    {
        return Err("Choose 1–50 decisions and a title up to 160 bytes.".into());
    }
    let cards = selected
        .iter()
        .map(|versions| {
            let count = if history { versions.len() } else { 1 };
            PublicCard {
                versions: versions
                    .iter()
                    .take(count)
                    .rev()
                    .map(|version| PublicVersion {
                        chosen_option: version.submission.chosen_option.clone(),
                        rationale: version.submission.rationale.clone(),
                        alternatives: version.submission.alternatives.clone(),
                        evidence: if evidence {
                            version.submission.evidence.clone()
                        } else {
                            vec![]
                        },
                        recorded_at: version.recorded_at.clone(),
                    })
                    .collect(),
            }
        })
        .collect();
    let snapshot = Snapshot {
        schema_version: 1,
        title: title.trim().into(),
        cards,
    };
    if serde_json::to_vec(&snapshot)
        .map_err(|_| "Snapshot could not be prepared")?
        .len()
        > 500_000
    {
        return Err("This selection is too large. Share fewer decisions or versions.".into());
    }
    Ok(snapshot)
}
