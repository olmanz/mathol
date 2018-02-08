extern crate mathol;
use mathol::basic::{Point, pow};
use mathol::geometrics::planimetry::{Planimetry, Triangle};
use mathol::coordinatesystems::{Cartesic2D, Polar, Cartesic3D, Cylindrical, Spherical};
use mathol::stochastics::{factorial, permutation, combination, combination_with_repetition, variation, variation_with_repetition};
use mathol::stochastics::{binomial_distribution, hypergeometric_distribution, poisson_distribution};
use mathol::stochastics::{gaussian_distribution, standard_distribution, exponential_distribution};
use mathol::statistics::{get_arithmetic_mean, get_harmonic_mean, get_quadratic_mean, get_variance, get_standard_deviation};
use mathol::statistics::{get_min, get_max, get_span};

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
    assert_eq!(1, factorial(zero).unwrap());
    assert_eq!(1, factorial(zero).unwrap());


    assert_eq!(120, factorial(5_u8).unwrap());
    assert_eq!(40320, factorial(8_u16).unwrap());
    assert_eq!(479001600, factorial(12_u32).unwrap());
    assert_eq!(2432902008176640000, factorial(20_u64).unwrap());

    assert_eq!(120, factorial(5_i8).unwrap());
    assert_eq!(5040, factorial(7_i16).unwrap());
    assert_eq!(479001600, factorial(12_i32).unwrap());
    assert_eq!(2432902008176640000, factorial(20_i64).unwrap());
}

#[test]
fn test_factorial_panic() {
    assert_eq!(Err("Value for facultation must be a positive integer!"), factorial(-1));
}

#[test]
fn test_permutation() {
    assert_eq!(10, permutation(5_u8, vec![3_u8, 2_u8]).unwrap());
    assert_eq!(10, permutation(5_u16, vec![3_u16, 2_u16]).unwrap());
    assert_eq!(10, permutation(5_u32, vec![3_u32, 2_u32]).unwrap());
    assert_eq!(10, permutation(5_u64, vec![3_u64, 2_u64]).unwrap());

    assert_eq!(10, permutation(5_i8, vec![3_i8, 2_i8]).unwrap());
    assert_eq!(10, permutation(5_i16, vec![3_i16, 2_i16]).unwrap());
    assert_eq!(10, permutation(5_i32, vec![3_i32, 2_i32]).unwrap());
    assert_eq!(10, permutation(5_i64, vec![3_i64, 2_i64]).unwrap());
}

#[test]
fn test_permutation_panic_1() {
    assert_eq!(Err("Sum of parts is not equal to whole"), permutation(5, vec![3, 3]));
}

#[test]
fn test_permutation_panic_2() {
    assert_eq!(Err("Sum of parts is not equal to whole"), permutation(5, vec![1, 3]));
}

#[test]
fn test_permutation_panic_3() {
    assert_eq!(Err("Parameter n must be a positive integer!"), permutation(-1, vec![3, 2]));
}

#[test]
fn test_permutation_panic_4() {
    assert_eq!(Err("Parameter karr is an empty vector!"), permutation(5, vec![]));
}

#[test]
fn test_combination_1() {
    assert_eq!(10, combination(5_u8, 3_u8).unwrap());
    assert_eq!(56, combination(8_u16, 5_u16).unwrap());
    assert_eq!(495, combination(12_u32, 8_u32).unwrap());
    assert_eq!(15504, combination(20_u64, 15_u64).unwrap());

    assert_eq!(10, combination(5_i8, 2_i8).unwrap());
    assert_eq!(35, combination(7_i16, 3_i16).unwrap());
    assert_eq!(792, combination(12_i32, 7_i32).unwrap());
    assert_eq!(38760, combination(20_i64, 14_i64).unwrap());

    assert_eq!(1, combination(10, 10).unwrap());
}

#[test]
fn test_combination_panic_1() {
    assert_eq!(Err("Number of selections outgrows the number of elements"), combination(10, 11));
}

