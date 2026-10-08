// Verify attribution, absent information, revision references, and record limits.
use memory_engine::CaptureDecision;
fn input() -> CaptureDecision {
    serde_json::from_value(serde_json::json!({"request_id":"test-1","user_confirmed":true,"chosen_option":"Rust","worker":"Synthetic test"})).unwrap()
}
#[test]
fn suggestions_are_rejected() {
    let mut request = input();
    request.user_confirmed = false;
    assert!(request.validate().is_err());
}
#[test]
fn missing_reasons_stay_absent() {
    let request = input();
    let value = serde_json::to_value(request.validate().unwrap().submission()).unwrap();
    assert!(value.get("rationale").is_none());
    assert!(value.get("user_statement").is_none());
    assert_eq!(value["evidence"], serde_json::json!([]));
}
#[test]
fn a_revision_requires_both_references() {
    let mut request = input();
    request.decision_id = Some("decision-1".into());
    assert!(request.validate().is_err());
}
#[test]
fn unsupported_fields_are_rejected() {
    let mut value = serde_json::to_value(input()).unwrap();
    value["invented_field"] = serde_json::json!(true);
    assert!(serde_json::from_value::<CaptureDecision>(value).is_err());
}
#[test]
fn blank_and_oversized_choices_are_rejected() {
    for choice in [" ".into(), "x".repeat(4097)] {
        let mut request = input();
        request.chosen_option = choice;
        assert!(request.validate().is_err());
    }
}
