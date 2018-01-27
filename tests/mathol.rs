extern crate mathol;
use mathol::basic::{Point, pow};
use mathol::geometrics::planimetry::{Planimetry, Triangle};
use mathol::coordinatesystems::{Cartesic2D, Polar, Cartesic3D, Cylindrical, Spherical};
use mathol::stochastics::{factorial, permutation, combination, combination_with_repetition, variation, variation_with_repetition};
use mathol::stochastics::{binomial_distribution, hypergeometric_distribution, poisson_distribution};
use mathol::stochastics::{gaussian_distribution, standard_distribution, exponential_distribution};

#[test]
fn test_pow() {
    assert_eq!(125, pow(5, 3));
    assert_eq!(125.0, pow(5.0, 3));
    assert_eq!(1, pow(2, 0));
}

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
fn test_factorial() {
    let zero= 0;
    assert_eq!(1, factorial(zero));
    assert_eq!(1, factorial(zero));


    assert_eq!(120, factorial(5_u8));
    assert_eq!(40320, factorial(8_u16));
    assert_eq!(479001600, factorial(12_u32));
    assert_eq!(2432902008176640000, factorial(20_u64));

    assert_eq!(120, factorial(5_i8));
    assert_eq!(5040, factorial(7_i16));
    assert_eq!(479001600, factorial(12_i32));
    assert_eq!(2432902008176640000, factorial(20_i64));
}

#[test]
#[should_panic(expected="Value for facultation must be a positive integer!")]
fn test_factorial_panic() {
    factorial(-1);
}

#[test]
fn test_permutation() {
    assert_eq!(10, permutation(5_u8, vec![3_u8, 2_u8]));
    assert_eq!(10, permutation(5_u16, vec![3_u16, 2_u16]));
    assert_eq!(10, permutation(5_u32, vec![3_u32, 2_u32]));
    assert_eq!(10, permutation(5_u64, vec![3_u64, 2_u64]));

    assert_eq!(10, permutation(5_i8, vec![3_i8, 2_i8]));
    assert_eq!(10, permutation(5_i16, vec![3_i16, 2_i16]));
    assert_eq!(10, permutation(5_i32, vec![3_i32, 2_i32]));
    assert_eq!(10, permutation(5_i64, vec![3_i64, 2_i64]));
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
    assert_eq!(10, combination(5_u8, 3_u8));
    assert_eq!(56, combination(8_u16, 5_u16));
    assert_eq!(495, combination(12_u32, 8_u32));
    assert_eq!(15504, combination(20_u64, 15_u64));

    assert_eq!(10, combination(5_i8, 2_i8));
    assert_eq!(35, combination(7_i16, 3_i16));
    assert_eq!(792, combination(12_i32, 7_i32));
    assert_eq!(38760, combination(20_i64, 14_i64));

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
    assert_eq!(35, combination_with_repetition(4, 4));
}

#[test]
fn test_variation_1() {
    assert_eq!(336, variation(8, 3));
}

#[test]
fn test_variation_2() {
    assert_eq!(40320, variation(8, 8));
}
//#[test]
//fn test_exponential_density() {
//    assert_eq!(0.0, exponential_density(2.0, -1.0));
//    assert_eq!(2.0, exponential_density(2.0, 0.0));
//    assert_eq!(0.2706705664732254, exponential_density(2.0, 1.0));
//    assert_eq!(0.03663127777746837, exponential_density(2.0, 2.0));
//}
#[test]
#[should_panic(expected="Number of selections outgrows the number of elements")]
fn test_variation_panic() {
    variation(8, 9);
}

#[test]
fn test_variation_with_repetition() {
    assert_eq!(125, variation_with_repetition(5, 3));
}

#[test]
fn test_binomial_distribution() {
    let mut vec = vec![];
    vec.push(0.5314410000000002);
    vec.push(0.35429400000000016);
    vec.push(0.09841500000000003);
    vec.push(0.014580000000000004);
    vec.push(0.0012150000000000004);
    vec.push(0.00005400000000000002);
    vec.push(0.0000010000000000000004);
    assert_eq!(vec, binomial_distribution(6_i32, 0.1));
}

