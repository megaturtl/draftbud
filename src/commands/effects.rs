use crate::utils::print_named_lists;

const DEFAULT: &str = include_str!("../../data/effects.json");

pub fn run() {
    print_named_lists(
        "effects.json",
        DEFAULT,
        "Status effects and how to obtain:",
        ["Effect", "Methods"],
    );
}
