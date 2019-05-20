use std::fmt;

use usvg::{Fill, NodeKind, Paint, PathSegment, Stroke};

#[derive(Clone, Copy, PartialEq)]
struct MStr<'a>(&'a str);

impl<'a> fmt::Debug for MStr<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

const RENDER_CONTEXT: &str = "paint_ctx.render_ctx";

fn brush_from_fill(fill: Fill) -> String {
  let mut color = String::new();
  match fill.paint {
    Paint::Color(c) => {
      let alpha = (fill.opacity.value() * 255.0) as u8;
      color = format!("0x{:x}_{:x}_{:x}_{:x}", c.red, c.green, c.blue, alpha);
    }
    _ => {
      //TODO figure this out!
      println!("I don't know what a Paint::Link is");
    }
  }
  format!(
    "let brush = {}.solid_brush({}).unwrap();",
    RENDER_CONTEXT, color
  )
}

fn brush_from_stroke(stroke: Stroke) -> String {
  let mut color = String::new();
  match stroke.paint {
    Paint::Color(c) => {
      let alpha = (stroke.opacity.value() * 255.0) as u8;
      color = format!("0x{:x}_{:x}_{:x}_{:x}", c.red, c.green, c.blue, alpha);
    }
    _ => {
      //TODO figure this out!
      println!("I don't know what a Paint::Link is");
    }
  }
  format!(
    "let brush = {}.solid_brush({}).unwrap();",
    RENDER_CONTEXT, color
  )
}

//TODO return a result
pub fn svg_to_strings(path: String) -> Vec<String> {
  let re_opt = usvg::Options {
    keep_named_groups: false,
    ..usvg::Options::default()
  };

  let tree = match usvg::Tree::from_file(path, &re_opt) {
    Ok(tree) => tree,
    Err(err) => panic!("Couldn't load SVG from file because: {}", err),
  };

  let root = tree.root();

  let mut piet_instructions: Vec<String> = Vec::new();

  for n in root.children() {
    match *n.borrow() {
      NodeKind::Path(ref p) => {
        // dbg!(p);

        piet_instructions.push("let mut path = BezPath::new();".to_string());
        for seg in &p.segments {
          match *seg {
            PathSegment::MoveTo { x, y } => {
              piet_instructions.push(format!("path.moveto(({:.2} + x, {:.2} + y));", x, y));
            }
            PathSegment::LineTo { x, y } => {
              piet_instructions.push(format!("path.lineto(({:.2} + x, {:.2} + y));", x, y));
            }
            PathSegment::CurveTo {
              x1,
              y1,
              x2,
              y2,
              x,
              y,
            } => {
              //QUESTION is this order correct?
              piet_instructions.push(format!(
                "path.curveto(({:.2} + x, {:.2} + y), ({:.2} + x, {:.2} + y), ({:.2} + x, {:.2} + y));",
                x1, y1, x2, y2, x, y
              ));
            }
            PathSegment::ClosePath => {
              piet_instructions.push(format!("path.closepath();"));
            }
          }
        }
        match p.fill {
          Some(ref fill) => {
            piet_instructions.push(brush_from_fill(fill.clone()));
            piet_instructions.push(format!(
              "{}.fill(path.clone(), &brush, FillRule::NonZero);",
              RENDER_CONTEXT
            ));
          }
          None => {}
        }

        match p.stroke {
          Some(ref stroke) => {
            piet_instructions.push(brush_from_stroke(stroke.clone()));
            piet_instructions.push(format!(
              "{}.stroke(path, &brush, {:.2}, None);",
              RENDER_CONTEXT,
              stroke.width.value()
            ));
          }
          None => {}
        }
      }
      _ => {}
    }
  }
  piet_instructions
}
