use svg_to_piet::*;

#[derive(SvgToPiet)]
#[file = "test2.svg"]
struct TrashCan;

fn main() {
    // This won't compile because we don't have render context or geometry
    // We'd normally call this inside the paint fn in Druid's Widget impl
    TrashCan::draw(paint_ctx, geom);
}
