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
    Comets - Ping up and down the strip.
        Button 2 - Launch another comet. Random direction. Random TTL.
 */
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_ledeffects::{
    Button, Strip,
    effect::{self, EffectIterator, comets},
    strip,
};
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{self, Input, Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use smart_leds::colors;
use {defmt_rtt as _, panic_probe as _};

// 8 x 32 2D LED panel
const NUM_LEDS: usize = 120;
const SEGMENT_LENGTH: usize = 8;
const SEGMENT_LAYOUT: strip::Layout = strip::Layout::ZigZag;
const FPS_TARGET: i32 = 60;
const FPS_ADJUST_SECS: i32 = 5;

// 8x32 grid. Horizontal strip segments
const HFIREGRID_COLS: usize = 8;
const HFIREGRID_ROWS: usize = NUM_LEDS / HFIREGRID_COLS;
// 32x8 grid. Vertical strip segments
const VFIREGRID_COLS: usize = 32;
const VFIREGRID_ROWS: usize = NUM_LEDS / VFIREGRID_COLS;

// Fire grids that don't use all the panel.
// 4x16 grid - Half the rows
const H2FIREGRID_COLS: usize = 4;
const H2FIREGRID_ROWS: usize = 16;

// 16x6 grid - Half the cols
const V2FIREGRID_COLS: usize = 16;
const V2FIREGRID_ROWS: usize = 6;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH0>;
});

static BTN_PRESSED: Signal<ThreadModeRawMutex, u8> = Signal::new();

