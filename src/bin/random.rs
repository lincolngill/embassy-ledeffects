/*
Main task:
    Applies a single frame effect to the LED strip.
    Writes the strip via PIO.
    Sleeps for a short delay.
    loops back to do the next frame.

frame rate task - strip::frame_rate_task
    Adjusts the main loop sleep time to achieve the target FPS.
    Checks every FPS_REFRESH_SECS (5)
    Debug outputs:
        Current FPS.
        Current main loop delay (ms).
        Calculated (FPS - TARGET_FPS) difference.
        New delay calculation.
        Total frame count.

On Board LED toggle task:
    Toggles Pin 25 - Sign of life.

Effects:
    Random - Random colour change at random times.
        Per pixel random period (500 - 2540 ms) between colour change.
        Achieves 62 FPS, with a 12ms main loop sleep delay, for a 120 LED strip.
            Uses ~10.5W = ~2.0A x 5V
 */
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_ledeffects::{
    Strip,
    effect::{self, EffectIterator},
};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const NUM_LEDS: usize = 120;
const FPS_TARGET: u32 = 60;
const FPS_ADJUST_SECS: u32 = 5;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Start");
    let p = embassy_rp::init(Default::default());

    spawner.spawn(unwrap!(toggle_led(Output::new(p.PIN_25, Level::Low))));
    spawner.spawn(unwrap!(embassy_ledeffects::strip::frame_rate_task(
        FPS_ADJUST_SECS,
        FPS_TARGET
    )));

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_16, &program);

    let mut strip = Strip::<NUM_LEDS>::new(FPS_TARGET);
    let mut effect = effect::Random::<NUM_LEDS>::new();
    loop {
        effect.nextframe(&mut strip).unwrap();
        ws2812.write(&strip.leds).await;
        Timer::after(strip.frame_delay()).await;
    }
}

#[embassy_executor::task]
async fn toggle_led(mut led: Output<'static>) {
    loop {
        //debug!("led on!");
        led.set_high();
        Timer::after_millis(500).await;

        //debug!("led off!");
        led.set_low();
        Timer::after_millis(2000).await;
    }
}
