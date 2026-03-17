# Embassy Pico 2 WS2812 LED Strip Effects

LED effects library and binary examples. For Raspberry Pico 2. Using Embassy embedded framework.

## Effects

| Effect | Description | Effect Button 2 |
|--------|-------------|----------|
| Random | Random colour changes at random times. | Increase random delay per pixel. |
| Wheel | Rotate each pixel through shades of red, green and blue. | Speed up rotation |
| OneColour | All pixels set to a single colour. | Alternate between Off (Black) & On (White). |
| FireGrid | Fire effect over a 2D grid. Can be Horizointal or Vertical | Increases cooling. |
| Fire | Simulated fire. | Increases cooling. |

_More to come_

Refer to effect_button.rs binary for example of all effects.

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

## Binary Crates

| Example | Description |
|---------|-------------|
| effect_buttons.rs | Rotates through all effects and second button adjusts an attribute of the effect. |
| random.rs | Just the random effect. Simple example. |