extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::*;
// use std::path::Path;
use pietify::*;
use syn::*;

fn create_draw_commands(path: String) -> proc_macro2::TokenStream {
  let operations = pietify::svg_to_strings(path);
  let mut stream = proc_macro2::TokenStream::new();

  for op in operations {
    stream.extend(op.parse::<proc_macro2::TokenStream>().unwrap());
  }

  stream
}

#[proc_macro_derive(SvgToPiet, attributes(file))]
pub fn svg_piet_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = input.ident;
  let attributes = input.attrs;

  let some_file_path = match attributes[0].parse_meta() {
    Ok(Meta::NameValue(nv)) => nv.lit,
    _ => help(),
  };

  let file_path = match some_file_path {
    Lit::Str(ref val) => val.value().clone(),
    _ => {
      panic!("#[derive(RustEmbed)] attribute value must be a string literal");
    }
  };

  let draw_commands = create_draw_commands(file_path);

  let output = quote! {
      impl #name {
          fn draw(paint_ctx: &mut PaintCtx, geom: &Geometry) {
              #draw_commands
          }
      }
  };
  proc_macro::TokenStream::from(output)
}

fn help() -> ! {
  panic!(
    "#[derive(SvgToPiet)] should contain one attribute like this #[file = \"examples/file.svg/\"]"
  );
}
