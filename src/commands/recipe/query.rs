use super::types::{ItemId, RecipesByOutput};

/// Resolve an item query (display name, snake_case id, or namespaced id) to its id
pub(super) fn find(recipes: &RecipesByOutput, query: &str) -> Option<ItemId> {
    let id = ItemId::parse(query);
    recipes.contains_key(&id).then_some(id)
}
