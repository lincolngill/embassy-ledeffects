use smart_leds::{RGB8, colors};

///The ColourEnrtry struct contains an RGB8 colour value and it's name.
pub struct ColourEntry {
    pub name: &'static str,
    pub colour: smart_leds::RGB8,
}

/// Reduce the brightness of a colour by a fixed amount, or increase it to a minimum if it is too dark.
const fn reduce_colour(c: RGB8) -> RGB8 {
    const REDUCTION: u8 = 70;
    RGB8 {
        r: c.r.saturating_sub(REDUCTION),
        g: c.g.saturating_sub(REDUCTION),
        b: c.b.saturating_sub(REDUCTION),
    }
}

/// An array constant of all web colour RGB8 values.
///
/// Designed to allow access to a colour via an index. E.g. for random colour selection.
/// This is an array of the individual constants found in [smart_leds::colors].
///
/// The COLOURS array is quite big because it has a name text string for each of the 140 colours.
///
/// See <https://en.wikipedia.org/wiki/Web_colors>
pub const COLOURS: [ColourEntry; 140] = [
    ColourEntry {
        name: "ALICE_BLUE",
        colour: reduce_colour(colors::ALICE_BLUE),
    },
    ColourEntry {
        name: "ANTINQUE_WHITE",
        colour: reduce_colour(colors::ANTINQUE_WHITE),
    },
    ColourEntry {
        name: "AQUA",
        colour: reduce_colour(colors::AQUA),
    },
    ColourEntry {
        name: "AQUAMARINE",
        colour: reduce_colour(colors::AQUAMARINE),
    },
    ColourEntry {
        name: "AZURE",
        colour: reduce_colour(colors::AZURE),
    },
    ColourEntry {
        name: "BEIGE",
        colour: reduce_colour(colors::BEIGE),
    },
    ColourEntry {
        name: "BISQUE",
        colour: reduce_colour(colors::BISQUE),
    },
    ColourEntry {
        name: "BLACK",
        colour: reduce_colour(colors::BLACK),
    },
    ColourEntry {
        name: "BLANCHED_ALMOND",
        colour: reduce_colour(colors::BLANCHED_ALMOND),
    },
    ColourEntry {
        name: "BLUE_VIOLET",
        colour: reduce_colour(colors::BLUE_VIOLET),
    },
    ColourEntry {
        name: "BLUE",
        colour: reduce_colour(colors::BLUE),
    },
    ColourEntry {
        name: "BROWN",
        colour: reduce_colour(colors::BROWN),
    },
    ColourEntry {
        name: "BURLYWOOD",
        colour: reduce_colour(colors::BURLYWOOD),
    },
    ColourEntry {
        name: "CADET_BLUE",
        colour: reduce_colour(colors::CADET_BLUE),
    },
    ColourEntry {
        name: "CHARTREUSE",
        colour: reduce_colour(colors::CHARTREUSE),
    },
    ColourEntry {
        name: "CHOCOLATE",
        colour: reduce_colour(colors::CHOCOLATE),
    },
    ColourEntry {
        name: "CORAL",
        colour: reduce_colour(colors::CORAL),
    },
    ColourEntry {
        name: "CORNFLOWER_BLUE",
        colour: reduce_colour(colors::CORNFLOWER_BLUE),
    },
    ColourEntry {
        name: "CORNSILK",
        colour: reduce_colour(colors::CORNSILK),
    },
    ColourEntry {
        name: "CRIMSON",
        colour: reduce_colour(colors::CRIMSON),
    },
    ColourEntry {
        name: "CYAN",
        colour: reduce_colour(colors::CYAN),
    },
    ColourEntry {
        name: "DARK_BLUE",
        colour: reduce_colour(colors::DARK_BLUE),
    },
    ColourEntry {
        name: "DARK_CYAN",
        colour: reduce_colour(colors::DARK_CYAN),
    },
    ColourEntry {
        name: "DARK_GOLDENROD",
        colour: reduce_colour(colors::DARK_GOLDENROD),
    },
    ColourEntry {
        name: "DARK_GRAY",
        colour: reduce_colour(colors::DARK_GRAY),
    },
    ColourEntry {
        name: "DARK_GREEN",
        colour: reduce_colour(colors::DARK_GREEN),
    },
    ColourEntry {
        name: "DARK_KHAKI",
        colour: reduce_colour(colors::DARK_KHAKI),
    },
    ColourEntry {
        name: "DARK_MAGENTA",
        colour: reduce_colour(colors::DARK_MAGENTA),
    },
    ColourEntry {
        name: "DARK_OLIVE_GREEN",
        colour: reduce_colour(colors::DARK_OLIVE_GREEN),
    },
    ColourEntry {
        name: "DARK_ORANGE",
        colour: reduce_colour(colors::DARK_ORANGE),
    },
    ColourEntry {
        name: "DARK_ORCHID",
        colour: reduce_colour(colors::DARK_ORCHID),
    },
    ColourEntry {
        name: "DARK_RED",
        colour: reduce_colour(colors::DARK_RED),
    },
    ColourEntry {
        name: "DARK_SALMON",
        colour: reduce_colour(colors::DARK_SALMON),
    },
    ColourEntry {
        name: "DARK_SEA_GREEN",
        colour: reduce_colour(colors::DARK_SEA_GREEN),
    },
    ColourEntry {
        name: "DARK_SLATE_BLUE",
        colour: reduce_colour(colors::DARK_SLATE_BLUE),
    },
    ColourEntry {
        name: "DARK_SLATE_GRAY",
        colour: reduce_colour(colors::DARK_SLATE_GRAY),
    },
    ColourEntry {
        name: "DARK_TURQUOISE",
        colour: reduce_colour(colors::DARK_TURQUOISE),
    },
    ColourEntry {
        name: "DARK_VIOLET",
        colour: reduce_colour(colors::DARK_VIOLET),
    },
    ColourEntry {
        name: "DEEP_PINK",
        colour: reduce_colour(colors::DEEP_PINK),
    },
    ColourEntry {
        name: "DEEP_SKY_BLUE",
        colour: reduce_colour(colors::DEEP_SKY_BLUE),
    },
    ColourEntry {
        name: "DIM_GRAY",
        colour: reduce_colour(colors::DIM_GRAY),
    },
    ColourEntry {
        name: "DODGER_BLUE",
        colour: reduce_colour(colors::DODGER_BLUE),
    },
    ColourEntry {
        name: "FIREBRICK",
        colour: reduce_colour(colors::FIREBRICK),
    },
    ColourEntry {
        name: "FLORAL_WHITE",
        colour: reduce_colour(colors::FLORAL_WHITE),
    },
    ColourEntry {
        name: "FOREST_GREEN",
        colour: reduce_colour(colors::FOREST_GREEN),
    },
    ColourEntry {
        name: "FUCHSIA",
        colour: reduce_colour(colors::FUCHSIA),
    },
    ColourEntry {
        name: "GAINSBORO",
        colour: reduce_colour(colors::GAINSBORO),
    },
    ColourEntry {
        name: "GHOST_WHITE",
        colour: reduce_colour(colors::GHOST_WHITE),
    },
    ColourEntry {
        name: "GOLD",
        colour: reduce_colour(colors::GOLD),
    },
    ColourEntry {
        name: "GOLDENROD",
        colour: reduce_colour(colors::GOLDENROD),
    },
    ColourEntry {
        name: "GRAY",
        colour: reduce_colour(colors::GRAY),
    },
    ColourEntry {
        name: "GREEN_YELLOW",
        colour: reduce_colour(colors::GREEN_YELLOW),
    },
    ColourEntry {
        name: "GREEN",
        colour: reduce_colour(colors::GREEN),
    },
    ColourEntry {
        name: "HONEYDEW",
        colour: reduce_colour(colors::HONEYDEW),
    },
    ColourEntry {
        name: "HOT_PINK",
        colour: reduce_colour(colors::HOT_PINK),
    },
    ColourEntry {
        name: "INDIAN_RED",
        colour: reduce_colour(colors::INDIAN_RED),
    },
    ColourEntry {
        name: "INDIGO",
        colour: reduce_colour(colors::INDIGO),
    },
    ColourEntry {
        name: "IVORY",
        colour: reduce_colour(colors::IVORY),
    },
    ColourEntry {
        name: "KHAKI",
        colour: reduce_colour(colors::KHAKI),
    },
    ColourEntry {
        name: "LAVENDER_BLUSH",
        colour: reduce_colour(colors::LAVENDER_BLUSH),
    },
    ColourEntry {
        name: "LAVENDER",
        colour: reduce_colour(colors::LAVENDER),
    },
    ColourEntry {
        name: "LAWN_GREEN",
        colour: reduce_colour(colors::LAWN_GREEN),
    },
    ColourEntry {
        name: "LEMON_CHIFFON",
        colour: reduce_colour(colors::LEMON_CHIFFON),
    },
    ColourEntry {
        name: "LIGHT_BLUE",
        colour: reduce_colour(colors::LIGHT_BLUE),
    },
    ColourEntry {
        name: "LIGHT_CORAL",
        colour: reduce_colour(colors::LIGHT_CORAL),
    },
    ColourEntry {
        name: "LIGHT_CYAN",
        colour: reduce_colour(colors::LIGHT_CYAN),
    },
    ColourEntry {
        name: "LIGHT_GOLDENROD_YELLOW",
        colour: reduce_colour(colors::LIGHT_GOLDENROD_YELLOW),
    },
    ColourEntry {
        name: "LIGHT_GRAY",
        colour: reduce_colour(colors::LIGHT_GRAY),
    },
    ColourEntry {
        name: "LIGHT_GREEN",
        colour: reduce_colour(colors::LIGHT_GREEN),
    },
    ColourEntry {
        name: "LIGHT_PINK",
        colour: reduce_colour(colors::LIGHT_PINK),
    },
    ColourEntry {
        name: "LIGHT_SALMON",
        colour: reduce_colour(colors::LIGHT_SALMON),
    },
    ColourEntry {
        name: "LIGHT_SEA_GREEN",
        colour: reduce_colour(colors::LIGHT_SEA_GREEN),
    },
    ColourEntry {
        name: "LIGHT_SKY_BLUE",
        colour: reduce_colour(colors::LIGHT_SKY_BLUE),
    },
    ColourEntry {
        name: "LIGHT_SLATE_GRAY",
        colour: reduce_colour(colors::LIGHT_SLATE_GRAY),
    },
    ColourEntry {
        name: "LIGHT_STEEL_BLUE",
        colour: reduce_colour(colors::LIGHT_STEEL_BLUE),
    },
    ColourEntry {
        name: "LIGHT_YELLOW",
        colour: reduce_colour(colors::LIGHT_YELLOW),
    },
    ColourEntry {
        name: "LIME_GREEN",
        colour: reduce_colour(colors::LIME_GREEN),
    },
    ColourEntry {
        name: "LIME",
        colour: reduce_colour(colors::LIME),
    },
    ColourEntry {
        name: "LINEN",
        colour: reduce_colour(colors::LINEN),
    },
    ColourEntry {
        name: "MAGENTA",
        colour: reduce_colour(colors::MAGENTA),
    },
    ColourEntry {
        name: "MAROON",
        colour: reduce_colour(colors::MAROON),
    },
    ColourEntry {
        name: "MEDIUM_AQUAMARINE",
        colour: reduce_colour(colors::MEDIUM_AQUAMARINE),
    },
    ColourEntry {
        name: "MEDIUM_BLUE",
        colour: reduce_colour(colors::MEDIUM_BLUE),
    },
    ColourEntry {
        name: "MEDIUM_ORCHID",
        colour: reduce_colour(colors::MEDIUM_ORCHID),
    },
    ColourEntry {
        name: "MEDIUM_PURPLE",
        colour: reduce_colour(colors::MEDIUM_PURPLE),
    },
    ColourEntry {
        name: "MEDIUM_SEA_GREEN",
        colour: reduce_colour(colors::MEDIUM_SEA_GREEN),
    },
    ColourEntry {
        name: "MEDIUM_SLATE_BLUE",
        colour: reduce_colour(colors::MEDIUM_SLATE_BLUE),
    },
    ColourEntry {
        name: "MEDIUM_SPRING_GREEN",
        colour: reduce_colour(colors::MEDIUM_SPRING_GREEN),
    },
    ColourEntry {
        name: "MEDIUM_TURQUOISE",
        colour: reduce_colour(colors::MEDIUM_TURQUOISE),
    },
    ColourEntry {
        name: "MEDIUM_VIOLET_RED",
        colour: reduce_colour(colors::MEDIUM_VIOLET_RED),
    },
    ColourEntry {
        name: "MIDNIGHT_BLUE",
        colour: reduce_colour(colors::MIDNIGHT_BLUE),
    },
    ColourEntry {
        name: "MINT_CREAM",
        colour: reduce_colour(colors::MINT_CREAM),
    },
    ColourEntry {
        name: "MISTY_ROSE",
        colour: reduce_colour(colors::MISTY_ROSE),
    },
    ColourEntry {
        name: "MOCCASIN",
        colour: reduce_colour(colors::MOCCASIN),
    },
    ColourEntry {
        name: "NAVAJO_WHITE",
        colour: reduce_colour(colors::NAVAJO_WHITE),
    },
    ColourEntry {
        name: "NAVY",
        colour: reduce_colour(colors::NAVY),
    },
    ColourEntry {
        name: "OLD_LACE",
        colour: reduce_colour(colors::OLD_LACE),
    },
    ColourEntry {
        name: "OLIVE_DRAB",
        colour: reduce_colour(colors::OLIVE_DRAB),
    },
    ColourEntry {
        name: "OLIVE",
        colour: reduce_colour(colors::OLIVE),
    },
    ColourEntry {
        name: "ORANGE_RED",
        colour: reduce_colour(colors::ORANGE_RED),
    },
    ColourEntry {
        name: "ORANGE",
        colour: reduce_colour(colors::ORANGE),
    },
    ColourEntry {
        name: "ORCHID",
        colour: reduce_colour(colors::ORCHID),
    },
    ColourEntry {
        name: "PALE_GOLDENROD",
        colour: reduce_colour(colors::PALE_GOLDENROD),
    },
    ColourEntry {
        name: "PALE_GREEN",
        colour: reduce_colour(colors::PALE_GREEN),
    },
    ColourEntry {
        name: "PALE_TURQUOISE",
        colour: reduce_colour(colors::PALE_TURQUOISE),
    },
    ColourEntry {
        name: "PALE_VIOLET_RED",
        colour: reduce_colour(colors::PALE_VIOLET_RED),
    },
    ColourEntry {
        name: "PAPAYA_WHIP",
        colour: reduce_colour(colors::PAPAYA_WHIP),
    },
    ColourEntry {
        name: "PEACH_PUFF",
        colour: reduce_colour(colors::PEACH_PUFF),
    },
    ColourEntry {
        name: "PERU",
        colour: reduce_colour(colors::PERU),
    },
    ColourEntry {
        name: "PINK",
        colour: reduce_colour(colors::PINK),
    },
    ColourEntry {
        name: "PLUM",
        colour: reduce_colour(colors::PLUM),
    },
    ColourEntry {
        name: "POWDER_BLUE",
        colour: reduce_colour(colors::POWDER_BLUE),
    },
    ColourEntry {
        name: "PURPLE",
        colour: reduce_colour(colors::PURPLE),
    },
    ColourEntry {
        name: "RED",
        colour: reduce_colour(colors::RED),
    },
    ColourEntry {
        name: "ROSY_BROWN",
        colour: reduce_colour(colors::ROSY_BROWN),
    },
    ColourEntry {
        name: "ROYAL_BLUE",
        colour: reduce_colour(colors::ROYAL_BLUE),
    },
    ColourEntry {
        name: "SADDLE_BROWN",
        colour: reduce_colour(colors::SADDLE_BROWN),
    },
    ColourEntry {
        name: "SALMON",
        colour: reduce_colour(colors::SALMON),
    },
    ColourEntry {
        name: "SANDY_BROWN",
        colour: reduce_colour(colors::SANDY_BROWN),
    },
    ColourEntry {
        name: "SEA_GREEN",
        colour: reduce_colour(colors::SEA_GREEN),
    },
    ColourEntry {
        name: "SEASHELL",
        colour: reduce_colour(colors::SEASHELL),
    },
    ColourEntry {
        name: "SIENNA",
        colour: reduce_colour(colors::SIENNA),
    },
    ColourEntry {
        name: "SILVER",
        colour: reduce_colour(colors::SILVER),
    },
    ColourEntry {
        name: "SKY_BLUE",
        colour: reduce_colour(colors::SKY_BLUE),
    },
    ColourEntry {
        name: "SLATE_BLUE",
        colour: reduce_colour(colors::SLATE_BLUE),
    },
    ColourEntry {
        name: "SLATE_GRAY",
        colour: reduce_colour(colors::SLATE_GRAY),
    },
    ColourEntry {
        name: "SNOW",
        colour: reduce_colour(colors::SNOW),
    },
    ColourEntry {
        name: "SPRING_GREEN",
        colour: reduce_colour(colors::SPRING_GREEN),
    },
    ColourEntry {
        name: "STEEL_BLUE",
        colour: reduce_colour(colors::STEEL_BLUE),
    },
    ColourEntry {
        name: "TAN",
        colour: reduce_colour(colors::TAN),
    },
    ColourEntry {
        name: "TEAL",
        colour: reduce_colour(colors::TEAL),
    },
    ColourEntry {
        name: "THISTLE",
        colour: reduce_colour(colors::THISTLE),
    },
    ColourEntry {
        name: "TOMATO",
        colour: reduce_colour(colors::TOMATO),
    },
    ColourEntry {
        name: "TURQUOISE",
        colour: reduce_colour(colors::TURQUOISE),
    },
    ColourEntry {
        name: "VIOLET",
        colour: reduce_colour(colors::VIOLET),
    },
    ColourEntry {
        name: "WHEAT",
        colour: reduce_colour(colors::WHEAT),
    },
    ColourEntry {
        name: "WHITE_SMOKE",
        colour: reduce_colour(colors::WHITE_SMOKE),
    },
    ColourEntry {
        name: "WHITE",
        colour: reduce_colour(colors::WHITE),
    },
    ColourEntry {
        name: "YELLOW_GREEN",
        colour: reduce_colour(colors::YELLOW_GREEN),
    },
    ColourEntry {
        name: "YELLOW",
        colour: reduce_colour(colors::YELLOW),
    },
];
