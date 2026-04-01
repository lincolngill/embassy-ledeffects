# Embassy Pico 2 WS2812 LED Strip & 2D Panel Effects

An LED effects library with examples, for Raspberry Pico 2. Using the [Embassy](https://embassy.dev/) embedded framework.

## Effects

| Effect | Description |
|--------|-------------|
| Random | Random colour changes at random times. | Increase random delay per pixel. |
| Wheel | Rotate each pixel through shades of red, green and blue. | Speed up rotation |
| OneColour | All pixels set to a single colour. | Alternate between Off (Black) & On (White). |
| FireGrid | Fire effect over a 2D grid. Can be Horizointal or Vertical | Increases cooling. |
| Fire | Simulated fire. | Increases cooling. |
| Comets | Ping up and down the strip | Launch a comet. Random direction and random TTL. |

_More to come_

## 2D LED Panel

A 2D panel of LEDs is supported by treating it as a single strip with a segment length (and layout).

E.g. a 5 col x 3 row LED strip, with ZipZag layout would be a:
* 15 LED strip, with;
* Segment legnth of 5, and;
* LEDs numbered as follows:

```
10 11 12 13 14
 9  8  7  6  5
 0  1  2  3  4
```

A strip wrapped around a cylinder can be treated as a number of columns and rows by using strip::Layout::Continuous and setting segment_length = to the number of LEDs on each level or rotation of the cylinder.

The default segment length is the entire strip. E.g. a 1D strip.

If the segment length = the strip size, the default layout is Continuous, otherwise it is ZigZag.

> [!NOTE]
> Currently only the FireGrid effect takes any notice of the strip 2D segmentation.

## Examples

### random

Just the Random colour effect. Simple example.

### comets

Simple Comets effect example using the launcher_task with comets going down the strip and pinging back up the strip.

### one_colour

Cycle through all 140 colours. Chaning colour every 2 secs.

Doesn't use the [crate::strip::frame_rate_task] to maintain a target FPS refresh rate. Just changes the colour and waits 2 secs in each loop iteration.

### strip_buttons

All effects relevant for an LED strip. Button 1 changes the effect.

| Effect | Button 2 |
|--------|----------|
| Random | Slows down the rate at which LED change colour. |
| Wheel | Speeds up the cycle throught the rainbow transition. |
| OneColour | Toggles from BLACK (off) to a random colour. |
| Comets | Manually launch another Comet. |
| Fire | Does nothing! |

### panel_buttons

All effects relevant to a 2D LED panel. Button 1 change the effect.

| Effect | Button 2 |
|--------|----------|
| Random | Slows down the rate at which LED change colour. |
| Wheel | Speeds up the cycle throught the rainbow transition. |
| OneColour | Toggles from BLACK (off) to a random colour. |
| FireGrid | Increase the cooling to reduce the flame. |
| FireGrid (smaller than the panel) | Does nothing! |