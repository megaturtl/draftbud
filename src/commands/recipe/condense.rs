use std::collections::HashSet;

use crate::utils::humanize;

use super::api;
use super::types::{Ingredient, ItemId, Recipe, RecipesByOutput};

/// One amount of a named thing in a step, e.g. `2x Any Oak Logs`.
pub(super) struct Amount {
    pub(super) qty: f64,
    pub(super) name: String,
}

/// A single craft producing `output`. `alternatives` holds one input list per
/// recipe that can make it; more than one is rendered as "A OR B".
pub(super) struct Step {
    pub(super) output: Amount,
    pub(super) alternatives: Vec<Vec<Amount>>,
}

/// One recipe's inputs: display amounts for every ingredient, plus its craftable
/// ingredients paired with the id + quantity needed to recurse into them.
struct RecipeInputs {
    amounts: Vec<Amount>,
    children: Vec<(ItemId, f64)>,
}

/// Break crafting one `item` down into ordered crafting steps, top item first.
pub(super) fn crafting_steps(recipes: &RecipesByOutput, item: &ItemId) -> Vec<Step> {
    let mut steps = Vec::new();
    expand(recipes, item, 1.0, &mut steps, &mut Vec::new());
    steps
}

/// Emit a step for `qty` of `item` (if it's craftable), then recurse into its
/// inputs. `path` is the chain of items currently being expanded, used to drop
/// recipes that would loop back on themselves.
fn expand(
    recipes: &RecipesByOutput,
    item: &ItemId,
    qty: f64,
    steps: &mut Vec<Step>,
    path: &mut Vec<ItemId>,
) {
    let mut targets = path.clone();
    targets.push(item.clone());

    let candidates: Vec<&Recipe> = match recipes.get(item) {
        Some(recipes_for_item) => recipes_for_item
            .iter()
            .filter(|r| !is_circular(recipes, r, &targets))
            .collect(),
        // no recipe, or every recipe loops back: a base material, no step
        None => return,
    };
    if candidates.is_empty() {
        return;
    }

    let computed: Vec<RecipeInputs> = candidates
        .iter()
        .map(|recipe| inputs(recipes, recipe, recipe.crafts_for(qty)))
        .collect();

    // A single recipe is walked down to its base materials; a genuine choice of
    // recipes is shown OR'd but not branched into (so `children` stays empty).
    let children = match &computed[..] {
        [single] => single.children.clone(),
        _ => Vec::new(),
    };

    steps.push(Step {
        output: Amount {
            qty,
            name: humanize(item.as_str()),
        },
        alternatives: computed.into_iter().map(|inputs| inputs.amounts).collect(),
    });

    if !children.is_empty() {
        path.push(item.clone());
        for (child, child_qty) in children {
            expand(recipes, &child, child_qty, steps, path);
        }
        path.pop();
    }
}

/// `recipe`'s inputs scaled by `crafts`. Identical ingredients are merged,
/// keeping recipe order.
fn inputs(recipes: &RecipesByOutput, recipe: &Recipe, crafts: f64) -> RecipeInputs {
    let mut amounts: Vec<Amount> = Vec::new();
    let mut children: Vec<(ItemId, f64)> = Vec::new();

    for ingredient in recipe.ingredients() {
        let (name, craftable) = classify(recipes, &ingredient);

        match amounts.iter_mut().find(|a| a.name == name) {
            Some(existing) => existing.qty += crafts,
            None => amounts.push(Amount { qty: crafts, name }),
        }
        if let Some(id) = craftable {
            match children.iter_mut().find(|(c, _)| *c == id) {
                Some((_, q)) => *q += crafts,
                None => children.push((id, crafts)),
            }
        }
    }

    RecipeInputs { amounts, children }
}

/// An ingredient's display name and, if it can be broken down further, the id to
/// recurse into. A tag resolves through a member; a member with no recipe leaves
/// the tag generic ("Any Oak Logs").
fn classify(recipes: &RecipesByOutput, ingredient: &Ingredient) -> (String, Option<ItemId>) {
    match resolve_to_item(ingredient) {
        // craftable item, or tag whose member is craftable: name it concretely
        Some(id) if recipes.contains_key(&id) => (humanize(id.as_str()), Some(id)),
        // base material, or a tag with no craftable member: a leaf
        _ => (ingredient.display_name(), None),
    }
}

/// The concrete item an ingredient points at: itself, or a tag's representative
/// member. `None` only if a tag can't be resolved.
fn resolve_to_item(ingredient: &Ingredient) -> Option<ItemId> {
    match ingredient {
        Ingredient::Item(id) => Some(id.clone()),
        Ingredient::Tag(tag) => api::resolve_tag_to_item(tag),
    }
}

/// Whether crafting `recipe` would (transitively) require something already
/// being expanded, e.g. an iron block recipe while expanding iron ingots.
fn is_circular(recipes: &RecipesByOutput, recipe: &Recipe, targets: &[ItemId]) -> bool {
    recipe
        .ingredients()
        .iter()
        .filter_map(resolve_to_item)
        .any(|child| depends_on(recipes, &child, targets, &mut HashSet::new()))
}

/// Whether crafting `item` ever requires one of `targets`.
fn depends_on(
    recipes: &RecipesByOutput,
    item: &ItemId,
    targets: &[ItemId],
    visited: &mut HashSet<ItemId>,
) -> bool {
    if targets.contains(item) {
        return true;
    }
    if !visited.insert(item.clone()) {
        return false;
    }
    recipes
        .get(item)
        .into_iter()
        .flatten()
        .flat_map(Recipe::ingredients)
        .filter_map(|ingredient| resolve_to_item(&ingredient))
        .any(|child| depends_on(recipes, &child, targets, visited))
}
