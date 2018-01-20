extern crate mathol;
use mathol::basic::Point;
use mathol::geometrics::planimetry::{Planimetry, Triangle};
use mathol::coordinatesystems::{Cartesic2D, Polar, Cartesic3D, Cylindrical, Spherical};
use mathol::stochastics::{faculty, permutation, combination, combination_with_repetition, variation, variation_with_repetition};
use std::cmp::Ordering;

#[test]
fn test_triangle_1() {
    let triangle = Triangle::build_with_vertices(Point{x: 2.0, y: 1.0}, Point{x: 5.5, y: 2.0}, Point{x: 3.0, y: 4.5});
    assert_eq!(3.5355339059327378, triangle.a_edge);
    assert_eq!(3.640054944640259, triangle.b_edge);
    assert_eq!(3.640054944640259, triangle.c_edge);
    assert_eq!(5.624999999999997, triangle.get_area());
    assert_eq!(10.815643795213255, triangle.get_perimeter());
}

#[test]
fn test_cartesic_to_polar_1() {
    let cart = Cartesic2D {x: 4.0, y: 3.0};
    let pol: Polar = cart.transform_to_polar();
    assert_eq!(5.0, pol.r);
    assert_eq!(36.86989764584402, pol.phi);
}

#[test]
fn test_cartesic_to_polar_2() {
    let cart = Cartesic2D {x: -4.0, y: 3.0};
    let pol: Polar = cart.transform_to_polar();
    assert_eq!(5.0, pol.r);
    assert_eq!(143.13010235415598, pol.phi);
}

#[test]
fn test_cartesic_to_polar_3() {
    let cart = Cartesic2D {x: -4.0, y: -3.0};
    let pol: Polar = cart.transform_to_polar();
    assert_eq!(5.0, pol.r);
    assert_eq!(-143.13010235415598, pol.phi);
}

#[test]
fn test_cartesic_to_polar_4() {
    let cart = Cartesic2D {x: 4.0, y: -3.0};
    let pol: Polar = cart.transform_to_polar();
    assert_eq!(5.0, pol.r);
    assert_eq!(-36.86989764584402, pol.phi);
}

#[test]
fn test_polar_to_cartesic_1() {
    let pol = Polar {r: 5.0, phi: 36.86989764584402};
    let cart: Cartesic2D = pol.transform_to_cartesic2d();
    assert_eq!(4.0, cart.x);
    assert_eq!(3.0, cart.y);
}

#[test]
fn test_polar_to_cartesic_2() {
    let pol = Polar {r: 5.0, phi: 143.13010235415598};
    let cart: Cartesic2D = pol.transform_to_cartesic2d();
    assert_eq!(-4.0, cart.x);
    assert_eq!(3.0, cart.y);
}

#[test]
fn test_polar_to_cartesic_3() {
    let pol = Polar {r: 5.0, phi: -143.13010235415598};
    let cart: Cartesic2D = pol.transform_to_cartesic2d();
    assert_eq!(-4.0, cart.x);
    assert_eq!(-3.0, cart.y);
}

#[test]
fn test_polar_to_cartesic_4() {
    let pol = Polar {r: 5.0, phi: -36.86989764584402};
    let cart: Cartesic2D = pol.transform_to_cartesic2d();
    assert_eq!(4.0, cart.x);
    assert_eq!(-3.0, cart.y);
}

#[test]
fn test_cartesic_to_cylindrical_1() {
    let cart = Cartesic3D {x: 3.0, y: 4.0, z: 5.0};
    let cyl: Cylindrical = cart.transform_to_cylindrical();
    assert_eq!(5.0, cyl.rho);
    assert_eq!(53.13010235415598, cyl.phi);
    assert_eq!(5.0, cyl.z);
}

#[test]
fn test_cartesic_to_cylindrical_2() {
    let cart = Cartesic3D {x: 0.0, y: 4.0, z: 5.0};
    let cyl: Cylindrical = cart.transform_to_cylindrical();
    assert_eq!(4.0, cyl.rho);
    assert_eq!(90.0, cyl.phi);
    assert_eq!(5.0, cyl.z);
}

#[test]
fn test_cartesic_to_cylindrical_3() {
    let cart = Cartesic3D {x: 0.0, y: -4.0, z: 5.0};
    let cyl: Cylindrical = cart.transform_to_cylindrical();
    assert_eq!(4.0, cyl.rho);
    assert_eq!(270.0, cyl.phi);
    assert_eq!(5.0, cyl.z);
}

#[test]
fn test_cylindrical_to_cartesic() {
    let cyl = Cylindrical {rho: 5.0, phi: 53.13010235415598, z: 5.0};
    let cart: Cartesic3D = cyl.transform_to_cartesic3d();
    assert_eq!(3.0000000000000004, cart.x);
    assert_eq!(3.9999999999999996, cart.y);
    assert_eq!(5.0, cart.z);
}

#[test]
fn test_cartesic_to_spherical() {
    let cart = Cartesic3D {x: 3.0, y: 4.0, z: 5.0};
    let sph: Spherical = cart.transform_to_spherical();
    assert_eq!(7.0710678118654755, sph.r);
    assert_eq!(45.00000000000001, sph.theta);
    assert_eq!(53.13010235415598, sph.phi);
}

#[test]
fn test_spherical_to_cartesic() {
    let sph = Spherical {r: 7.0710678118654755, theta: 45.0, phi: 53.13010235415598};
    let cart: Cartesic3D = sph.transform_to_cartesic3d();
    assert_eq!(3.0000000000000004, cart.x);
    assert_eq!(3.9999999999999996, cart.y);
    assert_eq!(5.000000000000001, cart.z);
}

#[test]
fn test_faculty() {
    assert_eq!(362880, faculty(9));
}

#[test]
fn test_permutation() {
    assert_eq!(10, permutation(5, vec![3, 2]));
}

#[test]
#[should_panic(expected="Sum of parts is not equal to whole")]
fn test_permutation_panic_1() {
    permutation(5, vec![3, 3]);
}

#[test]
#[should_panic(expected="Sum of parts is not equal to whole")]
fn test_permutation_panic_2() {
    permutation(5, vec![1, 3]);
}

#[test]
fn test_combination_1() {
    assert_eq!(120, combination(10, 3));
}

#[test]
fn test_combination_2() {
    assert_eq!(1, combination(10, 10));
}

#[test]
#[should_panic(expected="Number of selections outgrows the number of elements")]
fn test_combination_panic() {
    combination(10, 11);
}

#[test]
fn test_combination_with_repetition() {
    assert_eq!(220, combination_with_repetition(10, 3));
}

#[test]
fn test_combination_with_repetition_2() {
    assert_eq!(92378, combination_with_repetition(10, 10));
}

#[test]
fn test_variation_1() {
    assert_eq!(336, variation(8, 3));
}

#[test]
fn test_variation_2() {
    assert_eq!(40320, variation(8, 8));
}

#[test]
#[should_panic(expected="Number of selections outgrows the number of elements")]
fn test_variation_panic() {
    variation(8, 9);
}

#[test]
fn test_variation_with_repetition() {
    assert_eq!(125, variation_with_repetition(5, 3));
}