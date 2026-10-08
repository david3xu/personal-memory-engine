// Publish immutable, already-approved static snapshots into the owner's isolated GitHub Pages branch.
mod client;
pub mod login;
use base64::Engine as _;
pub use client::GitHub;
use memory_public_snapshot::render;
use serde::Serialize;
use serde_json::json;
use std::{process::Stdio, time::Duration};
const BRANCH: &str = "pme-public";
#[derive(Debug, Clone, Serialize)]
pub struct Submitted {
    pub url: String,
    pub commit: String,
}
fn target(value: &str) -> Result<(String, String), String> {
    let value = value
        .trim()
        .trim_end_matches('/')
        .strip_prefix("https://github.com/")
        .unwrap_or(value.trim().trim_end_matches('/'));
    let parts: Vec<_> = value.split('/').collect();
    if parts.len() != 2
        || parts.iter().any(|s| {
            s.is_empty()
                || s.len() > 100
                || !s
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
        })
        || parts.iter().any(|s| *s == "." || *s == "..")
    {
        return Err("Paste a GitHub repository URL or owner/repository.".into());
    }
    Ok((parts[0].into(), parts[1].into()))
}
fn id_valid(id: &str) -> bool {
    id.len() == 36 && id.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
}
pub fn public_url(repository: &str, id: &str) -> Result<String, String> {
    let (owner, repo) = target(repository)?;
    if !id_valid(id) {
        return Err("Unsupported publication ID".into());
    }
    let base = if repo.eq_ignore_ascii_case(&format!("{owner}.github.io")) {
        format!("https://{}.github.io/", owner.to_lowercase())
    } else {
        format!("https://{}.github.io/{repo}/", owner.to_lowercase())
    };
    Ok(format!("{base}shares/{id}/"))
}
impl GitHub {
    pub async fn publish(
        &self,
        repository: &str,
        id: &str,
        html: &str,
    ) -> Result<Submitted, String> {
        if !id_valid(id) || html.len() > 900_000 {
            return Err("The prepared snapshot is unsupported.".into());
        }
        let (owner, repo) = target(repository)?;
        let account = self.account().await?;
        let base = format!("repos/{owner}/{repo}");
        let details = self.api(&base, "GET", None).await?;
        if !account.eq_ignore_ascii_case(&owner)
            || details["private"] != false
            || details["permissions"]["admin"] != true
            || details["archived"] != false
        {
            return Err("Choose a public repository owned by the signed-in account with administrator access.".into());
        }
        if details["name"] != repo {
            return Err("Use the repository name exactly as displayed on GitHub. Prepare a new preview if the destination needs correction.".into());
        }
        let expected_base = if repo.eq_ignore_ascii_case(&format!("{owner}.github.io")) {
            format!("https://{}.github.io/", owner.to_lowercase())
        } else {
            format!("https://{}.github.io/{repo}/", owner.to_lowercase())
        };
        let pages = self.api(&format!("{base}/pages"), "GET", None).await;
        match &pages {
            Ok(site) if site["source"]["branch"]==BRANCH && site["source"]["path"]=="/" && site["html_url"]==expected_base && site["build_type"]=="legacy" => {},
            Ok(_) => return Err("This repository already hosts a different Pages site. Choose a dedicated sharing repository; it has not been changed.".into()),
            Err(error) if error=="not_found" => {},
            Err(error) => return Err(error.clone()),
        }
        let head = self
            .api(&format!("{base}/git/ref/heads/{BRANCH}"), "GET", None)
            .await;
        let mut parents = vec![];
        let mut tree = json!({"tree":[]});
        match head {
            Ok(head) => {
                let sha = head["object"]["sha"]
                    .as_str()
                    .ok_or("Publishing branch is invalid")?;
                let commit = self
                    .api(&format!("{base}/git/commits/{sha}"), "GET", None)
                    .await?;
                // The root must carry our marker before updating an existing branch.
                let root = self
                    .api(
                        &format!("{base}/contents/index.html?ref={BRANCH}"),
                        "GET",
                        None,
                    )
                    .await?;
                let content = root["content"]
                    .as_str()
                    .unwrap_or_default()
                    .replace(['\n', '\r'], "");
                let owned_root = root["encoding"] == "base64"
                    && base64::engine::general_purpose::STANDARD
                        .decode(content)
                        .is_ok_and(|bytes| bytes == render::landing().as_bytes());
                if !owned_root
                    || root["sha"].as_str().is_none()
                    || commit["message"]
                        .as_str()
                        .is_none_or(|message| !message.starts_with("Personal Memory Engine:"))
                {
                    return Err(
                        "The sharing branch belongs to another workflow. It has not been changed."
                            .into(),
                    );
                }
                parents.push(sha.to_string());
                tree["base_tree"] = commit["tree"]["sha"].clone();
            }
            Err(error) if error == "not_found" => {}
            Err(error) => return Err(error),
        }
        let mut files = vec![
            json!({"path":format!("shares/{id}/index.html"),"mode":"100644","type":"blob","content":html}),
        ];
        if parents.is_empty() {
            files.push(json!({"path":"index.html","mode":"100644","type":"blob","content":render::landing()}));
            files.push(json!({"path":".nojekyll","mode":"100644","type":"blob","content":""}));
        }
        tree["tree"] = json!(files);
        let created = self
            .api(&format!("{base}/git/trees"), "POST", Some(tree))
            .await?;
        let commit=self.api(&format!("{base}/git/commits"),"POST",Some(json!({"message":"Personal Memory Engine: publish selected snapshot","tree":created["sha"],"parents":parents}))).await?;
        let sha = commit["sha"]
            .as_str()
            .ok_or("GitHub did not return a publication commit")?
            .to_string();
        if parents.is_empty() {
            self.api(
                &format!("{base}/git/refs"),
                "POST",
                Some(json!({"ref":format!("refs/heads/{BRANCH}"),"sha":sha})),
            )
            .await?;
        } else {
            self.api(
                &format!("{base}/git/refs/heads/{BRANCH}"),
                "PATCH",
                Some(json!({"sha":sha,"force":false})),
            )
            .await?;
        }
        if pages.is_err() {
            self.api(
                &format!("{base}/pages"),
                "POST",
                Some(json!({"source":{"branch":BRANCH,"path":"/"},"build_type":"legacy"})),
            )
            .await?;
        }
        Ok(Submitted {
            url: format!("{expected_base}shares/{id}/"),
            commit: sha,
        })
    }
}
pub async fn verify_live(url: &str, expected: &str) -> Result<bool, String> {
    let parsed = url::Url::parse(url).map_err(|_| "The publication URL is invalid")?;
    if parsed.scheme() != "https"
        || parsed
            .host_str()
            .is_none_or(|host| !host.ends_with(".github.io"))
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.port().is_some()
    {
        return Err("Only the verified GitHub Pages host can be checked.".into());
    }
    let output = tokio::time::timeout(
        Duration::from_secs(15),
        tokio::process::Command::new("/usr/bin/curl")
            .args([
                "--fail",
                "--silent",
                "--max-time",
                "10",
                "--max-redirs",
                "0",
                url,
            ])
            .stdin(Stdio::null())
            .kill_on_drop(true)
            .output(),
    )
    .await
    .map_err(|_| "Public page verification timed out")?
    .map_err(|_| "Public page verification is unavailable")?;
    Ok(output.status.success() && output.stdout == expected.as_bytes())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repository_targets_cannot_be_urls_or_path_traversals() {
        for bad in [
            "../../secret",
            "https://other.example/u/r",
            "owner/repo?x=1",
            "o/r/extra",
            "o/..",
        ] {
            assert!(target(bad).is_err());
        }
        assert_eq!(
            target("https://github.com/owner/shares/").unwrap(),
            ("owner".into(), "shares".into())
        );
    }
}
