//! Example OneColour effect
#![no_std]
#![no_main]
use defmt::*;
use embassy_executor::Spawner;
use embassy_ledeffects::Strip;
use embassy_ledeffects::effect::{self, EffectIterator};
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_time::{Duration, Timer};
use smart_leds::colors;
use {defmt_rtt as _, panic_probe as _};

const NUM_LEDS: usize = 144;
const COLOUR_CHANGE_MS: u64 = 2000;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH0>;
});

struct ColourEntry {
    name: &'static str,
    colour: smart_leds::RGB8,
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);
    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_16, &program);

    let mut strip = Strip::<NUM_LEDS>::new(None, None);
    let mut onecolour_effect = effect::OneColour::new(colors::BLACK);
    loop {
        info!("OneColour - Start");
        for c in COLOUR_ENTRIES {
            onecolour_effect.set(c.colour);
            info!("Colour: {}", c.name);
            onecolour_effect.nextframe(&mut strip).unwrap();
            ws2812.write(&strip.leds).await;
            Timer::after(Duration::from_millis(COLOUR_CHANGE_MS)).await;
        }
    }
}

const COLOUR_ENTRIES: [ColourEntry; 140] = [
    ColourEntry {
        name: "ALICE_BLUE",
        colour: colors::ALICE_BLUE,
    },
    ColourEntry {
        name: "ANTINQUE_WHITE",
        colour: colors::ANTINQUE_WHITE,
    },
    ColourEntry {
        name: "AQUA",
        colour: colors::AQUA,
    },
    ColourEntry {
        name: "AQUAMARINE",
        colour: colors::AQUAMARINE,
    },
    ColourEntry {
        name: "AZURE",
        colour: colors::AZURE,
    },
    ColourEntry {
        name: "BEIGE",
        colour: colors::BEIGE,
    },
    ColourEntry {
        name: "BISQUE",
        colour: colors::BISQUE,
    },
    ColourEntry {
        name: "BLACK",
        colour: colors::BLACK,
    },
    ColourEntry {
        name: "BLANCHED_ALMOND",
        colour: colors::BLANCHED_ALMOND,
    },
    ColourEntry {
        name: "BLUE_VIOLET",
        colour: colors::BLUE_VIOLET,
    },
    ColourEntry {
        name: "BLUE",
        colour: colors::BLUE,
    },
    ColourEntry {
        name: "BROWN",
        colour: colors::BROWN,
    },
    ColourEntry {
        name: "BURLYWOOD",
        colour: colors::BURLYWOOD,
    },
    ColourEntry {
        name: "CADET_BLUE",
        colour: colors::CADET_BLUE,
    },
    ColourEntry {
        name: "CHARTREUSE",
        colour: colors::CHARTREUSE,
    },
    ColourEntry {
        name: "CHOCOLATE",
        colour: colors::CHOCOLATE,
    },
    ColourEntry {
        name: "CORAL",
        colour: colors::CORAL,
    },
    ColourEntry {
        name: "CORNFLOWER_BLUE",
        colour: colors::CORNFLOWER_BLUE,
    },
    ColourEntry {
        name: "CORNSILK",
        colour: colors::CORNSILK,
    },
    ColourEntry {
        name: "CRIMSON",
        colour: colors::CRIMSON,
    },
    ColourEntry {
        name: "CYAN",
        colour: colors::CYAN,
    },
    ColourEntry {
        name: "DARK_BLUE",
        colour: colors::DARK_BLUE,
    },
    ColourEntry {
        name: "DARK_CYAN",
        colour: colors::DARK_CYAN,
    },
    ColourEntry {
        name: "DARK_GOLDENROD",
        colour: colors::DARK_GOLDENROD,
    },
    ColourEntry {
        name: "DARK_GRAY",
        colour: colors::DARK_GRAY,
    },
    ColourEntry {
        name: "DARK_GREEN",
        colour: colors::DARK_GREEN,
    },
    ColourEntry {
        name: "DARK_KHAKI",
        colour: colors::DARK_KHAKI,
    },
    ColourEntry {
        name: "DARK_MAGENTA",
        colour: colors::DARK_MAGENTA,
    },
    ColourEntry {
        name: "DARK_OLIVE_GREEN",
        colour: colors::DARK_OLIVE_GREEN,
    },
    ColourEntry {
        name: "DARK_ORANGE",
        colour: colors::DARK_ORANGE,
    },
    ColourEntry {
        name: "DARK_ORCHID",
        colour: colors::DARK_ORCHID,
    },
    ColourEntry {
        name: "DARK_RED",
        colour: colors::DARK_RED,
    },
    ColourEntry {
        name: "DARK_SALMON",
        colour: colors::DARK_SALMON,
    },
    ColourEntry {
        name: "DARK_SEA_GREEN",
        colour: colors::DARK_SEA_GREEN,
    },
    ColourEntry {
        name: "DARK_SLATE_BLUE",
        colour: colors::DARK_SLATE_BLUE,
    },
    ColourEntry {
        name: "DARK_SLATE_GRAY",
        colour: colors::DARK_SLATE_GRAY,
    },
    ColourEntry {
        name: "DARK_TURQUOISE",
        colour: colors::DARK_TURQUOISE,
    },
    ColourEntry {
        name: "DARK_VIOLET",
        colour: colors::DARK_VIOLET,
    },
    ColourEntry {
        name: "DEEP_PINK",
        colour: colors::DEEP_PINK,
    },
    ColourEntry {
        name: "DEEP_SKY_BLUE",
        colour: colors::DEEP_SKY_BLUE,
    },
    ColourEntry {
        name: "DIM_GRAY",
        colour: colors::DIM_GRAY,
    },
    ColourEntry {
        name: "DODGER_BLUE",
        colour: colors::DODGER_BLUE,
    },
    ColourEntry {
        name: "FIREBRICK",
        colour: colors::FIREBRICK,
    },
    ColourEntry {
        name: "FLORAL_WHITE",
        colour: colors::FLORAL_WHITE,
    },
    ColourEntry {
        name: "FOREST_GREEN",
        colour: colors::FOREST_GREEN,
    },
    ColourEntry {
        name: "FUCHSIA",
        colour: colors::FUCHSIA,
    },
    ColourEntry {
        name: "GAINSBORO",
        colour: colors::GAINSBORO,
    },
    ColourEntry {
        name: "GHOST_WHITE",
        colour: colors::GHOST_WHITE,
    },
    ColourEntry {
        name: "GOLD",
        colour: colors::GOLD,
    },
    ColourEntry {
        name: "GOLDENROD",
        colour: colors::GOLDENROD,
    },
    ColourEntry {
        name: "GRAY",
        colour: colors::GRAY,
    },
    ColourEntry {
        name: "GREEN_YELLOW",
        colour: colors::GREEN_YELLOW,
    },
    ColourEntry {
        name: "GREEN",
        colour: colors::GREEN,
    },
    ColourEntry {
        name: "HONEYDEW",
        colour: colors::HONEYDEW,
    },
    ColourEntry {
        name: "HOT_PINK",
        colour: colors::HOT_PINK,
    },
    ColourEntry {
        name: "INDIAN_RED",
        colour: colors::INDIAN_RED,
    },
    ColourEntry {
        name: "INDIGO",
        colour: colors::INDIGO,
    },
    ColourEntry {
        name: "IVORY",
        colour: colors::IVORY,
    },
    ColourEntry {
        name: "KHAKI",
        colour: colors::KHAKI,
    },
    ColourEntry {
        name: "LAVENDER_BLUSH",
        colour: colors::LAVENDER_BLUSH,
    },
    ColourEntry {
        name: "LAVENDER",
        colour: colors::LAVENDER,
    },
    ColourEntry {
        name: "LAWN_GREEN",
        colour: colors::LAWN_GREEN,
    },
    ColourEntry {
        name: "LEMON_CHIFFON",
        colour: colors::LEMON_CHIFFON,
    },
    ColourEntry {
        name: "LIGHT_BLUE",
        colour: colors::LIGHT_BLUE,
    },
    ColourEntry {
        name: "LIGHT_CORAL",
        colour: colors::LIGHT_CORAL,
    },
    ColourEntry {
        name: "LIGHT_CYAN",
        colour: colors::LIGHT_CYAN,
    },
    ColourEntry {
        name: "LIGHT_GOLDENROD_YELLOW",
        colour: colors::LIGHT_GOLDENROD_YELLOW,
    },
    ColourEntry {
        name: "LIGHT_GRAY",
        colour: colors::LIGHT_GRAY,
    },
    ColourEntry {
        name: "LIGHT_GREEN",
        colour: colors::LIGHT_GREEN,
    },
    ColourEntry {
        name: "LIGHT_PINK",
        colour: colors::LIGHT_PINK,
    },
    ColourEntry {
        name: "LIGHT_SALMON",
        colour: colors::LIGHT_SALMON,
    },
    ColourEntry {
        name: "LIGHT_SEA_GREEN",
        colour: colors::LIGHT_SEA_GREEN,
    },
    ColourEntry {
        name: "LIGHT_SKY_BLUE",
        colour: colors::LIGHT_SKY_BLUE,
    },
    ColourEntry {
        name: "LIGHT_SLATE_GRAY",
        colour: colors::LIGHT_SLATE_GRAY,
    },
    ColourEntry {
        name: "LIGHT_STEEL_BLUE",
        colour: colors::LIGHT_STEEL_BLUE,
    },
    ColourEntry {
        name: "LIGHT_YELLOW",
        colour: colors::LIGHT_YELLOW,
    },
    ColourEntry {
        name: "LIME_GREEN",
        colour: colors::LIME_GREEN,
    },
    ColourEntry {
        name: "LIME",
        colour: colors::LIME,
    },
    ColourEntry {
        name: "LINEN",
        colour: colors::LINEN,
    },
    ColourEntry {
        name: "MAGENTA",
        colour: colors::MAGENTA,
    },
    ColourEntry {
        name: "MAROON",
        colour: colors::MAROON,
    },
    ColourEntry {
        name: "MEDIUM_AQUAMARINE",
        colour: colors::MEDIUM_AQUAMARINE,
    },
    ColourEntry {
        name: "MEDIUM_BLUE",
        colour: colors::MEDIUM_BLUE,
    },
    ColourEntry {
        name: "MEDIUM_ORCHID",
        colour: colors::MEDIUM_ORCHID,
    },
    ColourEntry {
        name: "MEDIUM_PURPLE",
        colour: colors::MEDIUM_PURPLE,
    },
    ColourEntry {
        name: "MEDIUM_SEA_GREEN",
        colour: colors::MEDIUM_SEA_GREEN,
    },
    ColourEntry {
        name: "MEDIUM_SLATE_BLUE",
        colour: colors::MEDIUM_SLATE_BLUE,
    },
    ColourEntry {
        name: "MEDIUM_SPRING_GREEN",
        colour: colors::MEDIUM_SPRING_GREEN,
    },
    ColourEntry {
        name: "MEDIUM_TURQUOISE",
        colour: colors::MEDIUM_TURQUOISE,
    },
    ColourEntry {
        name: "MEDIUM_VIOLET_RED",
        colour: colors::MEDIUM_VIOLET_RED,
    },
    ColourEntry {
        name: "MIDNIGHT_BLUE",
        colour: colors::MIDNIGHT_BLUE,
    },
    ColourEntry {
        name: "MINT_CREAM",
        colour: colors::MINT_CREAM,
    },
    ColourEntry {
        name: "MISTY_ROSE",
        colour: colors::MISTY_ROSE,
    },
    ColourEntry {
        name: "MOCCASIN",
        colour: colors::MOCCASIN,
    },
    ColourEntry {
        name: "NAVAJO_WHITE",
        colour: colors::NAVAJO_WHITE,
    },
    ColourEntry {
        name: "NAVY",
        colour: colors::NAVY,
    },
    ColourEntry {
        name: "OLD_LACE",
        colour: colors::OLD_LACE,
    },
    ColourEntry {
        name: "OLIVE_DRAB",
        colour: colors::OLIVE_DRAB,
    },
    ColourEntry {
        name: "OLIVE",
        colour: colors::OLIVE,
    },
    ColourEntry {
        name: "ORANGE_RED",
        colour: colors::ORANGE_RED,
    },
    ColourEntry {
        name: "ORANGE",
        colour: colors::ORANGE,
    },
    ColourEntry {
        name: "ORCHID",
        colour: colors::ORCHID,
    },
    ColourEntry {
        name: "PALE_GOLDENROD",
        colour: colors::PALE_GOLDENROD,
    },
    ColourEntry {
        name: "PALE_GREEN",
        colour: colors::PALE_GREEN,
    },
    ColourEntry {
        name: "PALE_TURQUOISE",
        colour: colors::PALE_TURQUOISE,
    },
    ColourEntry {
        name: "PALE_VIOLET_RED",
        colour: colors::PALE_VIOLET_RED,
    },
    ColourEntry {
        name: "PAPAYA_WHIP",
        colour: colors::PAPAYA_WHIP,
    },
    ColourEntry {
        name: "PEACH_PUFF",
        colour: colors::PEACH_PUFF,
    },
    ColourEntry {
        name: "PERU",
        colour: colors::PERU,
    },
    ColourEntry {
        name: "PINK",
        colour: colors::PINK,
    },
    ColourEntry {
        name: "PLUM",
        colour: colors::PLUM,
    },
    ColourEntry {
        name: "POWDER_BLUE",
        colour: colors::POWDER_BLUE,
    },
    ColourEntry {
        name: "PURPLE",
        colour: colors::PURPLE,
    },
    ColourEntry {
        name: "RED",
        colour: colors::RED,
    },
    ColourEntry {
        name: "ROSY_BROWN",
        colour: colors::ROSY_BROWN,
    },
    ColourEntry {
        name: "ROYAL_BLUE",
        colour: colors::ROYAL_BLUE,
    },
    ColourEntry {
        name: "SADDLE_BROWN",
        colour: colors::SADDLE_BROWN,
    },
    ColourEntry {
        name: "SALMON",
        colour: colors::SALMON,
    },
    ColourEntry {
        name: "SANDY_BROWN",
        colour: colors::SANDY_BROWN,
    },
    ColourEntry {
        name: "SEA_GREEN",
        colour: colors::SEA_GREEN,
    },
    ColourEntry {
        name: "SEASHELL",
        colour: colors::SEASHELL,
    },
    ColourEntry {
        name: "SIENNA",
        colour: colors::SIENNA,
    },
    ColourEntry {
        name: "SILVER",
        colour: colors::SILVER,
    },
    ColourEntry {
        name: "SKY_BLUE",
        colour: colors::SKY_BLUE,
    },
    ColourEntry {
        name: "SLATE_BLUE",
        colour: colors::SLATE_BLUE,
    },
    ColourEntry {
        name: "SLATE_GRAY",
        colour: colors::SLATE_GRAY,
    },
    ColourEntry {
        name: "SNOW",
        colour: colors::SNOW,
    },
    ColourEntry {
        name: "SPRING_GREEN",
        colour: colors::SPRING_GREEN,
    },
    ColourEntry {
        name: "STEEL_BLUE",
        colour: colors::STEEL_BLUE,
    },
    ColourEntry {
        name: "TAN",
        colour: colors::TAN,
    },
    ColourEntry {
        name: "TEAL",
        colour: colors::TEAL,
    },
    ColourEntry {
        name: "THISTLE",
        colour: colors::THISTLE,
    },
    ColourEntry {
        name: "TOMATO",
        colour: colors::TOMATO,
    },
    ColourEntry {
        name: "TURQUOISE",
        colour: colors::TURQUOISE,
    },
    ColourEntry {
        name: "VIOLET",
        colour: colors::VIOLET,
    },
    ColourEntry {
        name: "WHEAT",
        colour: colors::WHEAT,
    },
    ColourEntry {
        name: "WHITE_SMOKE",
        colour: colors::WHITE_SMOKE,
    },
    ColourEntry {
        name: "WHITE",
        colour: colors::WHITE,
    },
    ColourEntry {
        name: "YELLOW_GREEN",
        colour: colors::YELLOW_GREEN,
    },
    ColourEntry {
        name: "YELLOW",
        colour: colors::YELLOW,
    },
];
