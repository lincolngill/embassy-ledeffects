//! Button pressed tasks and debouncing support.
//!
//! Each Button has its own embassy [pressed_task] just waiting for the level to go low.
//! The task then signals a global static [PRESSED] Signal with the ID of the button that was pressed.
//! The sharing of the Signal means that detecting simulataneous pressing of butttons is not supported.
//!
//! Button presses are debounced.
//!
//! # Example
//!
//! ```rust
//! use defmt::*;
//! use embassy_executor::Spawner;
//! use embassy_rp::gpio::{self, Input, Level, Output};
//! use embassy_ledeffects::{Button, button};
//!
//! #[embassy_executor::main]
//! async fn main(spawner: Spawner) {
//!    let p = embassy_rp::init(Default::default());
//!    spawner.spawn(unwrap!(button::pressed_task(Button::new(
//!        1,
//!        Input::new(p.PIN_14, gpio::Pull::Up),
//!    ))));
//!    let mut btn_id = 0; // No button pressed
//!    if button::PRESSED.signaled() {
//!        btn_id = button::PRESSED.wait().await;
//!    }
//! }
//! ```
use embassy_rp::gpio::{Input, Level};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};

const BTN_DEBOUNCE: Duration = Duration::from_millis(20);
/// The maximim number of button tasks and hence the max button ID.
pub const MAX_BUTTON_ID: u8 = 4;

/// Button ID Signal containing the most recently pressed button ID.
///
/// Updated by any instance of [pressed_task] and designed to be awaited in the `main` task (or a single other task).
pub static PRESSED: Signal<ThreadModeRawMutex, u8> = Signal::new();

/// The Button struct represents the physical button.
///
/// It contains the button ID and GPIO Input pin.
pub struct Button<'a> {
    pub id: u8,
    input: Input<'a>,
}

impl<'a> Button<'a> {
    /// Create a new Button.
    ///
    /// # Arguments
    /// * `id` - The button ID. Must be between 1 and [`MAX_BUTTON_ID`]
    /// * `input` - The GPIO input pin associated with this button. (Recommend using gpio::Pull::Up in the pin config.)
    pub fn new(id: u8, input: Input<'a>) -> Self {
        assert!(
            id > 0 && id <= MAX_BUTTON_ID,
            "Button id {} must be between 1 and {}",
            id,
            MAX_BUTTON_ID
        );
        Self { id, input }
    }

    async fn level_change(&mut self) -> Level {
        loop {
            let l1 = self.input.get_level();

            self.input.wait_for_any_edge().await;

            Timer::after(BTN_DEBOUNCE).await;

            let l2 = self.input.get_level();
            if l1 != l2 {
                break l2;
            }
        }
    }
}

/// A button pressed embassy task.
///
/// Awaits a level change and signals the [PRESSED] button ID when the change is to Level::Low.
/// Level changes are debounced to ensure stable button press capture.
#[embassy_executor::task(pool_size = MAX_BUTTON_ID as usize)]
pub async fn pressed_task(mut btn: Button<'static>) {
    loop {
        if btn.level_change().await == Level::Low {
            PRESSED.signal(btn.id);
        }
    }
}
