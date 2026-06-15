use std::collections::BTreeMap;

use super::types::{Ingredient, Layout, Recipe, RecipesByOutput};

/// Merge recipes that produce the same item from the same inputs but with
/// different crafting methods. e.g. iron ingot has identical `smelting` and
/// `blasting` recipes, so this combines them into one with both methods.
pub(super) fn merge_method_variants(by_output: &mut RecipesByOutput) {
    for recipes in by_output.values_mut() {
        *recipes = merge_recipes_with_same_inputs(std::mem::take(recipes));
    }
}

fn merge_recipes_with_same_inputs(recipes: Vec<Recipe>) -> Vec<Recipe> {
    let mut groups: BTreeMap<String, Recipe> = BTreeMap::new();
    for recipe in recipes {
        let key = inputs_key(&recipe);
        if let Some(existing) = groups.get_mut(&key) {
            existing.methods.extend(recipe.methods);
        } else {
            groups.insert(key, recipe);
        }
    }

    let mut merged: Vec<Recipe> = groups.into_values().collect();
    for recipe in &mut merged {
        recipe.methods.sort();
        recipe.methods.dedup();
    }
    merged
}

/// Key identifying a recipe by its inputs + yield, ignoring the crafting method,
/// so smelting and blasting of the same item share a key.
fn inputs_key(recipe: &Recipe) -> String {
    let body = match &recipe.layout {
        Layout::Shaped(grid) => {
            let mut rows = Vec::new();
            for row in grid {
                let cells: Vec<String> = row
                    .iter()
                    .map(|c| c.as_ref().map(Ingredient::token).unwrap_or_default())
                    .collect();
                rows.push(cells.join("|"));
            }
            rows.join("/")
        }
        Layout::Shapeless(list) => {
            let mut tokens: Vec<String> = list.iter().map(Ingredient::token).collect();
            tokens.sort();
            tokens.join(",")
        }
    };
    format!("{body} => {}", recipe.yields)
}
