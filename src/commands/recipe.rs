//! `recipe` command: look up crafting recipes, optionally condensing them into a
//! step-by-step crafting tree.
//!
//! [`api`] downloads + parses the recipe JSON into [`types`], then
//! [`condense`] breaks an item down into steps, which [`render`] prints.

mod api;
mod condense;
mod merge;
mod render;
mod types;

use types::ItemId;

pub fn run(item: String, condense: bool) {
    let recipes = match api::load_recipes() {
        Ok(recipes) => recipes,
        Err(e) => return eprintln!("error: {e}"),
    };

    let id = ItemId::parse(&item);
    if !recipes.contains_key(&id) {
        return eprintln!("No crafting recipe found for {item:?}.");
    }

    if condense {
        let steps = condense::crafting_steps(&recipes, &id);
        render::print_steps(&id, &steps);
    } else {
        render::print_recipes(&id, &recipes[&id]);
    }
}
