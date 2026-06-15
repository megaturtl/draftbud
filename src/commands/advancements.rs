use crate::utils::print_named_lists;

const DEFAULT: &str = include_str!("../../data/advancements.json");

pub fn run() {
    print_named_lists(
        "advancements.json",
        DEFAULT,
        "Easy advancements:",
        ["Category", "Advancement"],
    );
}
