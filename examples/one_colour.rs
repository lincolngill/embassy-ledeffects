//! Example OneColour effect
#![no_std]
#![no_main]
use defmt::*;
use embassy_executor::Spawner;
use embassy_ledeffects::Strip;
use embassy_ledeffects::effect::{self, COLOURS, EffectIterator};
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
        for c in COLOURS {
            onecolour_effect.set(c.colour);
            info!("Colour: {}", c.name);
            onecolour_effect
                .nextframe(&mut strip)
                .expect("Failed to generate next OneColour LED frame");
            ws2812.write(&strip.leds).await;
            Timer::after(Duration::from_millis(COLOUR_CHANGE_MS)).await;
        }
    }
}
