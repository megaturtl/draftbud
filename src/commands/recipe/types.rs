use std::collections::HashMap;

use crate::utils::{humanize, strip_namespace};

/// A normalized item or tag id, e.g. `oak_planks`
#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) struct ItemId(String);

impl ItemId {
    /// The single place item/tag identity is defined: namespace-stripped,
    /// lowercased, spaces-to-underscores.
    pub(super) fn parse(raw: &str) -> Self {
        ItemId(strip_namespace(raw.trim()).to_lowercase().replace(' ', "_"))
    }

    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

/// Every known recipe, keyed by the id of the item it produces, e.g.
/// `oak_planks`. One item can have several recipes.
pub(super) type RecipesByOutput = HashMap<ItemId, Vec<Recipe>>;

#[derive(Clone)]
pub(super) enum Ingredient {
    /// One specific item, e.g. `oak_planks`.
    Item(ItemId),
    /// Any item belonging to a tag, e.g. `#planks`.
    Tag(ItemId),
}

impl Ingredient {
    /// Human-readable name for display, e.g. `Oak Planks` or `Any Planks`.
    pub(super) fn display_name(&self) -> String {
        match self {
            Ingredient::Item(id) => humanize(id.as_str()),
            Ingredient::Tag(tag) => format!("Any {}", humanize(tag.as_str())),
        }
    }

    /// Canonical string identity, used to tell ingredients apart when grouping
    /// recipes, e.g. `oak_planks` or `#planks`. The `#` keeps a tag distinct
    /// from an item that happens to share its name.
    pub(super) fn token(&self) -> String {
        match self {
            Ingredient::Item(id) => id.as_str().to_owned(),
            Ingredient::Tag(tag) => format!("#{}", tag.as_str()),
        }
    }
}

pub(super) enum Layout {
    /// A grid of slots (`None` is an empty slot) for shaped crafting.
    Shaped(Vec<Vec<Option<Ingredient>>>),
    /// An unordered list of ingredients for shapeless crafting or smelting.
    Shapeless(Vec<Ingredient>),
}

pub(super) struct Recipe {
    /// Crafting methods that produce this recipe, e.g. `["smelting", "blasting"]`.
    pub(super) methods: Vec<String>,
    pub(super) layout: Layout,
    /// How many output items from a single craft.
    pub(super) yields: u32,
}

impl Recipe {
    /// Flat list of input ingredients without empty slots
    pub(super) fn ingredients(&self) -> Vec<Ingredient> {
        match &self.layout {
            Layout::Shaped(grid) => grid.iter().flatten().flatten().cloned().collect(),
            Layout::Shapeless(list) => list.clone(),
        }
    }

    /// How many times this recipe must run to yield `qty` outputs (may be fractional).
    pub(super) fn crafts_for(&self, qty: f64) -> f64 {
        qty / self.yields.max(1) as f64
    }
}
