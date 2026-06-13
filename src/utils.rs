use comfy_table::presets::UTF8_HORIZONTAL_ONLY;
use comfy_table::{ContentArrangement, Table};
use heck::ToTitleCase;

/// Strip a namespace prefix, e.g. `minecraft:oak_planks` -> `oak_planks`.
pub fn strip_namespace(id: &str) -> &str {
    id.rsplit(':').next().unwrap_or(id)
}

pub fn humanize(id: &str) -> String {
    id.to_title_case()
}

pub fn new_table() -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_HORIZONTAL_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic);
    table
}

fn download(url: &str) -> Result<String, String> {
    let mut resp = ureq::get(url)
        .call()
        .map_err(|e| format!("request failed: {e}"))?;
    if resp.status() != 200 {
        return Err(format!(
            "{url} returned HTTP {} (is the version valid?)",
            resp.status()
        ));
    }
    resp.body_mut()
        .read_to_string()
        .map_err(|e| format!("failed to read response: {e}"))
}

/// Returns contents of a cached file namespaced by mc version,
/// or downloads it from `url` and caches at `$CACHE_DIR/draftbud`.
pub fn fetch_cached(version: &str, file: &str, url: &str) -> Result<String, String> {
    let path = dirs::cache_dir().map(|d| d.join("draftbud").join(version).join(file));

    if let Some(p) = &path
        && let Ok(contents) = std::fs::read_to_string(p)
    {
        return Ok(contents);
    }

    eprintln!("fetching {file} for {version}...");
    let body = download(url)?;

    if let Some(p) = &path {
        if let Some(parent) = p.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(p, &body);
    }

    Ok(body)
}
