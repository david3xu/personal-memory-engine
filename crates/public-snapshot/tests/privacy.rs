// Verify public projection omissions, revision scope, missing information and hostile text escaping.
use memory_engine::{CaptureDecision, DecisionVersion};
use memory_public_snapshot::{project, render};
fn record(option: &str) -> DecisionVersion {
    DecisionVersion { schema_version:1, decision_id:"PRIVATE-ID".into(),version_id:"PRIVATE-VERSION".into(),recorded_at:"2026-10-08T00:00:00Z".into(),submission:serde_json::from_value::<CaptureDecision>(serde_json::json!({"request_id":"PRIVATE-REQUEST","user_confirmed":true,"chosen_option":option,"worker":"PRIVATE-WORKER","user_statement":"PRIVATE-CHAT-QUOTE","evidence":[{"content":"PRIVATE-EVIDENCE","reference":"file:///private/source"}]})).unwrap() }
}
#[test]
fn selected_projection_has_no_private_transport_fields_or_unselected_history() {
    let histories = vec![vec![record("New choice"), record("PRIVATE-OLDER-CHOICE")]];
    let snapshot = project("Shared choices", &histories, false, false).unwrap();
    let encoded = serde_json::to_string(&snapshot).unwrap();
    assert!(!encoded.contains("PRIVATE-"));
    assert!(!encoded.contains("user_confirmed"));
    assert!(!encoded.contains("rationale"));
    assert_eq!(snapshot.cards[0].versions.len(), 1);
    let selected = project("Shared choices", &histories, true, true).unwrap();
    assert_eq!(
        selected.cards[0].versions[0].chosen_option,
        "PRIVATE-OLDER-CHOICE"
    );
    assert_eq!(selected.cards[0].versions[1].chosen_option, "New choice");
    assert_eq!(selected.cards[0].versions[1].evidence.len(), 1);
}
#[test]
fn text_cannot_create_scripts_links_or_network_requests() {
    let snapshot = project(
        "<script>alert(1)</script>",
        &[vec![record(
            "<img src=https://tracking.example onerror=alert(1)>",
        )]],
        false,
        false,
    )
    .unwrap();
    let page = render::html(&snapshot, "publication-id");
    assert!(!page.contains("<script>"));
    assert!(!page.contains("<img "));
    assert!(page.contains("&lt;img"));
    assert!(page.contains("default-src 'none'"));
    assert!(page.contains("Not stated"));
    assert!(!render::withdrawn("publication-id").contains("tracking.example"));
}
