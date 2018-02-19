extern crate mathol;
use mathol::matrices::matrice::Matrice;


fn main() {
    let m = Matrice::build_matrice(3, 3, vec![1, 1, -2, 1, -1, -2, 2, 3, -4]).unwrap();
    let result = m.solve(&vec![0, 0, 0]).expect("error");
    println!("{:?}", result);
}