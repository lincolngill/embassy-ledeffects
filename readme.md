# Embassy Pico 2 WS2812 LED Strip Effects

LED effects library and binary examples. For Raspberry Pico 2. Using Embassy embeded framework.

## Example

```rust
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
    let mut effect = effect::Random::<NUM_LEDS>::new(&strip);
    loop {
        effect.nextframe(&mut strip).unwrap();
        ws2812.write(&strip.leds).await;
        Timer::after(strip.frame_delay()).await;
    }
}
```

## Effects
* Random
* Wheel
* OneColour
* _More to come_

Refer to effect_button.rs binary for example of all effects.

### Random Effect

Random colour change at random times.

Per pixel random period (500 - 2540 ms) between colour change.

### Wheel Effect

Rotate each pixel through shades of red, green and blue.

Each pixel offset on wheel position.

### OneColour

All pixels set to a single colour. Can alternate between Black and White to turn off and on entire strip.

## Binary Crates
| Example | Description |
|---------|-------------|
| random.rs | Just the random effect. |
| effect_button.rs | Rotates through all effects and second button adjusts an attribute of the effect. |