#[test]
fn test_combination_panic_2() {
    assert_eq!(Err("Parameter n must be a positive integer!"), combination(-1, 2));
}

#[test]
fn test_combination_panic_3() {
    assert_eq!(Err("Parameter k must be a positive integer!"), combination(2, -1));
}

#[test]
fn test_combination_with_repetition() {
    assert_eq!(220, combination_with_repetition(10, 3).unwrap());
}

#[test]
fn test_combination_with_repetition_2() {
    assert_eq!(35, combination_with_repetition(4, 4).unwrap());
}

#[test]
fn test_combination_with_repetition_panic_1() {
    assert_eq!(Err("Parameter n must be a positive integer!"), combination_with_repetition(-1, 2));
}

#[test]
fn test_combination_with_repetition_panic_2() {
    assert_eq!(Err("Parameter k must be a positive integer!"), combination_with_repetition(2, -1));
}

#[test]
fn test_variation_1() {
    assert_eq!(336, variation(8, 3).unwrap());
}

#[test]
fn test_variation_2() {
    assert_eq!(40320, variation(8, 8).unwrap());
}

#[test]
fn test_variation_panic() {
    assert_eq!(Err("Number of selections outgrows the number of elements"), variation(8, 9));
}

#[test]
fn test_variation_panic_2() {
    assert_eq!(Err("Parameter n must be a positive integer!"), variation(-1, 2));
}

#[test]
fn test_variation_panic_3() {
    assert_eq!(Err("Parameter k must be a positive integer!"), variation(2, -1));
}

#[test]
fn test_variation_with_repetition() {
    assert_eq!(125, variation_with_repetition(5, 3).unwrap());
}

#[test]
fn test_variation_with_repetition_panic_1() {
    assert_eq!(Err("Parameter n must be a positive integer!"), variation_with_repetition(-1, 2));
}

#[test]
fn test_variation_with_repetition_panic_2() {
    assert_eq!(Err("Parameter k must be a positive integer!"), variation_with_repetition(2, -1));
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
    assert_eq!(vec, binomial_distribution(6_i32, 0.1).unwrap());
}

#[test]
fn test_binomial_panic_1() {
    assert_eq!(Err("p must be in a range between 0 and 1!"), binomial_distribution(6, -0.1));
}

#[test]
fn test_binomial_panic_2() {
    assert_eq!(Err("p must be in a range between 0 and 1!"), binomial_distribution(6, 1.1));
}

#[test]
fn test_binomial_panic_3() {
    assert_eq!(Err("Parameter n must be a positive integer!"), binomial_distribution(0, 0.5));
}

#[test]
fn test_hypergeometric_distribution() {
    let mut vec = vec![];
    vec.push(0.010101010101010102);
    vec.push(0.1414141414141414);
    vec.push(0.42424242424242425);
    vec.push(0.35353535353535354);
    vec.push(0.0707070707070707);
    assert_eq!(vec, hypergeometric_distribution(12, 7, 4).unwrap());
}

#[test]
fn test_hypergeometric_panic_1() {
    assert_eq!(Err("Parameter M must be smaller than N!"), hypergeometric_distribution(7, 12, 4));
}

#[test]
fn test_hypergeometric_panic_2() {
    assert_eq!(Err("Parameter n must be smaller than N!"), hypergeometric_distribution(12, 7, 14));
}

#[test]
fn test_hypergeometric_panic_3() {
    assert_eq!(Err("Parameter N must be a positive integer!"), hypergeometric_distribution(0, 7, 14));
}

#[test]
fn test_hypergeometric_panic_4() {
    assert_eq!(Err("Parameter M must be a positive integer!"), hypergeometric_distribution(12, 0, 14));
}

#[test]
fn test_hypergeometric_panic_5() {
    assert_eq!(Err("Parameter n must be a positive integer!"), hypergeometric_distribution(12, 7, 0));
}

