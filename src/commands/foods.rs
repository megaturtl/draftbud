use crate::utils::print_named_lists;

const DEFAULT: &str = include_str!("../../data/foods.json");

pub fn run() {
    print_named_lists(
        "foods.json",
        DEFAULT,
        "Foods and how to obtain:",
        ["Food", "Methods"],
    );
}
