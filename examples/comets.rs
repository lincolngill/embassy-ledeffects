//! Example Comets effect
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_ledeffects::effect::{self, EffectIterator, comets};
use embassy_ledeffects::{Strip, strip};
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const NUM_LEDS: usize = 144;
const FPS_TARGET: i32 = 60;
const FPS_ADJUST_SECS: i32 = 5;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Comets");
    let p = embassy_rp::init(Default::default());
    spawner.spawn(unwrap!(strip::frame_rate_task(FPS_ADJUST_SECS, FPS_TARGET)));
    spawner.spawn(unwrap!(comets::launcher_task(Some(100), Some(5000))));

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_16, &program);

    let mut strip = Strip::<NUM_LEDS>::new(None, None);
    let mut comets_effect = effect::Comets::new(&strip);
    loop {
        comets_effect.nextframe(&mut strip).unwrap();
        if comets::launch_signaled().await {
            match comets_effect.launch(Some(comets::CometDirection::Down), Some(1)) {
                Ok(_) => info!("Fire in the hole. Comets: {}", comets_effect.comet_cnt()),
                Err(e) => warn!("Failed to launch: {}", e),
            };
        }
        ws2812.write(&strip.leds).await;
        Timer::after(strip.frame_delay()).await;
    }
}
