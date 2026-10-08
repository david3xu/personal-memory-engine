// Render public snapshots as escaped, self-contained read-only pages without scripts or external assets.
use crate::{PublicVersion, Snapshot};
pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
fn version(version: &PublicVersion) -> String {
    let mut page = format!(
        "<h2>{}</h2><p class='date'>Recorded {}</p><h3>Stated rationale</h3><p>{}</p>",
        escape(&version.chosen_option),
        escape(&version.recorded_at),
        escape(version.rationale.as_deref().unwrap_or("Not stated"))
    );
    if !version.alternatives.is_empty() {
        page.push_str("<h3>Rejected alternatives</h3><ul>");
        for alt in &version.alternatives {
            page.push_str(&format!(
                "<li><strong>{}</strong><p>{}</p></li>",
                escape(&alt.option),
                escape(alt.reason.as_deref().unwrap_or("Reason not stated"))
            ));
        }
        page.push_str("</ul>");
    }
    if !version.evidence.is_empty() {
        page.push_str("<h3>Included source evidence</h3>");
        for item in &version.evidence {
            page.push_str(&format!("<p>{}</p>", escape(&item.content)));
            if let Some(reference) = &item.reference {
                page.push_str(&format!("<p class='date'>{}</p>", escape(reference)));
            }
        }
    }
    page
}
fn document(title: &str, marker: &str, body: &str) -> String {
    format!(
        "<!doctype html><html lang='en'><head><meta charset='utf-8'><meta name='viewport' content='width=device-width,initial-scale=1'><meta name='referrer' content='no-referrer'><meta http-equiv='Content-Security-Policy' content=\"default-src 'none'; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'\"><meta name='pme-snapshot' content='{}'><title>{}</title><style>body{{margin:0;background:#f5f7fa;color:#18293a;font:16px/1.65 system-ui,sans-serif}}main{{max-width:780px;margin:auto;padding:40px 20px}}header{{margin-bottom:28px}}h1{{font-size:32px;line-height:1.2}}h2{{font-size:23px;line-height:1.4}}h3{{font-size:14px;margin-bottom:6px}}article{{background:white;border:1px solid #dde4ed;border-radius:16px;padding:24px;margin:20px 0}}p{{white-space:pre-wrap;overflow-wrap:anywhere}}.date,footer{{color:#617182;font-size:13px}}summary{{cursor:pointer;padding:12px 0;font-weight:600}}details{{border-top:1px solid #dde4ed;margin-top:20px}}section{{padding-top:12px}}a{{color:inherit}}@media(max-width:500px){{main{{padding:24px 16px}}article{{padding:18px}}h1{{font-size:27px}}}}</style></head><body><main>{}<footer>Personal Memory Engine · Read-only public copy. The owner's private database and recording tools are not connected to this page.</footer></main></body></html>",
        escape(marker),
        escape(title),
        body
    )
}
pub fn html(snapshot: &Snapshot, publication_id: &str) -> String {
    let mut body = format!(
        "<header><p class='date'>OWNER-SELECTED SNAPSHOT</p><h1>{}</h1><p>Only the content selected for this publication is shown. Local changes do not update this copy.</p></header>",
        escape(&snapshot.title)
    );
    for card in &snapshot.cards {
        if let Some(current) = card.versions.last() {
            body.push_str("<article>");
            body.push_str(&version(current));
            if card.versions.len() > 1 {
                body.push_str(&format!(
                    "<details><summary>Earlier versions ({})</summary>",
                    card.versions.len() - 1
                ));
                for earlier in &card.versions[..card.versions.len() - 1] {
                    body.push_str("<section>");
                    body.push_str(&version(earlier));
                    body.push_str("</section>");
                }
                body.push_str("</details>");
            }
            body.push_str("</article>");
        }
    }
    document(&snapshot.title, publication_id, &body)
}
pub fn withdrawn(publication_id: &str) -> String {
    document(
        "Snapshot withdrawn",
        &format!("withdrawn-{publication_id}"),
        "<header><h1>Snapshot withdrawn</h1><p>The owner has withdrawn this live copy. Previously downloaded copies or repository history may still exist.</p></header>",
    )
}
pub fn landing() -> String {
    document(
        "Shared decision snapshots",
        "personal-memory-engine-public-root",
        "<header><h1>Shared decision snapshots</h1><p>Open the specific snapshot link provided by its owner.</p></header>",
    )
}
