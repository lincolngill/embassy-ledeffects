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
        const DEF_SPARKING: u8 = 120;
        // Use size of Strip to make sure Fire is the same size.
        if N != S {
            panic!("Fire<{}> must be the same size as Strip<{}>", N, S);
        }
        Self {
            cooling: (((cooling.unwrap_or(DEF_COOLING) as f32 * 10.0) / N as f32) + 2.0) as u8,
            sparking: sparking.unwrap_or(DEF_SPARKING),
            heat: [0; N],
        }
    }
}

impl<const N: usize> EffectIterator for Fire<N> {
    fn nextframe<const S: usize>(&mut self, strip: &mut Strip<S>) -> Option<()> {
        update_heat(&mut self.heat, self.cooling, self.sparking);
        /*
        /* Cooling */
        for spark in self.heat.iter_mut() {
            let rn = (self.rng.next_u32() % self.cooling as u32) as u8;
            *spark = spark.saturating_sub(rn)
        }
        /* Heating */
        for i in (2..self.heat.len()).rev() {
            self.heat[i] = (self.heat[i - 1]
                .saturating_add(self.heat[i - 2])
                .saturating_add(self.heat[i - 2]))
                / 3;
        }
        /* Sparks */
        let rn = (self.rng.next_u32() % 255) as u8;
        if rn < self.sparking {
            let i = self.rng.next_u32() as usize % (N / 7);
            let extra = (160 + (self.rng.next_u32() % (255 - 160))) as u8;
            self.heat[i] = self.heat[i].saturating_add(extra);
        }
        */
        for i in 0..N {
            strip.leds[i] = colour(self.heat[i]);
        }
        strip.inc_frame_cnt();
        Some(())
    }
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
        let i = RoscRng.next_u32() as usize % (heat_array.len() / 5);
        let extra = (160 + (RoscRng.next_u32() % (255 - 160))) as u8;
        heat_array[i] = heat_array[i].saturating_add(extra);
    }
}
