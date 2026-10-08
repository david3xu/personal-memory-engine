// Exercise GitHub API boundaries with a fake connector; no credentials or live network are used.
#![cfg(unix)]
use memory_github_publication::GitHub;
use std::os::unix::fs::PermissionsExt;
fn fixture(mode: &str) -> (tempfile::TempDir, GitHub) {
    let dir = tempfile::tempdir().unwrap();
    let cli = dir.path().join("connector");
    let details = if mode == "private" {
        r#"{"name":"shares","private":true,"archived":false,"permissions":{"admin":true}}"#
    } else {
        r#"{"name":"shares","private":false,"archived":false,"permissions":{"admin":true}}"#
    };
    let pages = if mode == "other-site" {
        r#"printf '%s' '{"source":{"branch":"main","path":"/"},"html_url":"https://owner.github.io/shares/","build_type":"legacy"}'"#
    } else {
        r#"echo 'gh: Not Found (HTTP 404)' >&2; exit 1"#
    };
    let ref_result = if mode == "conflict" {
        "echo 'gh: conflict (HTTP 422)' >&2;exit 1"
    } else {
        "printf '%s' '{}'"
    };
    let script = format!(
        r#"#!/bin/sh
printf '%s %s\n' "$5" "$6" >> '{}/calls'
case "$6" in
user) printf '%s' '{{"login":"owner"}}' ;;
repos/owner/shares) printf '%s' '{}' ;;
repos/owner/shares/pages) if [ "$5" = GET ]; then {}; else printf '%s' '{{}}'; fi ;;
repos/owner/shares/git/ref/heads/pme-public) echo 'gh: Not Found (HTTP 404)' >&2;exit 1 ;;
repos/owner/shares/git/trees) cat > '{}/tree.json'; printf '%s' '{{"sha":"tree"}}' ;;
repos/owner/shares/git/commits) cat > '{}/commit.json'; printf '%s' '{{"sha":"commit"}}' ;;
repos/owner/shares/git/refs) cat > '{}/ref.json'; {} ;;
*) echo 'Unexpected endpoint' >&2;exit 2 ;;
esac
"#,
        dir.path().display(),
        details,
        pages,
        dir.path().display(),
        dir.path().display(),
        dir.path().display(),
        ref_result
    );
    std::fs::write(&cli, script).unwrap();
    std::fs::set_permissions(&cli, std::fs::Permissions::from_mode(0o700)).unwrap();
    let client = GitHub::new(cli);
    (dir, client)
}
#[tokio::test]
async fn privacy_and_existing_site_checks_happen_before_any_write() {
    for mode in ["private", "other-site"] {
        let (dir, client) = fixture(mode);
        assert!(
            client
                .publish(
                    "owner/shares",
                    "00000000-0000-4000-8000-000000000001",
                    "selected static content"
                )
                .await
                .is_err()
        );
        let calls = std::fs::read_to_string(dir.path().join("calls")).unwrap();
        assert!(!calls.contains("POST"));
        assert!(!calls.contains("PATCH"));
    }
}
#[tokio::test]
async fn publication_is_an_orphan_tree_of_only_approved_static_files() {
    let (dir, client) = fixture("ok");
    let result = client
        .publish(
            "owner/shares",
            "00000000-0000-4000-8000-000000000001",
            "selected static content",
        )
        .await
        .unwrap();
    assert_eq!(
        result.url,
        "https://owner.github.io/shares/shares/00000000-0000-4000-8000-000000000001/"
    );
    let tree: serde_json::Value =
        serde_json::from_slice(&std::fs::read(dir.path().join("tree.json")).unwrap()).unwrap();
    assert!(tree.get("base_tree").is_none());
    assert_eq!(tree["tree"].as_array().unwrap().len(), 3);
    let commit: serde_json::Value =
        serde_json::from_slice(&std::fs::read(dir.path().join("commit.json")).unwrap()).unwrap();
    assert!(commit["parents"].as_array().unwrap().is_empty());
    assert!(
        !std::fs::read_to_string(dir.path().join("calls"))
            .unwrap()
            .contains("heads/main")
    );
    let (_dir, conflict) = fixture("conflict");
    assert!(
        conflict
            .publish(
                "owner/shares",
                "00000000-0000-4000-8000-000000000001",
                "selected static content"
            )
            .await
            .is_err()
    );
}
