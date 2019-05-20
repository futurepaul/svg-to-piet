use svg_to_piet::*;

#[derive(SvgToPiet)]
#[file = "test2.svg"]
struct TrashCan;

fn main() {
  // This won't compile because we don't have render context or geometry
  // We'd normally call this inside the paint fn in Druid's Widget impl
  // (x and y are f64 btw)
  TrashCan::draw(paint_ctx, x, y);
}
