use std::collections::BTreeMap;

use comfy_table::Cell;

use crate::utils::{humanize, new_table};

use super::condense::{Amount, Step};
use super::types::{Ingredient, ItemId, Layout, Recipe};

/// Print every recipe that produces `id`, one table per variant.
pub(super) fn print_recipes(id: &ItemId, recipes: &[Recipe]) {
    println!("Recipes for {}:\n", humanize(id.as_str()));

    for (n, recipe) in recipes.iter().enumerate() {
        let methods = recipe.methods.join(" / ");
        if recipes.len() > 1 {
            println!("Variant {} ({methods}):", n + 1);
        } else {
            println!("{methods}:");
        }

        let mut table = new_table();
        match &recipe.layout {
            Layout::Shaped(grid) => {
                for row in grid {
                    let cells: Vec<Cell> = row
                        .iter()
                        .map(|c| {
                            Cell::new(c.as_ref().map(Ingredient::display_name).unwrap_or_default())
                        })
                        .collect();
                    table.add_row(cells);
                }
            }
            Layout::Shapeless(list) => {
                // count duplicates so "2x Oak Planks" reads cleanly
                let mut counts: BTreeMap<String, u32> = BTreeMap::new();
                for ingredient in list {
                    *counts.entry(ingredient.display_name()).or_insert(0) += 1;
                }
                for (name, count) in counts {
                    table.add_row(vec![Cell::new(format!("{count}x")), Cell::new(name)]);
                }
            }
        }

        println!("{table}");
        println!("-> yields {} {}\n", recipe.yields, humanize(id.as_str()));
    }
}

/// Print the crafting steps needed to make one `id`, top item first.
pub(super) fn print_steps(id: &ItemId, steps: &[Step]) {
    println!("Crafting steps for {}:\n", humanize(id.as_str()));

    let mut table = new_table();
    for step in steps {
        let many_recipes = step.alternatives.len() > 1;
        let inputs = step
            .alternatives
            .iter()
            .map(|alt| {
                let list = alt.iter().map(format_amount).collect::<Vec<_>>().join(", ");
                // parenthesize a multi-input recipe when it sits beside alternatives
                if many_recipes && alt.len() > 1 {
                    format!("({list})")
                } else {
                    list
                }
            })
            .collect::<Vec<_>>()
            .join(" OR ");

        table.add_row(vec![
            Cell::new(inputs),
            Cell::new("→"),
            Cell::new(format_amount(&step.output)),
        ]);
    }

    println!("{table}");
}

/// e.g. `2x Any Oak Logs`. Quantities are rounded up to a craftable whole.
fn format_amount(amount: &Amount) -> String {
    format!("{}x {}", amount.qty.ceil() as u64, amount.name)
}
