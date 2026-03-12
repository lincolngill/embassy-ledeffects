use embassy_rp::gpio::{Input, Level};
use embassy_time::{Duration, Timer};

const BTN_DEBOUNCE: Duration = Duration::from_millis(20);

pub struct Button<'a> {
    pub id: u8,
    input: Input<'a>,
    pub debounce: Duration,
}

impl<'a> Button<'a> {
    pub fn new(id: u8, input: Input<'a>) -> Self {
        Self {
            id,
            input,
            debounce: BTN_DEBOUNCE,
        }
    }

    pub async fn level_change(&mut self) -> Level {
        loop {
            let l1 = self.input.get_level();

            self.input.wait_for_any_edge().await;

            Timer::after(self.debounce).await;

            let l2 = self.input.get_level();
            if l1 != l2 {
                break l2;
            }
        }
    }
}