#[test]
fn test_poisson_distribution() {
    let my = 1;
    assert_eq!(0.36787944117144233, poisson_distribution(my, 0).unwrap());
    assert_eq!(0.36787944117144233, poisson_distribution(my, 1).unwrap());
    assert_eq!(0.18393972058572117, poisson_distribution(my, 2).unwrap());
    assert_eq!(0.061313240195240384, poisson_distribution(my, 3).unwrap());
    assert_eq!(0.015328310048810096, poisson_distribution(my, 4).unwrap());
}

#[test]
fn test_poisson_panic_1() {
    assert_eq!(Err("Parameter µ must be positive!"), poisson_distribution(-1, 0));
}

#[test]
fn test_poisson_panic_2() {
    assert_eq!(Err("Parameter x must be a positive integer!"), poisson_distribution(5, -1));
}

#[test]
fn test_gaussian_distribution() {
    assert_eq!(0.10648266850745075, gaussian_distribution(2.0, 3.0, 4.0).unwrap());
    assert_eq!(0.10648266850745075, gaussian_distribution(2, 3, 4).unwrap());
    assert_eq!(0.017996988837729353, gaussian_distribution(2, 3, -4).unwrap());
}

#[test]
fn test_gaussian_panic() {
    assert_eq!(Err("Parameter \u{03c3} must be bigger than 0!"), gaussian_distribution(2, -1, 4));
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
    assert_eq!(0.0, exponential_distribution(2.0, -1.0).unwrap());
    assert_eq!(2.0, exponential_distribution(2.0, 0.0).unwrap());
    assert_eq!(0.2706705664732254, exponential_distribution(2.0, 1.0).unwrap());
    assert_eq!(0.03663127777746836, exponential_distribution(2.0, 2.0).unwrap());
}

#[test]
fn test_exponential_panic() {
    assert_eq!(Err("Parameter \u{03bb} must be bigger than 0!"), exponential_distribution(-1, 0));
}

//#[test]
//fn test_get_absolute_frequency() {
//    let a = vec![9.0, 5.0, 4.0, 9.0, 9.0, 0.0, 4.0];
//    let map = get_sample_2(&a);
//    let sample = &*(map.get(&9).unwrap());
//    assert_eq!(3, sample.absolute_frequency);
//    assert_eq!(0.42857142857142855, sample.relative_frequency);
//}

#[test]
fn test_arithmetic_mean_1() {
    let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
    assert_eq!(10.1, get_arithmetic_mean(&vec).unwrap());
}