#[test]
#[should_panic(expected="p must be in a range between 0 and 1!")]
fn test_binomial_panic_1() {
    binomial_distribution(6, -0.1);
}

#[test]
#[should_panic(expected="p must be in a range between 0 and 1!")]
fn test_binomial_panic_2() {
    binomial_distribution(6, 1.1);
}

//#[test]
//fn test_convert() {
//    assert_eq!(1.0, convert(1_i8));
//    assert_eq!(1.0, convert(1_i16));
//    assert_eq!(1.0, convert(1_i32));
//    assert_eq!(1.0, convert(1_i64));
//
//    assert_eq!(1.0, convert(1_u8));
//    assert_eq!(1.0, convert(1_u16));
//    assert_eq!(1.0, convert(1_u32));
//    assert_eq!(1.0, convert(1_u64));
//}
//
#[test]
fn test_hypergeometric_distribution() {
    let mut vec = vec![];
    vec.push(0.010101010101010102);
    vec.push(0.1414141414141414);
    vec.push(0.42424242424242425);
    vec.push(0.35353535353535354);
    vec.push(0.0707070707070707);
    assert_eq!(vec, hypergeometric_distribution(12, 7, 4));
}

#[test]
#[should_panic(expected="Parameter M must be smaller than N!")]
fn test_hypergeometric_panic_1() {
    hypergeometric_distribution(7, 12, 4);
}

#[test]
#[should_panic(expected="Parameter n must be smaller than N!")]
fn test_hypergeometric_panic_2() {
    hypergeometric_distribution(12, 7, 14);
}

#[test]
fn test_poisson_distribution() {
    let my = 1;
    assert_eq!(0.36787944117144233, poisson_distribution(my, 0));
    assert_eq!(0.36787944117144233, poisson_distribution(my, 1));
    assert_eq!(0.18393972058572117, poisson_distribution(my, 2));
    assert_eq!(0.061313240195240384, poisson_distribution(my, 3));
    assert_eq!(0.015328310048810096, poisson_distribution(my, 4));
}

#[test]
#[should_panic(expected="Parameter µ must be positive!")]
fn test_poisson_panic() {
    poisson_distribution(-1, 0);
}

#[test]
fn test_gaussian_distribution() {
    assert_eq!(0.10648266850745075, gaussian_distribution(2.0, 3.0, 4.0));
    assert_eq!(0.10648266850745075, gaussian_distribution(2, 3, 4));
    assert_eq!(0.017996988837729353, gaussian_distribution(2, 3, -4));
}

#[test]
#[should_panic(expected="Parameter \u{03c3} must be bigger than 0!")]
fn test_gaussian_panic() {
    gaussian_distribution(2, -1, 4);
}

#[test]
fn test_standard_distribution() {
    assert_eq!(0.3989422804014327, standard_distribution(0.0));
    assert_eq!(0.24197072451914337, standard_distribution(1.0));
    assert_eq!(0.05399096651318806, standard_distribution(2.0));

    assert_eq!(0.3989422804014327, standard_distribution(0));
    assert_eq!(0.24197072451914337, standard_distribution(1));
    assert_eq!(0.05399096651318806, standard_distribution(2));
}

#[test]
fn test_exponential_density() {
    assert_eq!(0.0, exponential_distribution(2.0, -1.0));
    assert_eq!(2.0, exponential_distribution(2.0, 0.0));
    assert_eq!(0.2706705664732254, exponential_distribution(2.0, 1.0));
    assert_eq!(0.03663127777746836, exponential_distribution(2.0, 2.0));
}

#[test]
#[should_panic(expected="Parameter \u{03bb} must be bigger than 0!")]
fn test_exponential_panic() {
    exponential_distribution(-1, 0);
}