//! All LED strip strip effects.
//!
//! * Button 1 - Change the effect.
//! * Button 2 - Change an attribute of the current effect.
//!
//!
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
        Current FPS (FPSc).
        Delta FPS (Dfps) from target FPS (FPSt).
            Dfps = FPSt - FPSc.
        Delta Delay calculation (Dt).
            Dt (ms) = 1000/FPSt - 1000/FPSc.
        Current Delay setting (Tc) (ms). Minimum = 2 ms.
        New delay setting (Tn). Or "(No Change)"
            Tn (ms) = Tc +  Dt
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
        Button 2 - Toggle to some random colour.
    Fire - Single strip of fire effect.
    Comets - Ping up and down the strip.
        The comets_task randomly sends launch signals based on a min and max delay period.
        Button 2 - Launch another comet. Random direction. Random TTL.
 */
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_ledeffects::effect::{self, COLOURS, EffectIterator, comets};
use embassy_ledeffects::{Button, button};
use embassy_ledeffects::{Strip, strip};
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{self, Input, Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_time::Timer;
use smart_leds::colors;
use {defmt_rtt as _, panic_probe as _};

const NUM_LEDS: usize = 144;
const FPS_TARGET: i32 = 60;
const FPS_ADJUST_SECS: i32 = 5;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH0>;
});

#[derive(Default)]
enum EffectState {
    Random,
    Wheel,
    #[default]
    OneColour,
    Comets,
    Fire,
}
impl defmt::Format for EffectState {
    fn format(&self, fmt: Formatter) {
        match self {
            EffectState::Random => defmt::write!(fmt, "Random"),
            EffectState::Wheel => defmt::write!(fmt, "Wheel"),
            EffectState::OneColour => defmt::write!(fmt, "OneColour"),
            EffectState::Comets => defmt::write!(fmt, "Comets"),
            EffectState::Fire => defmt::write!(fmt, "Fire"),
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Start - OneColour Black");
    let p = embassy_rp::init(Default::default());

    spawner.spawn(unwrap!(toggle_led(Output::new(p.PIN_25, Level::Low))));
    spawner.spawn(unwrap!(strip::frame_rate_task(FPS_ADJUST_SECS, FPS_TARGET)));
    spawner.spawn(unwrap!(button::pressed_task(Button::new(
        1,
        Input::new(p.PIN_14, gpio::Pull::Up),
    ))));
    spawner.spawn(unwrap!(button::pressed_task(Button::new(
        2,
        Input::new(p.PIN_15, gpio::Pull::Up),
    ))));

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_16, &program);

    let mut strip = Strip::<NUM_LEDS>::new(None, None);

    let mut random_effect = effect::Random::<NUM_LEDS>::new(&strip, None);
    let mut wheel_effect = effect::Wheel::new(None);
    let mut onecolour_effect = effect::OneColour::new(colors::BLACK);
    let mut comets_effect = effect::Comets::new(&strip);
    let mut fire_effect = effect::Fire::<NUM_LEDS>::new(&strip, None, None);
    let mut effect = EffectState::default();
    let mut btn_id: u8;
    loop {
        btn_id = 0; // No button pressed
        if button::PRESSED.signaled() {
            btn_id = button::PRESSED.wait().await;
        }
        // State machine for EffectState
        match effect {
            EffectState::Random => {
                random_effect
                    .nextframe(&mut strip)
                    .expect("Failed to generate next Random LED frame");
                if btn_id == 1 {
                    effect = EffectState::Wheel;
                }
                if btn_id == 2 {
                    debug!("Random delay_factor: {}", random_effect.slow_down());
                }
            }
            EffectState::Wheel => {
                wheel_effect
                    .nextframe(&mut strip)
                    .expect("Failed to generate next Wheel LED frame");
                if btn_id == 1 {
                    effect = EffectState::OneColour;
                    onecolour_effect.refresh();
                }
                if btn_id == 2 {
                    debug!("Wheel speed: {}", wheel_effect.speedup());
                }
            }
            EffectState::OneColour => {
                onecolour_effect
                    .nextframe(&mut strip)
                    .expect("Failed to generate next OneColour LED frame");
                if btn_id == 1 {
                    effect = EffectState::Comets;
                    spawner.spawn(unwrap!(comets::launcher_task(None, None)));
                }
                if btn_id == 2 {
                    if onecolour_effect.get() == colors::BLACK {
                        let rn = RoscRng.next_u32();
                        let ci = rn as usize % COLOURS.len();
                        onecolour_effect.set(COLOURS[ci].colour);
                        debug!("OneColour {}", COLOURS[ci].name);
                    } else {
                        onecolour_effect.set(colors::BLACK);
                        debug!("OneColour BLACK");
                    }
                }
            }
            EffectState::Comets => {
                comets_effect
                    .nextframe(&mut strip)
                    .expect("Failed to generate next Comets LED frame");
                if btn_id == 1 {
                    match comets::stop_launcher_task().await {
                        Ok(()) => debug!("Comets launcher task ended"),
                        Err(e) => error!("Failed to stop launcher task: {}", e),
                    }
                    effect = EffectState::Fire;
                }
                if btn_id == 2 || comets::launch_signaled().await {
                    let ttl_pings = RoscRng.next_u32() as u8 % 3;
                    let direction: effect::CometDirection;
                    if RoscRng.next_u32() % 2 == 0 {
                        direction = effect::CometDirection::Up;
                    } else {
                        direction = effect::CometDirection::Down;
                    }
                    match comets_effect.launch(Some(direction), Some(ttl_pings)) {
                        Ok(_) => info!(
                            "Fire in the hole. TTL_pings: {} Dir: {} Comets: {}",
                            ttl_pings,
                            direction,
                            comets_effect.comet_cnt()
                        ),
                        Err(e) => warn!("Failed to launch: {}", e),
                    };
                }
            }
            EffectState::Fire => {
                fire_effect
                    .nextframe(&mut strip)
                    .expect("Failed to generate next Fire LED frame");
                if btn_id == 1 {
                    effect = EffectState::Random;
                }
                if btn_id == 2 {
                    debug!("btn2 {}", effect);
                }
            }
        }
        ws2812.write(&strip.leds).await;
        Timer::after(strip.frame_delay()).await;
        if btn_id == 1 {
            // New EffectState
            info!("EffectState: {}", effect);
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
