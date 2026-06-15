use std::path::PathBuf;

use comfy_table::presets::UTF8_HORIZONTAL_ONLY;
use comfy_table::{Cell, ContentArrangement, Table};
use heck::ToTitleCase;
use serde::Deserialize;
use serde::de::DeserializeOwned;

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

/// ANSI foreground codes (cyan, yellow, green, magenta, blue).
const LIST_COLORS: [u8; 5] = [36, 33, 32, 35, 34];

/// Render a list in a single cell: one item per line w/ unique colour.
pub fn colored_list<S: AsRef<str>>(items: &[S]) -> String {
    if items.is_empty() {
        return "-".to_string();
    }
    items
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let code = LIST_COLORS[i % LIST_COLORS.len()];
            format!("\x1b[{code}m{}\x1b[0m", s.as_ref())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Read the file at `path`, or create it and return that.
/// `None` path (no config/cache dir found) just runs `seed()` without persisting.
fn read_or_create(
    path: Option<PathBuf>,
    seed: impl FnOnce() -> Result<String, String>,
) -> Result<String, String> {
    if let Some(p) = &path
        && let Ok(contents) = std::fs::read_to_string(p)
    {
        return Ok(contents);
    }

    let body = seed()?;
    if let Some(p) = &path {
        if let Some(parent) = p.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(p, &body);
    }
    Ok(body)
}

/// Load and deserialise a persistent mutable JSON config.
pub fn load_config<T: DeserializeOwned>(file: &str, default: &str) -> Result<T, String> {
    let path = dirs::config_dir().map(|d| d.join("draftbud").join(file));
    let json = read_or_create(path, || Ok(default.to_string()))?;
    serde_json::from_str(&json).map_err(|e| format!("failed to parse {file}: {e}"))
}

/// Print a titled table from a header and body rows.
pub fn print_listing(title: &str, headers: &[&str], rows: impl IntoIterator<Item = Vec<Cell>>) {
    let mut table = new_table();
    table.set_header(headers.to_vec());
    for row in rows {
        table.add_row(row);
    }
    println!("{title}\n");
    println!("{table}");
}

/// A label and the list of things under it, e.g. an effect and its methods.
/// `name`/`methods` are the canonical keys with aliases for `category`/`goals`
#[derive(Deserialize)]
struct NamedList {
    #[serde(alias = "category")]
    name: String,
    #[serde(alias = "goals", default)]
    methods: Vec<String>,
}

/// Shared logic to print a simple table from a JSON config.
pub fn print_named_lists(file: &str, default: &str, title: &str, headers: [&str; 2]) {
    let entries: Vec<NamedList> = match load_config(file, default) {
        Ok(entries) => entries,
        Err(e) => return eprintln!("error: {e}"),
    };
    let rows = entries
        .iter()
        .map(|e| vec![Cell::new(&e.name), Cell::new(colored_list(&e.methods))]);
    print_listing(title, &headers, rows);
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

/// Fetch mcmeta data and cache at `$CACHE_DIR/draftbud/<version>/`.
pub fn fetch_cached(version: &str, file: &str, url: &str) -> Result<String, String> {
    let path = dirs::cache_dir().map(|d| d.join("draftbud").join(version).join(file));
    read_or_create(path, || {
        eprintln!("fetching {file} for {version}...");
        download(url)
    })
}
