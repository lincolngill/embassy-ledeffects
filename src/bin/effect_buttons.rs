/*
Main task:
    Applies a single frame effect to the LED strip.
    Writes the strip via PIO.
    Sleeps for a short delay.
    If button 1 has been pressed rotate the effect that is being applied.
    If button 2 has been pressed alter the current effect. Refer below.
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

Button 1 task (GPIO 14 - Input with pull up):
    Waits for button press. (Low input)
    Signals main task to rotate the effect.

Button 2 task (GPIO 15 - Input with pull up):
    Waits for button press. (Low input)
    Signals main task to alter attribute of current effect.

On Board LED toggle task:
    Toggles Pin 25 - Sign of life.

Effects:
    Random - Random colour change at random times.
        Per pixel random period (500 - 2540 ms) between colour change.
        Achieves 62 FPS, with a 12ms main loop sleep delay, for a 120 LED strip.
            Uses ~10.5W = ~2.0A x 5V
        Button 2 - Increases the random delay_factor
    Wheel - Colour wheel effect.
        Button 2 - Speeds up the effect
    OneColour - All LEDs Black (Off).
        Button 2 - Changes colour to White (On)
    FireGrid - Fire effect in columns and rows
        Can be vertical or horizontal.
    Fire
 */
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_ledeffects::{
    Button, Strip,
    effect::{self, Effect, EffectIterator},
    strip::frame_rate_task,
};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{self, Input, Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use smart_leds::colors;
use {defmt_rtt as _, panic_probe as _};

const NUM_LEDS: usize = 256;
const FPS_TARGET: u32 = 30;
const FPS_ADJUST_SECS: u32 = 5;

const HFIREGRID_COLS: usize = 8;
const HFIREGRID_ROWS: usize = NUM_LEDS / HFIREGRID_COLS;
const VFIREGRID_COLS: usize = 32;
const VFIREGRID_ROWS: usize = NUM_LEDS / VFIREGRID_COLS;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH0>;
});

static BTN_PRESSED: Signal<ThreadModeRawMutex, u8> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Start - OneColour Black");
    let p = embassy_rp::init(Default::default());

    spawner.spawn(unwrap!(toggle_led(Output::new(p.PIN_25, Level::Low))));
    spawner.spawn(unwrap!(frame_rate_task(FPS_ADJUST_SECS, FPS_TARGET)));
    spawner.spawn(unwrap!(button_task(Button::new(
        1,
        Input::new(p.PIN_14, gpio::Pull::Up),
    ))));
    spawner.spawn(unwrap!(button_task(Button::new(
        2,
        Input::new(p.PIN_15, gpio::Pull::Up),
    ))));

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_16, &program);

    let mut strip = Strip::<NUM_LEDS>::new();

    let mut random_effect = effect::Random::<NUM_LEDS>::new(&strip, None);
    let mut wheel_effect = effect::Wheel::new(None);
    let mut onecolour_effect = effect::OneColour::new(colors::BLACK);
    let mut h_firegrid_effect = effect::FireGrid::<HFIREGRID_COLS, HFIREGRID_ROWS>::new(
        &strip,
        None,
        None,
        effect::StripDirection::Horizontal,
    );
    let mut v_firegrid_effect = effect::FireGrid::<VFIREGRID_COLS, VFIREGRID_ROWS>::new(
        &strip,
        None,
        None,
        effect::StripDirection::Vertical,
    );
    let mut fire_effect = effect::Fire::<NUM_LEDS>::new(&strip, None, None);
    let mut effect = Effect::OneColour;
    loop {
        match effect {
            Effect::Random => random_effect.nextframe(&mut strip).unwrap(),
            Effect::Wheel => wheel_effect.nextframe(&mut strip).unwrap(),
            Effect::OneColour => onecolour_effect.nextframe(&mut strip).unwrap(),
            Effect::HFireGrid => h_firegrid_effect.nextframe(&mut strip).unwrap(),
            Effect::VFireGrid => v_firegrid_effect.nextframe(&mut strip).unwrap(),
            Effect::Fire => fire_effect.nextframe(&mut strip).unwrap(),
        }
        ws2812.write(&strip.leds).await;
        Timer::after(strip.frame_delay()).await;
        if BTN_PRESSED.signaled() {
            let btn_id = BTN_PRESSED.wait().await;
            if btn_id == 1 {
                // Next effect
                match effect {
                    Effect::Random => {
                        effect = Effect::Wheel;
                    }
                    Effect::Wheel => {
                        effect = Effect::OneColour;
                    }
                    Effect::OneColour => {
                        effect = Effect::HFireGrid;
                    }
                    Effect::HFireGrid => {
                        effect = Effect::VFireGrid;
                    }
                    Effect::VFireGrid => {
                        effect = Effect::Fire;
                    }
                    Effect::Fire => {
                        effect = Effect::Random;
                    }
                }
                debug!("Effect: {}", effect);
            }
            if btn_id == 2 {
                // Change current effect
                match effect {
                    Effect::Random => {
                        debug!("Random delay_factor: {}", random_effect.slow_down());
                    }
                    Effect::Wheel => {
                        debug!("Wheel speed: {}", wheel_effect.speedup());
                    }
                    Effect::OneColour => {
                        if onecolour_effect.colour == colors::BLACK {
                            onecolour_effect.colour = colors::WHITE;
                        } else {
                            onecolour_effect.colour = colors::BLACK;
                        }
                        debug!(
                            "OneColour {} {} {}",
                            onecolour_effect.colour.r,
                            onecolour_effect.colour.g,
                            onecolour_effect.colour.b
                        );
                    }
                    Effect::HFireGrid => {
                        let mut cooling = h_firegrid_effect.inc_cooling(8);
                        if cooling > 80 {
                            cooling = h_firegrid_effect.set_cooling(None);
                        }
                        debug!("HFireGrid cooling: {}", cooling);
                    }
                    Effect::VFireGrid => {
                        let mut cooling = v_firegrid_effect.inc_cooling(8);
                        if cooling > 124 {
                            cooling = v_firegrid_effect.set_cooling(None);
                        }
                        debug!("VFireGrid cooling: {}", cooling);
                    }
                    _ => {
                        debug!("btn2 {}", effect);
                    }
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
        Timer::after_millis(1500).await;
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn button_task(mut btn: Button<'static>) {
    loop {
        if btn.level_change().await == Level::Low {
            BTN_PRESSED.signal(btn.id);
        }
    }
}
