# Embassy Pico 2 WS2812 LED Strip & 2D Panel Effects

An LED effects library with examples, for Raspberry Pico 2. Using the [Embassy](https://embassy.dev/) embedded framework.

## Effects

| Effect | Description | Effect Button 2 |
|--------|-------------|----------|
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

| Example | Description |
|---------|-------------|
| random.rs | Just the Random effect. Simple example. |
| strip_buttons.rs | Button 1 - Rotates through all LED strip effects. |
| panel_buttons.rs | Button 1 - Rotates through all 2D LED panel effects. |