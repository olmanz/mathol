extern crate mathol;
use mathol::basic::Point;
use mathol::geometrics::planimetry::{Planimetry, Triangle};
use mathol::coordinatesystems::{Cartesic2D, Polar, Cartesic3D, Cylindrical, Spherical};
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