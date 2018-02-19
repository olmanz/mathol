extern crate mathol;
//use mathol::stochastics::probability::*;
//use mathol::stochastics::distribution::*;
use mathol::statistics::*;

fn main() {
    let n = get_span(&Vec::<f64>::new()).expect("error");
    println!("Result: {:?}", n);
}