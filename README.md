# svg-to-piet

This is a simple utility to help convert an SVG into [piet](https://github.com/linebender/piet) draw instructions. Mostly for the sake of rapid UI prototyping in [druid](https://github.com/xi-editor/druid).

## Macro usage

The macro imports the SVG at compile time and turns into a function you can call at runtime by passing a paint_ctx.

```rust
use svg_to_piet::*;

#[derive(SvgToPiet)]
#[file = "test.svg"]
struct TrashCan;

TrashCan::draw(paint_ctx, geom);
```

## Command line usage

Outputs a list of draw instructions to the command line so you can copy and paste them wherever you'd like.

```sh
cargo run test.svg
```
