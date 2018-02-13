extern crate mathol;
use mathol::basic::{Point, pow};
use mathol::geometrics::planimetry::{Planimetry, Triangle};
use mathol::coordinatesystems::{Cartesic2D, Polar, Cartesic3D, Cylindrical, Spherical};
use mathol::stochastics::{factorial, permutation, combination, combination_with_repetition, variation, variation_with_repetition};
use mathol::stochastics::{binomial_distribution, hypergeometric_distribution, poisson_distribution};
use mathol::stochastics::{gaussian_distribution, standard_distribution, exponential_distribution};
use mathol::statistics::{get_arithmetic_mean, get_harmonic_mean, get_quadratic_mean, get_variance, get_standard_deviation};
use mathol::statistics::{get_min, get_max, get_span};
use mathol::vectoroperations::{Vector, Vectoroperations, Line, Plane};

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

#[test]
fn test_get_length_1() {
    let vec = Vector {x: 2, y: 3, z: 4};
    assert_eq!(5.385164807134504, vec.get_length());
}

#[test]
fn test_get_length_2() {
    let vec = Vector {x: 2.7, y: 3.6, z: 4.5};
    assert_eq!(6.363961030678928, vec.get_length());
}

#[test]
fn test_get_direction_angle() {
    let vec = Vector {x: 4, y: -2, z: 5};
    assert_eq!(0.9319311825594854, vec.get_direction_angle().0);
    assert_eq!(1.873542278417901, vec.get_direction_angle().1);
    assert_eq!(0.7297276562269663, vec.get_direction_angle().2);
}

#[test]
fn test_multiply_with_scalar_1() {
    let vec = Vector {x: 2, y: 3, z: 4};
    assert_eq!(4, vec.multiply_with_scalar(2).x);
    assert_eq!(6, vec.multiply_with_scalar(2).y);
    assert_eq!(8, vec.multiply_with_scalar(2).z);
}

#[test]
fn test_multiply_with_scalar_2() {
    let vec = Vector {x: 2.7, y: 3.6, z: 4.5};
    assert_eq!(5.4, vec.multiply_with_scalar(2.0).x);
    assert_eq!(7.2, vec.multiply_with_scalar(2.0).y);
    assert_eq!(9.0, vec.multiply_with_scalar(2.0).z);
}

#[test]
fn test_get_scalar_product_1() {
    let a = Vector {x: 2, y: 3, z: 4};
    let b = Vector {x: 5, y: 6, z: 7};
    assert_eq!(56, a.get_scalar_product(&b));
}

#[test]
fn test_get_scalar_product_2() {
    let a = Vector {x: 2.7, y: 3.6, z: 4.5};
    let b = Vector {x: 5.4, y: 6.3, z: 7.2};
    assert_eq!(69.66, a.get_scalar_product(&b));
}

#[test]
fn test_get_cut_angle_1() {
    let a = Vector {x: 1, y: 2, z: -3};
    let b = Vector {x: 5, y: -1, z: -5};
    assert_eq!(0.6736330697086078, a.get_cut_angle(&b));
}

#[test]
fn test_get_vector_product() {
    let a = Vector {x: 1, y: 4, z: 0};
    let b = Vector {x: -2, y: 5, z: 3};
    assert_eq!(12, a.get_vector_product(&b).x);
    assert_eq!(-3, a.get_vector_product(&b).y);
    assert_eq!(13, a.get_vector_product(&b).z);
}

#[test]
fn test_get_triple_product() {
    let a = Vector {x: 1, y: -2, z: 4};
    let b = Vector {x: 4, y: 1, z: 2};
    let c = Vector {x: -2, y: -5, z: 6};
    assert_eq!(0, a.get_triple_product(&b, &c))
}

#[test]
fn test_distance_from_point() {
    let p = Vector {x: 1, y: 5, z: 3};
    let l = Line {r: Vector {x: 1, y: 1, z: 4}, a: Vector {x: 2, y: -3, z: 5}};
    assert_eq!(3.0650834967591445, l.distance_from_point(&p));
}

#[test]
fn test_are_parallel() {
    let l1 = Line {r: Vector {x: 1, y: 0, z: 5}, a: Vector {x: 2, y: 1, z: 1}};
    let l2 = Line {r: Vector {x: 0, y: 2, z: 1}, a: Vector {x: 2, y: 1, z: 1}};
    assert_eq!(true, l1.are_parallel(&l2));
}

#[test]
fn test_distance_from_line_1() {
    let l1 = Line {r: Vector {x: 1, y: 0, z: 5}, a: Vector {x: 2, y: 1, z: 1}};
    let l2 = Line {r: Vector {x: 0, y: 2, z: 1}, a: Vector {x: 2, y: 1, z: 1}};
    assert_eq!(Ok(4.281744192888377), l1.distance_from_line(&l2));
}

#[test]
fn test_distance_from_line_2() {
    let l1 = Line {r: Vector {x: 5, y: 2, z: 1}, a: Vector {x: 1, y: 1, z: 3}};
    let l2 = Line {r: Vector {x: 2, y: -1, z: 0}, a: Vector {x: 3, y: 2, z: 1}};
    assert_eq!(Ok(0.8432740427115678), l1.distance_from_line(&l2));
}

