fn main() {
    match std::env::args()
        .skip(1)
        .next()
        .and_then(|s| s.parse::<String>().ok())
    {
        Some(path) => {
            svg_to_piet::print_piet(&path);
        }
        _ => eprintln!("Oops, expected a valid path to an SVG"),
    }
}
