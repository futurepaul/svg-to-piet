# svg-to-piet

This is a simple utility to help convert an SVG into [piet](https://github.com/linebender/piet) draw instructions. Mostly for the sake of rapid UI prototyping in [druid](https://github.com/xi-editor/druid).

## Macro usage

```
use svg_to_piet::*;

#[derive(SvgToPiet)]
#[file = "test2.svg"]
struct TrashCan;

TrashCan::draw(paint_ctx, geom);
```

## Command line usage

```
cargo run test.svg
```
