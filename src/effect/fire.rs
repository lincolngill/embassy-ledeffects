//! Fire effect.
//!
//! The fire effect has a u8 heat value for each pixel in the [crate:Strip].
//! Each frame of the effect goes through the following steps:
//! 1. Cool down each heat value by a random amount, up to the cooling [Fire::cooling] value.
//! 2. Shifts heat values up the [crate::Strip], by averaging each sequence of 3 pixels.
//! 3. Randomly ignites new sparks near the bottom of the [crate::Strip] with a chance based on the sparking [Fire::sparking] value.
//! 4. Maps the heat value to a colour gradient.
use crate::Strip;
use crate::effect::EffectIterator;
use embassy_rp::clocks::RoscRng;
use smart_leds::RGB8;

/// The Fire struct holds a heat value for each LED and the heating and cooling parameters used to generate the [Fire::nextframe].
pub struct Fire<const N: usize> {
    pub cooling: u8,
    pub sparking: u8,
    heat: [u8; N],
}

impl<const N: usize> Fire<N> {
    /// Creates a new Fire effect with the given cooling and sparking values. The size of the Fire effect heat array is determined by the generic parameter N, which must match the size of the [crate::Strip] it is used with.
    ///
    /// # Arguments
    /// * `strip` - A reference to the [crate::Strip] that the effect uses to check the size of N matches S.
    /// * `cooling` - Higher values will cool down the heat values faster, resulting in a shorter flame. Default is 40.
    /// * `sparking` - Higher values will increase the chance of new sparks being ignited, resulting in a more intense flame. Default is 100.
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
    /// Generates the next frame of the Fire effect by updating the heat values and mapping them to colours on the [crate::Strip].
    fn nextframe<const S: usize>(&mut self, strip: &mut Strip<S>) -> Option<()> {
        update_heat(&mut self.heat, self.cooling, self.sparking);
        for i in 0..N {
            strip.leds[i] = colour(self.heat[i]);
        }
        strip.inc_frame_cnt();
        Some(())
    }
}

/// Helper function to calculate the cooling value based on the height of the strip. This is used to make sure the effect looks good on different sized strips.
pub fn cooling_val(cooling: f32, height: f32) -> u8 {
    let mut c = (((cooling as f32 * 10.0) / height) + 2.0) as u8;
    if height < 14.0 {
        c = c.saturating_mul(2);
    }
    c
}

/// Helper function to map a heat value to a colour. The colour gradient goes from black (0) to red (128) to yellow (133) to white (255).
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

/// Helper function to update the heat values for each frame of the effect. This is where the main logic of the fire effect is implemented.
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
