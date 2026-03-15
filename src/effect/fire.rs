use crate::Strip;
use crate::effect::EffectIterator;
use embassy_rp::clocks::RoscRng;
use smart_leds::RGB8;

pub struct Fire<const N: usize> {
    pub cooling: u8,
    pub sparking: u8,
    heat: [u8; N],
}

impl<const N: usize> Fire<N> {
    pub fn new<const S: usize>(_: &Strip<S>, cooling: Option<u8>, sparking: Option<u8>) -> Self {
        const DEF_COOLING: u8 = 40;
        const DEF_SPARKING: u8 = 100;
        // Use size of Strip to make sure Fire is the same size.
        if N != S {
            panic!("Fire<{}> must be the same size as Strip<{}>", N, S);
        }
        Self {
            cooling: cooling_val(cooling.unwrap_or(DEF_COOLING) as f32, N as f32),
            sparking: sparking.unwrap_or(DEF_SPARKING),
            heat: [0; N],
        }
    }
}

impl<const N: usize> EffectIterator for Fire<N> {
    fn nextframe<const S: usize>(&mut self, strip: &mut Strip<S>) -> Option<()> {
        update_heat(&mut self.heat, self.cooling, self.sparking);
        for i in 0..N {
            strip.leds[i] = colour(self.heat[i]);
        }
        strip.inc_frame_cnt();
        Some(())
    }
}

pub fn cooling_val(cooling: f32, height: f32) -> u8 {
    let mut c = (((cooling as f32 * 10.0) / height) + 2.0) as u8;
    if height < 14.0 {
        c *= 2;
    }
    c
}

pub fn colour(heat: u8) -> RGB8 {
    if heat >= 0x85 {
        let heat_ramp = 3u8.saturating_mul(heat - 0x85);
        (255, 255, heat_ramp).into()
    } else if heat >= 0x40 {
        let heat_ramp = 3u8.saturating_mul(heat - 0x40);
        (255, heat_ramp, 0).into()
    } else {
        let heat_ramp = 3u8.saturating_mul(heat);
        (heat_ramp, 0, 0).into()
    }
}

pub fn update_heat(heat_array: &mut [u8], cooling: u8, sparking: u8) {
    /* Cooling */
    for spark in heat_array.iter_mut() {
        let rn = (RoscRng.next_u32() % cooling as u32) as u8;
        *spark = spark.saturating_sub(rn)
    }
    /* Heating */
    for i in (2..heat_array.len()).rev() {
        heat_array[i] = (heat_array[i - 1]
            .saturating_add(heat_array[i - 2])
            .saturating_add(heat_array[i - 2]))
            / 3;
    }
    /* Sparks */
    let rn = (RoscRng.next_u32() % 255) as u8;
    if rn < sparking {
        let i = RoscRng.next_u32() as usize % (heat_array.len() / 7).max(2); // Make sure it's at least 2
        let extra = (160 + (RoscRng.next_u32() % (255 - 160))) as u8;
        heat_array[i] = heat_array[i].saturating_add(extra);
    }
}