#[test]
fn test_arithmetic_mean_2() {
    let vec = [9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
    assert_eq!(10.1, get_arithmetic_mean(&vec).unwrap());
}

#[test]
fn test_arithmetic_mean_3() {
    let vec = vec![8, 6, 5, 11, 6, 6];
    assert_eq!(7.0, get_arithmetic_mean(&vec).unwrap());
}

#[test]
fn test_arithmetic_mean_4() {
    let vec = vec![9];
    assert_eq!(9.0, get_arithmetic_mean(&vec).unwrap());
}

#[test]
fn test_arithmetic_mean_5() {
    let vec: Vec<f64> = vec![];
    assert_eq!(Err("Vector or Array is empty"), get_arithmetic_mean(&vec));
}

#[test]
fn test_variance_1() {
    let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
    assert_eq!(0.034285714285714246, get_variance(&vec).unwrap());
}

#[test]
fn test_harmonic_mean_1() {
    let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
    assert_eq!(10.097006905739999, get_harmonic_mean(&vec).unwrap());
}

#[test]
fn test_harmonic_mean_2() {
    let vec = vec![8, 6, 5, 11, 6, 6];
    assert_eq!(6.550868486352359, get_harmonic_mean(&vec).unwrap());
}

#[test]
fn test_harmonic_mean_3() {
    let vec: Vec<f64> = vec![];
    assert_eq!(Err("Vector or Array is empty"), get_harmonic_mean(&vec));
}


#[test]
fn test_harmonic_mean_4() {
    let vec = vec![8, 6, 5, 0, 6, 6];
    assert_eq!(Err("Vector or Array contains zero"), get_harmonic_mean(&vec));
}

#[test]
fn test_quadratic_mean_1() {
    let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
    assert_eq!(10.101485039339513, get_quadratic_mean(&vec).unwrap());
}

#[test]
fn test_quadratic_mean_2() {
    let vec = vec![8, 6, 5, 11, 6, 6];
    assert_eq!(7.280109889280518, get_quadratic_mean(&vec).unwrap());
}

#[test]
fn test_quadratic_mean_3() {
    let vec: Vec<f64> = vec![];
    assert_eq!(Err("Vector or Array is empty"), get_quadratic_mean(&vec));
}
#[test]
fn test_variance_2() {
    let vec = vec![8, 6, 5, 11, 6, 6];
    assert_eq!(4.8, get_variance(&vec).unwrap());
}

#[test]
fn test_variance_3() {
    let vec: Vec<f64> = vec![];
    assert_eq!(Err("Vector or Array is empty"), get_variance(&vec));
}

#[test]
fn test_standard_deviation_1() {
    let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
    assert_eq!(0.18516401995451018, get_standard_deviation(&vec).unwrap());
}


#[test]
fn test_standard_deviation_2() {
    let vec = vec![8, 6, 5, 11, 6, 6];
    assert_eq!(2.1908902300206643, get_standard_deviation(&vec).unwrap());
}

#[test]
fn test_standard_deviation_3() {
    let vec: Vec<i64> = vec![];
    assert_eq!(Err("Vector or Array is empty"), get_standard_deviation(&vec));
}

#[test]
fn test_get_min_1() {
    let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
    assert_eq!(9.8, get_min(&vec).unwrap());
}

#[test]
fn test_get_min_2() {
    let vec = vec![8, 6, 5, 11, 6, 6];
    assert_eq!(5, get_min(&vec).unwrap());
}

#[test]
fn test_get_min_3() {
    let vec: Vec<f64> = vec![];
    assert_eq!(Err("Vector or Array is empty"), get_min(&vec));
}

#[test]
fn test_get_max_1() {
    let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
    assert_eq!(10.3, get_max(&vec).unwrap());
}

#[test]
fn test_get_max_2() {
    let vec = vec![8, 6, 5, 11, 6, 6];
    assert_eq!(11, get_max(&vec).unwrap());
}

#[test]
fn test_get_max_3() {
    let vec: Vec<f64> = vec![];
    assert_eq!(Err("Vector or Array is empty"), get_min(&vec));
}

#[test]
fn test_get_span_1() {
    let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
    assert_eq!(0.5, get_span(&vec).unwrap());
}

#[test]
fn test_get_span_2() {
    let vec = vec![8, 6, 5, 11, 6, 6];
    assert_eq!(6, get_span(&vec).unwrap());
}

#[test]
fn test_get_span_3() {
    let vec: Vec<f64> = vec![];
    assert_eq!(Err("Vector or Array is empty"), get_span(&vec));
}

//#[test]
//fn test_sample_build() {
//    let n = 25;
//    let a = vec![1, 3, 6, 9, 4, 2];
//    let r = vec![0.04, 0.12, 0.24, 0.36, 0.16, 0.08];
//    let f = vec![0.04, 0.16, 0.40, 0.76, 0.92, 1.0];
//    let sample = Sample::build(n, &a);
//    assert_eq!(n, sample.number_of_elements);
//    assert_eq!(a, sample.absolute_frequency);
//    assert_eq!(r, sample.relative_frequency);
//    assert_eq!(f, sample.distribution_function);
//}
//
//#[test]
//#[should_panic(expected="Sum of sample units must equal number_of_elements")]
//fn test_sample_build_panic() {
//    let n = 25;
//    let a = vec![1, 3, 6, 9, 4, 1];
//    Sample::build(n, &a);
//}