#[test]
fn test_distance_from_line_3() {
    let l1 = Line {r: Vector {x: 1, y: 1, z: 0}, a: Vector {x: 2, y: 1, z: 1}};
    let l2 = Line {r: Vector {x: 2, y: 0, z: 2}, a: Vector {x: 1, y: -1, z: 2}};
    assert_eq!(Err("Lines do cross"), l1.distance_from_line(&l2));
}

#[test]
fn test_do_cross() {
    let l1 = Line {r: Vector {x: 1, y: 1, z: 0}, a: Vector {x: 2, y: 1, z: 1}};
    let l2 = Line {r: Vector {x: 2, y: 0, z: 2}, a: Vector {x: 1, y: -1, z: 2}};
    assert_eq!(true, l1.do_cross(&l2));
}

#[test]
fn test_are_askew() {
    let l1 = Line {r: Vector {x: 5, y: 2, z: 1}, a: Vector {x: 1, y: 1, z: 3}};
    let l2 = Line {r: Vector {x: 2, y: -1, z: 0}, a: Vector {x: 3, y: 2, z: 1}};
    assert_eq!(true, l1.are_askew(&l2));
}

#[test]
fn build_plane_from_three_points() {
    let p = Vector {x: 1, y: 1, z: 2};
    let q = Vector {x: 0, y: 4, z: -5};
    let r = Vector {x: -3, y: 4, z: 9};
    let vec = Plane::build_plane_from_three_points(&p, &q, &r);
    assert_eq!(1, vec.r.x);
    assert_eq!(1, vec.r.y);
    assert_eq!(2, vec.r.z);
    assert_eq!(-1, vec.a.x);
    assert_eq!(3, vec.a.y);
    assert_eq!(-7, vec.a.z);
    assert_eq!(-4, vec.b.x);
    assert_eq!(3, vec.b.y);
    assert_eq!(7, vec.b.z);
}

#[test]
fn test_get_distance_from_point() {
    let r = Vector {x: 3.0, y: 1.0, z: 8.0};
    let a = Vector {x: -2.0, y: 2.0, z: 1.0};
    let b = Vector {x: 4.5, y: 3.0, z: 1.0};
    let q = Vector {x: 1.0, y: 2.0, z: 0.0};
    let plane = Plane {r, a, b};
    assert_eq!(7.845728264713728, plane.get_distance_from_point(&q));
}

#[test]
fn test_is_plane_parallel_to_line() {
    let l = Line {r: Vector {x: 1, y: 2, z: 3}, a: Vector {x: 4, y: 2, z: 2}};
    let p = Plane {r: Vector {x: 2, y: 3, z: 5}, a: Vector {x: 2, y: 1, z: 1}, b: Vector {x: 1, y: 3, z: 4}};
    assert_eq!(true, p.is_parallel_to_line(&l));
}

#[test]
fn test_get_distance_of_plane_to_line() {
    let l = Line {r: Vector {x: 1, y: 2, z: 3}, a: Vector {x: 4, y: 2, z: 2}};
    let p = Plane {r: Vector {x: 2, y: 3, z: 5}, a: Vector {x: 2, y: 1, z: 1}, b: Vector {x: 1, y: 3, z: 4}};
    assert_eq!(Ok(0.46188021535170054), p.get_distance_from_line(&l));
}

#[test]
fn test_get_distance_of_plane_to_line_panic() {
    let l = Line {r: Vector {x: 1, y: 2, z: 3}, a: Vector {x: 4, y: 2, z: 3}};
    let p = Plane {r: Vector {x: 2, y: 3, z: 5}, a: Vector {x: 2, y: 1, z: 1}, b: Vector {x: 1, y: 3, z: 4}};
    assert_eq!(Err("Line is not parallel to plane"), p.get_distance_from_line(&l));
}

#[test]
fn test_is_plane_parallel_to_plane() {
    let p = Plane {r: Vector {x: 2, y: 3, z: 5}, a: Vector {x: 2, y: 1, z: 1}, b: Vector {x: 1, y: 3, z: 4}};
    let q = Plane {r: Vector {x: 4, y: 3, z: 7}, a: Vector {x: 4, y: 2, z: 2}, b: Vector {x: 2, y: 6, z: 8}};
    assert_eq!(true, p.is_parallel_to_plane(&q));
}

#[test]
fn test_get_distance_of_plane_to_plane() {
    let p = Plane {r: Vector {x: 2, y: 3, z: 5}, a: Vector {x: 2, y: 1, z: 1}, b: Vector {x: 1, y: 3, z: 4}};
    let q = Plane {r: Vector {x: 4, y: 3, z: 7}, a: Vector {x: 4, y: 2, z: 2}, b: Vector {x: 2, y: 6, z: 8}};
    assert_eq!(Ok(1.3856406460551016), p.get_distance_from_plane(&q));
}

#[test]
fn test_get_distance_of_plane_to_plane_panic() {
    let p = Plane {r: Vector {x: 2, y: 3, z: 5}, a: Vector {x: 2, y: 1, z: 1}, b: Vector {x: 1, y: 3, z: 4}};
    let q = Plane {r: Vector {x: 4, y: 3, z: 7}, a: Vector {x: 4, y: 2, z: 3}, b: Vector {x: 2, y: 6, z: 8}};
    assert_eq!(Err("The planes are not parallel"), p.get_distance_from_plane(&q));
}