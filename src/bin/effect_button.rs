/*
Main task:
    Applies a single frame effect to the LED strip.
    Writes the strip via PIO.
    Sleeps for a short delay.
    If the button has been pressed rotate the effect that is being aopplied.
    loops back to do the next frame.

frame rate task - strip::frame_rate_task
    Adjusts the main loop sleep time to achieve the target FPS.
    Checks every FPS_ADJUST_SECS (5)
    Debug outputs:
        Current FPS.
        Current main loop delay (ms).
        Calculated (FPS - FPS_TARGET) difference.
        New delay calculation.
        Tolerance = FPS varaince observed when the new delay calc == current delay.
            Delay is not adjusted again unless subsequent FPS variance exceeds tolerance.
            Tolerance should be <= 3 FPS but will be > 0 when the FPS target is high and the delay is low.
        Total frame count.

Button task (GPIO 15 - Input with pull down):
    Waits for button press. (falling edge. I.e. button released.)
    Signals main task to rotate the effect.

On Board LED toggle task:
    Toggles Pin 25 - Sign of life.

Effects:
    Random - Random colour change at random times.
        Per pixel random period (500 - 2540 ms) between colour change.
        Achieves 62 FPS, with a 12ms main loop sleep delay, for a 120 LED strip.
            Uses ~10.5W = ~2.0A x 5V
    Wheel - Colour wheel effect.
    Off - All LEDs off.
 */
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_ledeffects::{
    Button, Strip,
    effect::{self, EffectIterator},
};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{self, Input, Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

const NUM_LEDS: usize = 120;
const FPS_TARGET: u32 = 60;
const FPS_ADJUST_SECS: u32 = 5;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH0>;
});

static BTN_PRESSED: Signal<ThreadModeRawMutex, ()> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Start - Off");
    let p = embassy_rp::init(Default::default());

    let btn = Button::new(
        Input::new(p.PIN_15, gpio::Pull::Down),
        Duration::from_millis(20),
    );

    spawner.spawn(unwrap!(toggle_led(Output::new(p.PIN_25, Level::Low))));
    spawner.spawn(unwrap!(embassy_ledeffects::strip::frame_rate_task(
        FPS_ADJUST_SECS,
        FPS_TARGET
    )));
    spawner.spawn(unwrap!(button_task(btn)));

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_16, &program);

    let mut strip = Strip::<NUM_LEDS>::new();

    enum Effect {
        Random,
        Wheel,
        Off,
    }

    let mut random_effect = effect::Random::<NUM_LEDS>::new(&strip);
    let mut wheel_effect = effect::Wheel::new();
    let mut off_effect = effect::Off::new();
    let mut effect = Effect::Off;
    loop {
        match effect {
            Effect::Random => random_effect.nextframe(&mut strip).unwrap(),
            Effect::Wheel => wheel_effect.nextframe(&mut strip).unwrap(),
            Effect::Off => off_effect.nextframe(&mut strip).unwrap(),
        }
        ws2812.write(&strip.leds).await;
        Timer::after(strip.frame_delay()).await;
        if BTN_PRESSED.signaled() {
            BTN_PRESSED.wait().await;
            match effect {
                Effect::Random => {
                    effect = Effect::Wheel;
                    debug!("Effect = Wheel");
                }
                Effect::Wheel => {
                    effect = Effect::Off;
                    debug!("Effect = Off");
                }
                Effect::Off => {
                    effect = Effect::Random;
                    debug!("Effect = Random");
                }
            }
        }
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

#[embassy_executor::task]
async fn button_task(mut btn: Button<'static>) {
    loop {
        btn.falling_edge().await;
        BTN_PRESSED.signal(());
    }
}
