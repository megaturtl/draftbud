use colored::Color;

struct Stew {
    flowers: &'static [&'static str],
    effect: &'static str,
    locations: &'static [&'static str],
    duration: &'static str,
    color: Color,
}

const STEWS: &[Stew] = &[
    Stew {
        flowers: &["Allium"],
        effect: "Fire Resistance",
        locations: &["Flower Forest", "Meadow", "Woodland Mansion (Potted)"],
        duration: "3s",
        color: Color::Red,
    },
    Stew {
        flowers: &["Azure Bluet", "Open Eyeblossom"],
        effect: "Blindness",
        locations: &[
            "Plains",
            "Sunflower Plains",
            "Flower Forest",
            "Meadow",
            "Woodland Mansion (Potted)",
        ],
        duration: "11s",
        color: Color::BrightBlack,
    },
    Stew {
        flowers: &["Blue Orchid", "Dandelion", "Golden Dandelion"],
        effect: "Saturation",
        locations: &[
            "Swamp",
            "Woodland Mansion (Potted)",
            "Dandelions can spawn on dirt/grass in all biomes except Swamps, Badlands, Pale, Cherry, Mountains",
        ],
        duration: "0.35s",
        color: Color::Yellow,
    },
    Stew {
        flowers: &["Closed Eyeblossom"],
        effect: "Nausea",
        locations: &["Pale Garden"],
        duration: "7s",
        color: Color::Green,
    },
    Stew {
        flowers: &["Cornflower"],
        effect: "Jump Boost",
        locations: &[
            "Plains",
            "Sunflower Plains",
            "Flower Forest",
            "Meadow",
            "Woodland Mansion (Potted)",
        ],
        duration: "5s",
        color: Color::BrightBlue,
    },
    Stew {
        flowers: &["Lily of the Valley"],
        effect: "Poison",
        locations: &["Forest", "Flower Forest", "Birch Forest", "Dark Forest"],
        duration: "11s",
        color: Color::Green,
    },
    Stew {
        flowers: &["Oxeye Daisy"],
        effect: "Regeneration",
        locations: &[
            "Plains",
            "Sunflower Plains",
            "Flower Forest",
            "Meadow",
            "Woodland Mansion (Potted)",
        ],
        duration: "7s",
        color: Color::Magenta,
    },
    Stew {
        flowers: &["Poppy", "Torchflower"],
        effect: "Night Vision",
        locations: &[
            "Poppies can spawn on dirt/grass in all biomes except Swamps, Badlands, Pale, Cherry, Mountains",
        ],
        duration: "5s",
        color: Color::BrightYellow,
    },
    Stew {
        flowers: &["Tulips"],
        effect: "Weakness",
        locations: &[
            "Plains (rare tulip patches)",
            "Sunflower Plains (rare tulip patches)",
            "Flower Forest",
            "Woodland Mansion (Potted)",
        ],
        duration: "7s",
        color: Color::White,
    },
    Stew {
        flowers: &["Wither Rose"],
        effect: "Wither",
        locations: &["Drops upon Wither killing a mob"],
        duration: "7s",
        color: Color::BrightBlack,
    },
];

use comfy_table::presets::{UTF8_HORIZONTAL_ONLY};
use comfy_table::{Cell, Color as TableColor, ContentArrangement, Table};

fn cell_color(c: Color) -> TableColor {
    match c {
        Color::Red => TableColor::Red,
        Color::Green => TableColor::Green,
        Color::Yellow => TableColor::Yellow,
        Color::Blue => TableColor::Blue,
        Color::Magenta => TableColor::Magenta,
        Color::White => TableColor::White,
        Color::BrightBlack => TableColor::DarkGrey,
        Color::BrightBlue => TableColor::Blue,
        Color::BrightYellow => TableColor::Yellow,
        _ => TableColor::Reset,
    }
}

pub fn run() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_HORIZONTAL_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Effect", "Duration", "Flowers", "Locations"]);

    for stew in STEWS {
        table.add_row(vec![
            Cell::new(stew.effect).fg(cell_color(stew.color)),
            Cell::new(format!("({})", stew.duration)),
            Cell::new(stew.flowers.join(" / ")),
            Cell::new(stew.locations.join("\n")),
        ]);
    }

    println!("Suspicious Stew effects:\n");
    println!("{table}");
}
