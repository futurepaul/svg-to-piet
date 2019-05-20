use svg_to_piet::*;

#[derive(SvgToPiet)]
#[file = "test2.svg"]
struct TrashCan;

fn main() {
  TrashCan::draw();
}
