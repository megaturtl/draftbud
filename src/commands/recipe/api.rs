use std::collections::HashMap;

use serde::Deserialize;

use crate::utils;

use super::merge;
use super::types::{Ingredient, ItemId, Layout, Recipe, RecipesByOutput};

const DEFAULT_MC_VERSION: &str = "26.1";
const API_BASE_URL: &str = "https://raw.githubusercontent.com/misode/mcmeta";

/// The Minecraft version to pull data for (override with `DRAFTBUD_MC_VERSION` env variable).
fn mc_version() -> String {
    std::env::var("DRAFTBUD_MC_VERSION").unwrap_or_else(|_| DEFAULT_MC_VERSION.into())
}

#[derive(Deserialize)]
struct ApiResult {
    id: String,
    count: Option<u32>,
}

#[derive(Deserialize)]
struct ApiRecipe {
    /// e.g. `minecraft:crafting_shaped`, `minecraft:smelting`. The JSON key is
    /// `type`, which is a Rust keyword so needs to be renamed.
    #[serde(rename = "type", default)]
    method: String,
    #[serde(default)]
    key: Option<HashMap<String, String>>,
    #[serde(default)]
    pattern: Option<Vec<String>>,
    #[serde(default)]
    ingredients: Option<Vec<String>>,
    /// singular ingredient used by smelting/stonecutting
    #[serde(default)]
    ingredient: Option<String>,
    result: Option<ApiResult>,
}

#[derive(Deserialize)]
struct ApiTag {
    #[serde(default)]
    values: Vec<String>,
}

/// Fetch (or read from cache) the full recipe data and parse it into draftbud `Recipe`s,
/// indexed by output item. Recipes that only differ by crafting method are merged.
pub(super) fn load_recipes() -> Result<RecipesByOutput, String> {
    let version = mc_version();
    let url = format!("{API_BASE_URL}/{version}-summary/data/recipe/data.min.json");
    let json = utils::fetch_cached(&version, "recipes.json", &url)?;

    // Parse loosely, then convert each entry on its own, so an unexpected recipe
    // type is skipped rather than breaking the whole file.
    let entries: HashMap<String, serde_json::Value> =
        serde_json::from_str(&json).map_err(|e| format!("failed to parse recipes: {e}"))?;

    let mut by_output: RecipesByOutput = HashMap::new();
    for value in entries.into_values() {
        if let Ok(raw) = serde_json::from_value::<ApiRecipe>(value)
            && let Some((output, recipe)) = to_recipe(raw)
        {
            by_output.entry(output).or_default().push(recipe);
        }
    }

    merge::merge_method_variants(&mut by_output);
    Ok(by_output)
}

/// Resolve a `#tag` reference to a single item id, following nested
/// tags. Returns `None` if the tag file can't be fetched or is empty.
pub(super) fn resolve_tag_to_item(tag: &ItemId) -> Option<ItemId> {
    let version = mc_version();
    let tag = tag.as_str();
    let url = format!("{API_BASE_URL}/{version}-data/data/minecraft/tags/item/{tag}.json");
    let json = utils::fetch_cached(&version, &format!("tag-{tag}.json"), &url).ok()?;
    let first = serde_json::from_str::<ApiTag>(&json)
        .ok()?
        .values
        .into_iter()
        .next()?;
    match first.strip_prefix('#') {
        Some(nested) => resolve_tag_to_item(&ItemId::parse(nested)),
        None => Some(ItemId::parse(&first)),
    }
}

/// Convert a single entry to its output id and `Recipe`. `None` if it isn't in
/// an expected format.
fn to_recipe(raw: ApiRecipe) -> Option<(ItemId, Recipe)> {
    let result = raw.result?;
    let output = ItemId::parse(&result.id);
    let yields = result.count.unwrap_or(1);

    let layout = match (raw.pattern, raw.key) {
        // shaped: look each pattern character up in the key. A space is never a
        // key, so `get` returns `None`, which becomes an empty slot.
        (Some(pattern), Some(key)) => {
            let mut grid = Vec::new();
            for line in &pattern {
                let row = line
                    .chars()
                    .map(|c| key.get(&c.to_string()).map(|s| parse_ingredient(s)));
                grid.push(row.collect());
            }
            Layout::Shaped(grid)
        }
        // shapeless: a flat list, or a lone `ingredient` (smelting/stonecutting)
        _ => {
            let list = match raw.ingredients {
                Some(items) => items.iter().map(|s| parse_ingredient(s)).collect(),
                None => vec![parse_ingredient(raw.ingredient.as_ref()?)],
            };
            Layout::Shapeless(list)
        }
    };

    let recipe = Recipe {
        methods: vec![utils::strip_namespace(&raw.method).into()],
        layout,
        yields,
    };
    Some((output, recipe))
}

/// Parse an item id or `#tag` reference into an `Ingredient`.
fn parse_ingredient(raw: &str) -> Ingredient {
    match raw.strip_prefix('#') {
        Some(tag) => Ingredient::Tag(ItemId::parse(tag)),
        None => Ingredient::Item(ItemId::parse(raw)),
    }
}
