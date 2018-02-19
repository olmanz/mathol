extern crate mathol;
use mathol::geometrics::planimetry::*;

fn main() {
    let t = Triangle::build_triangle_with_edges(3, 4, 5).unwrap();
    let (alpha, beta, gamma) = t.get_angles();
    println!("alpha: {}, beta: {}, gamma: {}", alpha, beta, gamma);
    let c = t.get_inner_circle();
    let area = t.get_area();
    let perimeter = t.get_perimeter();
}