#[derive(Default)]
enum EffectState {
    Random,
    Wheel,
    #[default]
    OneColour,
    Comets,
    HFireGrid,
    VFireGrid,
    H2FireGrid,
    V2FireGrid,
    Fire,
}
impl defmt::Format for EffectState {
    fn format(&self, fmt: Formatter) {
        match self {
            EffectState::Random => defmt::write!(fmt, "Random"),
            EffectState::Wheel => defmt::write!(fmt, "Wheel"),
            EffectState::OneColour => defmt::write!(fmt, "OneColour"),
            EffectState::Comets => defmt::write!(fmt, "Comets"),
            EffectState::HFireGrid => defmt::write!(fmt, "HFireGrid"),
            EffectState::VFireGrid => defmt::write!(fmt, "VFireGrid"),
            EffectState::H2FireGrid => defmt::write!(fmt, "H2FireGrid"),
            EffectState::V2FireGrid => defmt::write!(fmt, "V2FireGrid"),
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

    let mut strip = Strip::<NUM_LEDS>::new(Some(SEGMENT_LENGTH), Some(SEGMENT_LAYOUT));

    let mut random_effect = effect::Random::<NUM_LEDS>::new(&strip, None);
    let mut wheel_effect = effect::Wheel::new(None);
    let mut onecolour_effect = effect::OneColour::new(colors::BLACK);
    let mut comets_effect = effect::Comets::new(&strip);
    let mut h_firegrid_effect = effect::FireGrid::<HFIREGRID_COLS, HFIREGRID_ROWS>::new(
        &strip,
        None,
        None,
        effect::GridDirection::Horizontal,
    );
    let mut v_firegrid_effect = effect::FireGrid::<VFIREGRID_COLS, VFIREGRID_ROWS>::new(
        &strip,
        None,
        None,
        effect::GridDirection::Vertical,
    );
    let mut h2_firegrid_effect = effect::FireGrid::<H2FIREGRID_COLS, H2FIREGRID_ROWS>::new(
        &strip,
        None,
        None,
        effect::GridDirection::Horizontal,
    );
    let mut v2_firegrid_effect = effect::FireGrid::<V2FIREGRID_COLS, V2FIREGRID_ROWS>::new(
        &strip,
        None,
        None,
        effect::GridDirection::Vertical,
    );
    let mut fire_effect = effect::Fire::<NUM_LEDS>::new(&strip, None, None);
    let mut effect = EffectState::default();
    let mut btn_id: u8;
    loop {
        btn_id = 0; // No button pressed
        if BTN_PRESSED.signaled() {
            btn_id = BTN_PRESSED.wait().await;
        }
        // State machine for EffectState
        match effect {
            EffectState::Random => {
                random_effect.nextframe(&mut strip).unwrap();
                if btn_id == 1 {
                    effect = EffectState::Wheel;
                }
                if btn_id == 2 {
                    debug!("Random delay_factor: {}", random_effect.slow_down());
                }
            }
            EffectState::Wheel => {
                wheel_effect.nextframe(&mut strip).unwrap();
                if btn_id == 1 {
                    effect = EffectState::OneColour;
                }
                if btn_id == 2 {
                    debug!("Wheel speed: {}", wheel_effect.speedup());
                }
            }
            EffectState::OneColour => {
                onecolour_effect.nextframe(&mut strip).unwrap();
                if btn_id == 1 {
                    effect = EffectState::Comets;
                    spawner.spawn(unwrap!(comets::comets_task(None, None)));
                }
                if btn_id == 2 {
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
            }
            EffectState::Comets => {
                comets_effect.nextframe(&mut strip).unwrap();
                if btn_id == 1 {
                    comets::COMETS_IN_MSG.signal(comets::CometsInMsg::Stop);
                    // delay effect change debug message till the TaskEnded signal
                    btn_id = 0;
                }
                let mut launch_signal: bool = false;
                if comets::COMETS_OUT_MSG.signaled() {
                    match comets::COMETS_OUT_MSG.wait().await {
                        comets::CometsOutMsg::Launch => launch_signal = true,
                        comets::CometsOutMsg::TaskEnded => {
                            debug!("Comets task ended");
                            effect = EffectState::HFireGrid;
                            btn_id = 1; // Trigger effect change debug message
                        }
                    }
                }
                if btn_id == 2 || launch_signal {
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
                        Err(_) => warn!(
                            "Failed to launch. Too many inflight comets {}.",
                            comets_effect.comet_cnt()
                        ),
                    };
                }
            }
            EffectState::HFireGrid => {
                h_firegrid_effect.nextframe(&mut strip).unwrap();
                if btn_id == 1 {
                    effect = EffectState::VFireGrid;
                }
                if btn_id == 2 {
                    let mut cooling = h_firegrid_effect.inc_cooling(8);
                    if cooling > 80 {
                        cooling = h_firegrid_effect.set_cooling(None);
                    }
                    debug!("HFireGrid cooling: {}", cooling);
                }
            }
            EffectState::VFireGrid => {
                v_firegrid_effect.nextframe(&mut strip).unwrap();
                if btn_id == 1 {
                    effect = EffectState::H2FireGrid;
                }
                if btn_id == 2 {
                    let mut cooling = v_firegrid_effect.inc_cooling(8);
                    if cooling > 124 {
                        cooling = v_firegrid_effect.set_cooling(None);
                    }
                    debug!("VFireGrid cooling: {}", cooling);
                }
            }
            EffectState::H2FireGrid => {
                h2_firegrid_effect.nextframe(&mut strip).unwrap();
                if btn_id == 1 {
                    effect = EffectState::V2FireGrid;
                }
                if btn_id == 2 {
                    debug!("btn2 {}", effect);
                }
            }
            EffectState::V2FireGrid => {
                v2_firegrid_effect.nextframe(&mut strip).unwrap();
                if btn_id == 1 {
                    effect = EffectState::Fire;
                }
                if btn_id == 2 {
                    debug!("btn2 {}", effect);
                }
            }
            EffectState::Fire => {
                fire_effect.nextframe(&mut strip).unwrap();
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

#[embassy_executor::task(pool_size = 2)]
async fn button_task(mut btn: Button<'static>) {
    loop {
        if btn.level_change().await == Level::Low {
            BTN_PRESSED.signal(btn.id);
        }
    }
}
