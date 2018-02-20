extern crate mathol;
use mathol::basics::{pow};
use mathol::geometrics::planimetry::{Triangle, Rectangle, Parallelogram, Trapeze, Polygon, Circle, Ellipsis};
use mathol::geometrics::stereometry::{Cuboid, Pyramid, Wedge, Cylinder, Cone, Sphere, Ellipsoid, SphericBarrel, ParabolicBarrel, Torus};
use mathol::geometrics::traits::*;
use mathol::stochastics::probability::{factorial, permutation, combination, combination_with_repetition, variation, variation_with_repetition};
use mathol::stochastics::distribution::{binomial_distribution, hypergeometric_distribution, poisson_distribution};
use mathol::stochastics::distribution::{gaussian_distribution, standard_distribution, exponential_distribution};
use mathol::statistics::{get_arithmetic_mean, get_harmonic_mean, get_quadratic_mean, get_variance, get_standard_deviation};
use mathol::statistics::{get_min, get_max, get_span};
use mathol::vectoroperations::vector2d::{Vector2D, Polar};
use mathol::vectoroperations::vector3d::{Vector3D, Cylindrical, Spherical};
use mathol::vectoroperations::line3d::Line3D;
use mathol::vectoroperations::plane::Plane;
use mathol::matrices::matrice::Matrice;
use mathol::matrices::solvable::Solvable;

#[test]
fn test_pow() {
    assert_eq!(125, pow(5, 3));
    assert_eq!(125.0, pow(5.0, 3));
    assert_eq!(1, pow(2, 0));
}

#[test]
fn test_triangle_1() {
    let triangle = Triangle::build_triangle_with_edges(3, 4, 5).expect("error");
    assert_eq!(6.0 , triangle.get_area());
}

#[test]
fn test_triangle_2() {
    let triangle = Triangle::build_triangle_with_points(Vector2D{x: 1, y:2}, Vector2D{x: 6, y:2}, Vector2D{x: 3, y:4});
    assert_eq!(3.605551275463989, triangle.a);
    assert_eq!(2.8284271247461903, triangle.b);
    assert_eq!(5.0, triangle.c);
}

#[test]
#[should_panic(expected="Length of triangle edges must be positive")]
fn test_triangle_panic_1() {
    Triangle::build_triangle_with_edges(-3, 4, 5).expect("error");
}

#[test]
#[should_panic(expected="Cannot create a triangle with the given edges")]
fn test_triangle_panic_2() {
    Triangle::build_triangle_with_edges(1, 2, 5).expect("error");
}

#[test]
fn test_perimeter_1() {
    let triangle = Triangle::build_triangle_with_edges(3, 4, 5).unwrap();
    assert_eq!(12.0 , triangle.get_perimeter());
}

#[test]
fn test_get_angles_1() {
    let triangle = Triangle::build_triangle_with_edges(3, 4, 5).unwrap();
    let (alpha, beta, gamma) = triangle.get_angles();
    assert_eq!(36.86989764584401, alpha);
    assert_eq!(53.13010235415599, beta);
    assert_eq!(90.0, gamma);
}

#[test]
fn get_triangle_height() {
    let triangle = Triangle::build_triangle_with_edges(3, 4, 5).unwrap();
    assert_eq!(2.4000000000000004, triangle.get_height());
}

#[test]
fn get_triangle_inner_circle() {
    let triangle = Triangle::build_triangle_with_edges(3, 4, 5).expect("error");
    let circle = triangle.get_inner_circle();
    assert_eq!(1.0, circle.r);
}

#[test]
fn get_triangle_outer_circle() {
    let triangle = Triangle::build_triangle_with_edges(3, 4, 5).expect("error");
    let circle = triangle.get_outer_circle();
    assert_eq!(2.5, circle.r);
}

#[test]
fn get_rectangle_diagonal() {
    let rectangle = Rectangle::build_rectangle(4, 9).expect("error");
    assert_eq!(9.848857801796104, rectangle.get_diagonal());
}

#[test]
fn get_rectangle_area() {
    let rectangle = Rectangle::build_rectangle(4, 9).expect("error");
    assert_eq!(36.0, rectangle.get_area());
}

#[test]
fn get_rectangle_perimeter() {
    let rectangle = Rectangle::build_rectangle(4, 9).expect("error");
    assert_eq!(26.0, rectangle.get_perimeter());
}

#[test]
#[should_panic(expected="Rectangle must have a positive length or width")]
fn get_rectangle_panic() {
    Rectangle::build_rectangle(4, -9).expect("error");
}

#[test]
fn get_parallelogram_area() {
    let parallelogram = Parallelogram::build_parallelogram(9,5, 4).expect("error");
    assert_eq!(36.0, parallelogram.get_area());
}

#[test]
fn get_parallelogram_perimeter() {
    let parallelogram = Parallelogram::build_parallelogram(9,5, 4).expect("error");
    assert_eq!(28.0, parallelogram.get_perimeter());
}

#[test]
#[should_panic(expected="Parallelogram must have a positive length, width or height")]
fn get_parallelogram_panic() {
    Parallelogram::build_parallelogram(9,-5, 4).expect("error");
}

#[test]
fn get_trapeze_height() {
    let trapeze = Trapeze::build_trapeze(9.0, 6.0, 4.2, 4.5).expect("error");
    assert_eq!(4.062729993489599, trapeze.get_height());
}

#[test]
fn get_trapeze_area() {
    let trapeze = Trapeze::build_trapeze(9.0, 6.0, 4.2, 4.5).expect("error");
    assert_eq!(30.470474951171994, trapeze.get_area());
}

#[test]
fn get_trapeze_perimeter() {
    let trapeze = Trapeze::build_trapeze(9.0, 6.0, 4.2, 4.5).expect("error");
    assert_eq!(23.7, trapeze.get_perimeter());
}

#[test]
#[should_panic(expected="Trapeze edges must have a positive length")]
fn get_trapeze_panic() {
    Trapeze::build_trapeze(-9.0, 6.0, 4.2, 4.5).expect("error");
}

#[test]
fn get_polygon_area() {
    let polygon = Polygon::build_polygon(2, 8).expect("error");
    assert_eq!(19.31370849898476, polygon.get_area());
}

#[test]
fn get_polygon_perimeter() {
    let polygon = Polygon::build_polygon(2, 8).expect("error");
    assert_eq!(16.0, polygon.get_perimeter());
}

#[test]
fn get_polygon_radius() {
    let polygon = Polygon::build_polygon(2, 8).expect("error");
    assert_eq!(2.613125929752753, polygon.get_radius());
}

#[test]
#[should_panic(expected="Polygon edges must have a positive length")]
fn get_polygon_panic_1() {
    Polygon::build_polygon(-2, 8).expect("error");
}

#[test]
#[should_panic(expected="Polygon must have a positive number of edges")]
fn get_polygon_panic_2() {
    Polygon::build_polygon(2, -8).expect("error");
}

#[test]
fn get_circle_area() {
    let circle = Circle::build_circle(2).expect("error");
    assert_eq!(12.566370614359172, circle.get_area());
}

#[test]
fn get_circle_perimeter() {
    let circle = Circle::build_circle(2).expect("error");
    assert_eq!(12.566370614359172, circle.get_perimeter());
}

#[test]
#[should_panic(expected="Circle radius must be positive")]
fn get_circle_panic() {
    Circle::build_circle(-2).expect("error");
}

#[test]
fn get_ellipsis_area() {
    let ellipsis = Ellipsis::build_ellipsis(2, 3).expect("error");
    assert_eq!(18.84955592153876, ellipsis.get_area());
}

#[test]
fn get_ellipsis_perimeter() {
    let ellipsis = Ellipsis::build_ellipsis(2, 3).expect("error");
    assert_eq!(15.866645920952264, ellipsis.get_perimeter());
}

#[test]
#[should_panic(expected="Ellipsis must have a positive length or width")]
fn get_ellipsis_panic() {
    Ellipsis::build_ellipsis(2, -3).expect("error");
}

#[test]
fn get_cuboid_diagonal_1() {
    let cuboid = Cuboid::build_cuboid(4, 4, 4).expect("error");
    assert_eq!(6.928203230275509, cuboid.get_diagonal());
}

#[test]
fn get_cuboid_diagonal_2() {
    let cuboid = Cuboid::build_cuboid(1, 4, 9).expect("error");
    assert_eq!(9.899494936611665, cuboid.get_diagonal());
}

#[test]
fn get_cuboid_volume_1() {
    let cuboid = Cuboid::build_cuboid(4, 4, 4).expect("error");
    assert_eq!(64.0, cuboid.get_volume());
}

#[test]
fn get_cuboid_volume_2() {
    let cuboid = Cuboid::build_cuboid(1, 4, 9).expect("error");
    assert_eq!(36.0, cuboid.get_volume());
}

#[test]
fn get_cuboid_surface_1() {
    let cuboid = Cuboid::build_cuboid(4, 4, 4).expect("error");
    assert_eq!(96.0, cuboid.get_surface());
}

#[test]
fn get_cuboid_surface_2() {
    let cuboid = Cuboid::build_cuboid(1, 4, 9).expect("error");
    assert_eq!(98.0, cuboid.get_surface());
}

#[test]
#[should_panic(expected="Cuboid must have a positive length, width or height")]
fn get_cuboid_panic() {
    Cuboid::build_cuboid(-1, 4, 9).expect("error");
}

#[test]
fn get_pyramid_volume() {
    let pyramid = Pyramid::build_pyramid(5, 7).expect("error");
    assert_eq!(11.666666666666666, pyramid.get_volume());
}

#[test]
#[should_panic(expected="Pyramid must have a positive area or height")]
fn get_pyramid_panic() {
    Pyramid::build_pyramid(5, -7).expect("error");
}

#[test]
fn get_wedge_volume() {
    let wedge = Wedge::build_wedge(5, 2, 3, 7).expect("error");
    assert_eq!(30.33333333333333, wedge.get_volume());
}

#[test]
#[should_panic(expected="Wedge must have a positive length, width or height")]
fn get_wedge_panic() {
    Wedge::build_wedge(5, 2, -3, 7).expect("error");
}

#[test]
fn get_cylinder_volume() {
    let cylinder = Cylinder::build_cylinder(2, 8).expect("error");
    assert_eq!(100.53096491487338, cylinder.get_volume());
}

#[test]
fn get_cylinder_surface() {
    let cylinder = Cylinder::build_cylinder(2, 8).expect("error");
    assert_eq!(125.66370614359172, cylinder.get_surface());
}

#[test]
fn get_cylinder_lateral() {
    let cylinder = Cylinder::build_cylinder(2, 8).expect("error");
    assert_eq!(100.53096491487338, cylinder.get_lateral());
}

#[test]
#[should_panic(expected="Cylinder must have a positive radius or height")]
fn get_cylinder_panic() {
    Cylinder::build_cylinder(-2, 8).expect("error");
}

#[test]
fn get_cone_volume() {
    let cone = Cone::build_cone(3, 7).expect("error");
    assert_eq!(65.97344572538566, cone.get_volume());
}

#[test]
fn get_cone_surface() {
    let cone = Cone::build_cone(3, 7).expect("error");
    assert_eq!(100.05130440467447, cone.get_surface());
}

#[test]
fn get_cone_lateral() {
    let cone = Cone::build_cone(3, 7).expect("error");
    assert_eq!(71.77697052236633, cone.get_lateral());
}

#[test]
#[should_panic(expected="Cone must have a positive radius or height")]
fn get_cone_panic() {
    Cone::build_cone(3, -7).expect("error");
}

#[test]
fn get_sphere_volume() {
    let sphere = Sphere::build_sphere(4).expect("error");
    assert_eq!(268.082573106329, sphere.get_volume());
}

#[test]
fn get_sphere_surface() {
    let sphere = Sphere::build_sphere(4).expect("error");
    assert_eq!(201.06192982974676, sphere.get_surface());
}

#[test]
#[should_panic(expected="Sphere must have a positive radius")]
fn get_sphere_panic() {
    Sphere::build_sphere(-4).expect("error");
}

#[test]
fn get_ellipsoid_volume() {
    let ellipsoid = Ellipsoid::build_ellipsoid(2, 3, 4).expect("error");
    assert_eq!(100.53096491487338, ellipsoid.get_volume());
}

#[test]
#[should_panic(expected="Ellipsoid must have a positive length, width or height")]
fn get_ellipsoid_panic() {
    Ellipsoid::build_ellipsoid(2, -3, 4).expect("error");
}

#[test]
fn get_spheric_barrel_volume() {
    let barrel = SphericBarrel::build_barrel(4, 2, 6).expect("error");
    assert_eq!(56.548667764616276, barrel.get_volume());
}

#[test]
#[should_panic(expected="Barrel must have a positive radius or height")]
fn get_spheric_barrel_panic() {
    SphericBarrel::build_barrel(-4, 2, 6).expect("error");
}

#[test]
fn get_parabolic_barrel_volume() {
    let barrel = ParabolicBarrel::build_barrel(4, 2, 6).expect("error");
    assert_eq!(54.03539364174444, barrel.get_volume());
}

#[test]
#[should_panic(expected="Barrel must have a positive radius or height")]
fn get_parabolic_barrel_panic() {
    ParabolicBarrel::build_barrel(-4, 2, 6).expect("error");
}

#[test]
fn get_torus_volume() {
    let torus = Torus::build_torus(8, 2).expect("error");
    assert_eq!(157.91367041742973, torus.get_volume());
}

#[test]
fn get_torus_surface() {
    let torus = Torus::build_torus(8, 2).expect("error");
    assert_eq!(315.82734083485946, torus.get_surface());
}

#[test]
#[should_panic(expected="Torus must have a positive radius")]
fn get_torus_panic_1() {
    Torus::build_torus(-8, 2).expect("error");
}

#[test]
#[should_panic(expected="R must be bigger than r")]
fn get_torus_panic_2() {
    Torus::build_torus(2, 8).expect("error");
}

#[test]
fn test_cartesic_to_polar_1() {
    let cart = Vector2D::build_vector(4.0, 3.0);
    let pol = cart.transform_to_polar();
    assert_eq!(5.0, pol.r);
    assert_eq!(36.86989764584402, pol.phi);
}

#[test]
fn test_cartesic_to_polar_2() {
    let cart = Vector2D::build_vector(-4.0, 3.0);
    let pol = cart.transform_to_polar();
    assert_eq!(5.0, pol.r);
    assert_eq!(143.13010235415598, pol.phi);
}

#[test]
fn test_cartesic_to_polar_3() {
    let cart = Vector2D::build_vector(-4.0, -3.0);
    let pol = cart.transform_to_polar();
    assert_eq!(5.0, pol.r);
    assert_eq!(-143.13010235415598, pol.phi);
}

#[test]
fn test_cartesic_to_polar_4() {
    let cart = Vector2D::build_vector(4.0, -3.0);
    let pol = cart.transform_to_polar();
    assert_eq!(5.0, pol.r);
    assert_eq!(-36.86989764584402, pol.phi);
}

#[test]
fn test_polar_to_cartesic_1() {
    let pol = Polar::build_polar(5.0, 36.86989764584402);
    let cart = pol.transform_to_vector2d();
    assert_eq!(4.0, cart.x);
    assert_eq!(3.0, cart.y);
}

#[test]
fn test_polar_to_cartesic_2() {
    let pol = Polar {r: 5.0, phi: 143.13010235415598};
    let cart = pol.transform_to_vector2d();
    assert_eq!(-4.0, cart.x);
    assert_eq!(3.0, cart.y);
}

#[test]
fn test_polar_to_cartesic_3() {
    let pol = Polar {r: 5.0, phi: -143.13010235415598};
    let cart = pol.transform_to_vector2d();
    assert_eq!(-4.0, cart.x);
    assert_eq!(-3.0, cart.y);
}

#[test]
fn test_polar_to_cartesic_4() {
    let pol = Polar {r: 5.0, phi: -36.86989764584402};
    let cart = pol.transform_to_vector2d();
    assert_eq!(4.0, cart.x);
    assert_eq!(-3.0, cart.y);
}

#[test]
fn test_cartesic_to_cylindrical_1() {
    let cart = Vector3D {x: 3.0, y: 4.0, z: 5.0};
    let cyl = cart.transform_to_cylindrical();
    assert_eq!(5.0, cyl.rho);
    assert_eq!(53.13010235415598, cyl.phi);
    assert_eq!(5.0, cyl.z);
}

#[test]
fn test_cartesic_to_cylindrical_2() {
    let cart = Vector3D {x: 0.0, y: 4.0, z: 5.0};
    let cyl = cart.transform_to_cylindrical();
    assert_eq!(4.0, cyl.rho);
    assert_eq!(90.0, cyl.phi);
    assert_eq!(5.0, cyl.z);
}

#[test]
fn test_cartesic_to_cylindrical_3() {
    let cart = Vector3D {x: 0.0, y: -4.0, z: 5.0};
    let cyl = cart.transform_to_cylindrical();
    assert_eq!(4.0, cyl.rho);
    assert_eq!(270.0, cyl.phi);
    assert_eq!(5.0, cyl.z);
}

#[test]
fn test_cylindrical_to_cartesic() {
    let cyl = Cylindrical {rho: 5.0, phi: 53.13010235415598, z: 5.0};
    let cart = cyl.transform_to_vector3d();
    assert_eq!(3.0000000000000004, cart.x);
    assert_eq!(3.9999999999999996, cart.y);
    assert_eq!(5.0, cart.z);
}

#[test]
fn test_cartesic_to_spherical() {
    let cart = Vector3D {x: 3.0, y: 4.0, z: 5.0};
    let sph = cart.transform_to_spherical();
    assert_eq!(7.0710678118654755, sph.r);
    assert_eq!(45.00000000000001, sph.theta);
    assert_eq!(53.13010235415598, sph.phi);
}

#[test]
fn test_spherical_to_cartesic() {
    let sph = Spherical {r: 7.0710678118654755, theta: 45.0, phi: 53.13010235415598};
    let cart = sph.transform_to_vector3d();
    assert_eq!(3.0000000000000004, cart.x);
    assert_eq!(3.9999999999999996, cart.y);
    assert_eq!(5.000000000000001, cart.z);
}

#[test]
fn test_factorial() {
    let zero= 0;
    assert_eq!(1, factorial(zero).expect("error"));
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

//#[test]
//fn test_factorial_panic() {
//    factorial(-1).expect("error");
//}

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

//#[test]
//fn test_permutation_panic_1() {
//    assert_eq!(Err("Sum of parts is not equal to whole"), permutation(5, vec![3, 3]));
//}
//
//#[test]
//fn test_permutation_panic_2() {
//    assert_eq!(Err("Sum of parts is not equal to whole"), permutation(5, vec![1, 3]));
//}
//
//#[test]
//fn test_permutation_panic_3() {
//    assert_eq!(Err("Parameter n must be a positive integer!"), permutation(-1, vec![3, 2]));
//}
//
//#[test]
//fn test_permutation_panic_4() {
//    assert_eq!(Err("Parameter karr is an empty vector!"), permutation(5, vec![]));
//}

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

//#[test]
//fn test_combination_panic_1() {
//    assert_eq!(Err("Number of selections outgrows the number of elements"), combination(10, 11));
//}
//
//#[test]
//fn test_combination_panic_2() {
//    assert_eq!(Err("Parameter n must be a positive integer!"), combination(-1, 2));
//}
//
//#[test]
//fn test_combination_panic_3() {
//    assert_eq!(Err("Parameter k must be a positive integer!"), combination(2, -1));
//}

#[test]
fn test_combination_with_repetition() {
    assert_eq!(220, combination_with_repetition(10, 3).unwrap());
}

#[test]
fn test_combination_with_repetition_2() {
    assert_eq!(35, combination_with_repetition(4, 4).unwrap());
}

//#[test]
//fn test_combination_with_repetition_panic_1() {
//    assert_eq!(Err("Parameter n must be a positive integer!"), combination_with_repetition(-1, 2));
//}
//
//#[test]
//fn test_combination_with_repetition_panic_2() {
//    assert_eq!(Err("Parameter k must be a positive integer!"), combination_with_repetition(2, -1));
//}

#[test]
fn test_variation_1() {
    assert_eq!(336, variation(8, 3).unwrap());
}

#[test]
fn test_variation_2() {
    assert_eq!(40320, variation(8, 8).unwrap());
}

//#[test]
//fn test_variation_panic() {
//    assert_eq!(Err("Number of selections outgrows the number of elements"), variation(8, 9));
//}
//
//#[test]
//fn test_variation_panic_2() {
//    assert_eq!(Err("Parameter n must be a positive integer!"), variation(-1, 2));
//}
//
//#[test]
//fn test_variation_panic_3() {
//    assert_eq!(Err("Parameter k must be a positive integer!"), variation(2, -1));
//}

#[test]
fn test_variation_with_repetition() {
    assert_eq!(125, variation_with_repetition(5, 3).unwrap());
}

//#[test]
//fn test_variation_with_repetition_panic_1() {
//    assert_eq!(Err("Parameter n must be a positive integer!"), variation_with_repetition(-1, 2));
//}
//
//#[test]
//fn test_variation_with_repetition_panic_2() {
//    assert_eq!(Err("Parameter k must be a positive integer!"), variation_with_repetition(2, -1));
//}

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

//#[test]
//fn test_binomial_panic_1() {
//    assert_eq!(Err("p must be in a range between 0 and 1!"), binomial_distribution(6, -0.1));
//}
//
//#[test]
//fn test_binomial_panic_2() {
//    assert_eq!(Err("p must be in a range between 0 and 1!"), binomial_distribution(6, 1.1));
//}
//
//#[test]
//fn test_binomial_panic_3() {
//    assert_eq!(Err("Parameter n must be a positive integer!"), binomial_distribution(0, 0.5));
//}

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

//#[test]
//fn test_hypergeometric_panic_1() {
//    assert_eq!(Err("Parameter M must be smaller than N!"), hypergeometric_distribution(7, 12, 4));
//}
//
//#[test]
//fn test_hypergeometric_panic_2() {
//    assert_eq!(Err("Parameter n must be smaller than N!"), hypergeometric_distribution(12, 7, 14));
//}
//
//#[test]
//fn test_hypergeometric_panic_3() {
//    assert_eq!(Err("Parameter N must be a positive integer!"), hypergeometric_distribution(0, 7, 14));
//}
//
//#[test]
//fn test_hypergeometric_panic_4() {
//    assert_eq!(Err("Parameter M must be a positive integer!"), hypergeometric_distribution(12, 0, 14));
//}
//
//#[test]
//fn test_hypergeometric_panic_5() {
//    assert_eq!(Err("Parameter n must be a positive integer!"), hypergeometric_distribution(12, 7, 0));
//}

#[test]
fn test_poisson_distribution() {
    let my = 1;
    assert_eq!(0.36787944117144233, poisson_distribution(my, 0).unwrap());
    assert_eq!(0.36787944117144233, poisson_distribution(my, 1).unwrap());
    assert_eq!(0.18393972058572117, poisson_distribution(my, 2).unwrap());
    assert_eq!(0.061313240195240384, poisson_distribution(my, 3).unwrap());
    assert_eq!(0.015328310048810096, poisson_distribution(my, 4).unwrap());
}

//#[test]
//fn test_poisson_panic_1() {
//    assert_eq!(Err("Parameter µ must be positive!"), poisson_distribution(-1, 0));
//}
//
//#[test]
//fn test_poisson_panic_2() {
//    assert_eq!(Err("Parameter x must be a positive integer!"), poisson_distribution(5, -1));
//}

#[test]
fn test_gaussian_distribution() {
    assert_eq!(0.10648266850745075, gaussian_distribution(2.0, 3.0, 4.0).unwrap());
    assert_eq!(0.10648266850745075, gaussian_distribution(2, 3, 4).unwrap());
    assert_eq!(0.017996988837729353, gaussian_distribution(2, 3, -4).unwrap());
}

//#[test]
//fn test_gaussian_panic() {
//    assert_eq!(Err("Parameter \u{03c3} must be bigger than 0!"), gaussian_distribution(2, -1, 4));
//}

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

//#[test]
//fn test_exponential_panic() {
//    assert_eq!(Err("Parameter \u{03bb} must be bigger than 0!"), exponential_distribution(-1, 0));
//}

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

//#[test]
//fn test_arithmetic_mean_5() {
//    let vec: Vec<f64> = vec![];
//    assert_eq!(Err("Vector or Array is empty"), get_arithmetic_mean(&vec));
//}

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

//#[test]
//fn test_harmonic_mean_3() {
//    let vec: Vec<f64> = vec![];
//    assert_eq!(Err("Vector or Array is empty"), get_harmonic_mean(&vec));
//}


//#[test]
//fn test_harmonic_mean_4() {
//    let vec = vec![8, 6, 5, 0, 6, 6];
//    assert_eq!(Err("Vector or Array contains zero"), get_harmonic_mean(&vec));
//}

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

//#[test]
//fn test_quadratic_mean_3() {
//    let vec: Vec<f64> = vec![];
//    assert_eq!(Err("Vector or Array is empty"), get_quadratic_mean(&vec));
//}

#[test]
fn test_variance_2() {
    let vec = vec![8, 6, 5, 11, 6, 6];
    assert_eq!(4.8, get_variance(&vec).unwrap());
}

//#[test]
//fn test_variance_3() {
//    let vec: Vec<f64> = vec![];
//    assert_eq!(Err("Vector or Array is empty"), get_variance(&vec));
//}

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

//#[test]
//fn test_standard_deviation_3() {
//    let vec: Vec<i64> = vec![];
//    assert_eq!(Err("Vector or Array is empty"), get_standard_deviation(&vec));
//}

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

//#[test]
//fn test_get_min_3() {
//    let vec: Vec<f64> = vec![];
//    assert_eq!(Err("Vector or Array is empty"), get_min(&vec));
//}

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

//#[test]
//fn test_get_max_3() {
//    let vec: Vec<f64> = vec![];
//    assert_eq!(Err("Vector or Array is empty"), get_min(&vec));
//}

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

//#[test]
//fn test_get_span_3() {
//    let vec: Vec<f64> = vec![];
//    assert_eq!(Err("Vector or Array is empty"), get_span(&vec));
//}

#[test]
fn test_get_length_1() {
    let vec = Vector3D {x: 2, y: 3, z: 4};
    assert_eq!(5.385164807134504, vec.get_length());
}

#[test]
fn test_get_length_2() {
    let vec = Vector3D {x: 2.7, y: 3.6, z: 4.5};
    assert_eq!(6.363961030678928, vec.get_length());
}

#[test]
fn test_get_direction_angle() {
    let vec = Vector3D {x: 4, y: -2, z: 5};
    assert_eq!(0.9319311825594854, vec.get_direction_angle().0);
    assert_eq!(1.873542278417901, vec.get_direction_angle().1);
    assert_eq!(0.7297276562269663, vec.get_direction_angle().2);
}

#[test]
fn test_multiply_with_scalar_1() {
    let vec = Vector3D {x: 2, y: 3, z: 4};
    assert_eq!(4, vec.multiply_with_scalar(2).x);
    assert_eq!(6, vec.multiply_with_scalar(2).y);
    assert_eq!(8, vec.multiply_with_scalar(2).z);
}

#[test]
fn test_multiply_with_scalar_2() {
    let vec = Vector3D {x: 2.7, y: 3.6, z: 4.5};
    assert_eq!(5.4, vec.multiply_with_scalar(2.0).x);
    assert_eq!(7.2, vec.multiply_with_scalar(2.0).y);
    assert_eq!(9.0, vec.multiply_with_scalar(2.0).z);
}

#[test]
fn test_get_scalar_product_1() {
    let a = Vector3D {x: 2, y: 3, z: 4};
    let b = Vector3D {x: 5, y: 6, z: 7};
    assert_eq!(56, a.get_scalar_product(b));
}

#[test]
fn test_get_scalar_product_2() {
    let a = Vector3D {x: 2.7, y: 3.6, z: 4.5};
    let b = Vector3D {x: 5.4, y: 6.3, z: 7.2};
    assert_eq!(69.66, a.get_scalar_product(b));
}

#[test]
fn test_get_cut_angle_1() {
    let a = Vector3D {x: 1, y: 2, z: -3};
    let b = Vector3D {x: 5, y: -1, z: -5};
    assert_eq!(0.6736330697086078, a.get_cut_angle(b));
}

#[test]
fn test_get_vector_product() {
    let a = Vector3D {x: 1, y: 4, z: 0};
    let b = Vector3D {x: -2, y: 5, z: 3};
    assert_eq!(12, a.get_vector_product(b).x);
    assert_eq!(-3, a.get_vector_product(b).y);
    assert_eq!(13, a.get_vector_product(b).z);
}

#[test]
fn test_get_triple_product() {
    let a = Vector3D {x: 1, y: -2, z: 4};
    let b = Vector3D {x: 4, y: 1, z: 2};
    let c = Vector3D {x: -2, y: -5, z: 6};
    assert_eq!(0, a.get_triple_product(b, c));
}

#[test]
fn test_build_line_from_two_points() {
    let p = Vector3D::build_vector(-1, 5, 0);
    let q = Vector3D::build_vector(1, -3, 2);
    let line = Line3D::build_line_from_two_points(p, q);
    assert_eq!(-1, line.r.x);
    assert_eq!(5, line.r.y);
    assert_eq!(0, line.r.z);
    assert_eq!(2, line.a.x);
    assert_eq!(-8, line.a.y);
    assert_eq!(2, line.a.z);
}

#[test]
fn test_distance_from_point() {
    let p = Vector3D {x: 1, y: 5, z: 3};
    let l = Line3D {r: Vector3D {x: 1, y: 1, z: 4}, a: Vector3D {x: 2, y: -3, z: 5}};
    assert_eq!(3.0650834967591445, l.distance_from_point(p));
}

#[test]
fn test_are_parallel() {
    let l1 = Line3D {r: Vector3D {x: 1, y: 0, z: 5}, a: Vector3D {x: 2, y: 1, z: 1}};
    let l2 = Line3D {r: Vector3D {x: 0, y: 2, z: 1}, a: Vector3D {x: 2, y: 1, z: 1}};
    assert_eq!(true, l1.are_parallel(l2));
}

#[test]
fn test_distance_from_line_1() {
    let l1 = Line3D {r: Vector3D {x: 1, y: 0, z: 5}, a: Vector3D {x: 2, y: 1, z: 1}};
    let l2 = Line3D {r: Vector3D {x: 0, y: 2, z: 1}, a: Vector3D {x: 2, y: 1, z: 1}};
    assert_eq!(Ok(4.281744192888377), l1.distance_from_line(l2));
}

#[test]
fn test_distance_from_line_2() {
    let l1 = Line3D {r: Vector3D {x: 5, y: 2, z: 1}, a: Vector3D {x: 1, y: 1, z: 3}};
    let l2 = Line3D {r: Vector3D {x: 2, y: -1, z: 0}, a: Vector3D {x: 3, y: 2, z: 1}};
    assert_eq!(Ok(0.8432740427115678), l1.distance_from_line(l2));
}

//#[test]
//fn test_distance_from_line_3() {
//    let l1 = Line3D {r: Vector3D {x: 1, y: 1, z: 0}, a: Vector3D {x: 2, y: 1, z: 1}};
//    let l2 = Line3D {r: Vector3D {x: 2, y: 0, z: 2}, a: Vector3D {x: 1, y: -1, z: 2}};
//    assert_eq!(Err("Lines do cross"), l1.distance_from_line(&l2));
//}

#[test]
fn test_do_cross() {
    let l1 = Line3D {r: Vector3D {x: 1, y: 1, z: 0}, a: Vector3D {x: 2, y: 1, z: 1}};
    let l2 = Line3D {r: Vector3D {x: 2, y: 0, z: 2}, a: Vector3D {x: 1, y: -1, z: 2}};
    assert_eq!(true, l1.do_cross(l2));
}

#[test]
fn test_are_skew() {
    let l1 = Line3D {r: Vector3D {x: 5, y: 2, z: 1}, a: Vector3D {x: 1, y: 1, z: 3}};
    let l2 = Line3D {r: Vector3D {x: 2, y: -1, z: 0}, a: Vector3D {x: 3, y: 2, z: 1}};
    assert_eq!(true, l1.are_skew(l2));
}

#[test]
fn build_plane_from_three_points() {
    let p = Vector3D {x: 1, y: 1, z: 2};
    let q = Vector3D {x: 0, y: 4, z: -5};
    let r = Vector3D {x: -3, y: 4, z: 9};
    let vec = Plane::build_plane_from_three_points(p, q, r);
    assert_eq!(1, vec.r.x);
    assert_eq!(1, vec.r.y);
    assert_eq!(2, vec.r.z);
    assert_eq!(42, vec.n.x);
    assert_eq!(35, vec.n.y);
    assert_eq!(9, vec.n.z);
}

#[test]
fn test_get_distance_from_point() {
    let r = Vector3D::build_vector(3, 1, 8);
    let n = Vector3D::build_vector(-1, 5, 3);
    let plane = Plane::build_plane_with_vectors(r, n);
    let q = Vector3D::build_vector(1, 2, 0);
    assert_eq!(2.8735244660769563, plane.get_distance_from_point(q));
}

#[test]
fn test_is_plane_parallel_to_line() {
    let l = Line3D {r: Vector3D {x: 0, y: 7, z: -3}, a: Vector3D {x: 2, y: -1, z: -1}};
    let p = Plane {r: Vector3D {x: 1, y: 3, z: 2}, n: Vector3D {x: 2, y: -1, z: 5}};
    assert_eq!(true, p.is_parallel_to_line(l));
}

#[test]
fn test_get_distance_of_plane_to_line() {
    let l = Line3D {r: Vector3D {x: 0, y: 7, z: -3}, a: Vector3D {x: 2, y: -1, z: -1}};
    let p = Plane {r: Vector3D {x: 1, y: 3, z: 2}, n: Vector3D {x: 2, y: -1, z: 5}};
    assert_eq!(5.659799760886717, p.get_distance_from_line(l).expect("error"));
}

//#[test]
//fn test_get_distance_of_plane_to_line_panic() {
//    let l = Line3D {r: Vector3D {x: 1, y: 2, z: 3}, a: Vector3D {x: 4, y: 2, z: 3}};
//    let p = Plane {r: Vector3D {x: 2, y: 3, z: 5}, a: Vector3D {x: 2, y: 1, z: 1}, b: Vector3D {x: 1, y: 3, z: 4}};
//    assert_eq!(Err("Line is not parallel to plane"), p.get_distance_from_line(&l));
//}

#[test]
fn test_is_plane_parallel_to_plane() {
    let p = Plane {r: Vector3D {x: 3, y: 1, z: -2}, n: Vector3D {x: 2, y: -1, z: 4}};
    let q = Plane {r: Vector3D {x: -4, y: 3, z: 0}, n: Vector3D {x: -4, y: 2, z: -8}};
    assert_eq!(true, p.is_parallel_to_plane(q));
}

#[test]
fn test_get_distance_of_plane_to_plane() {
    let p = Plane {r: Vector3D {x: 3, y: 1, z: -2}, n: Vector3D {x: 2, y: -1, z: 4}};
    let q = Plane {r: Vector3D {x: -4, y: 3, z: 0}, n: Vector3D {x: -4, y: 2, z: -8}};
    assert_eq!(1.7457431218879391, p.get_distance_from_plane(q).expect("error"));
}

//#[test]
//fn test_get_distance_of_plane_to_plane_panic() {
//    let p = Plane {r: Vector3D {x: 2, y: 3, z: 5}, a: Vector3D {x: 2, y: 1, z: 1}, b: Vector3D {x: 1, y: 3, z: 4}};
//    let q = Plane {r: Vector3D {x: 4, y: 3, z: 7}, a: Vector3D {x: 4, y: 2, z: 3}, b: Vector3D {x: 2, y: 6, z: 8}};
//    assert_eq!(Err("The planes are not parallel"), p.get_distance_from_plane(&q));
//}

#[test]
fn test_cutting_point_of_line_with_plane() {
    let l = Line3D::build_line_from_two_points(Vector3D::build_vector(2, 0, 5), Vector3D::build_vector(5, -4, 4));
    let p = Plane::build_plane_with_vectors(Vector3D::build_vector(1, 1, 2), Vector3D::build_vector(2, 1, 1));
    let s = p.get_cutting_point_with_line(l).expect("error");
    assert_eq!(-10, s.x);
    assert_eq!(16, s.y);
    assert_eq!(9, s.z);
}

#[test]
#[should_panic(expected="The line is parallel to the plane")]
fn test_cutting_point_of_line_with_plane_panic() {
    let l = Line3D {r: Vector3D {x: 0, y: 7, z: -3}, a: Vector3D {x: 2, y: -1, z: -1}};
    let p = Plane {r: Vector3D {x: 1, y: 3, z: 2}, n: Vector3D {x: 2, y: -1, z: 5}};
    p.get_cutting_point_with_line(l).expect("error");
}

#[test]
fn test_add_vector2d() {
    let vec_1 = Vector2D::build_vector(1,2);
    let vec_2 = Vector2D::build_vector(3,4);
    let vec_3 = vec_1.add_vector(vec_2);
    assert_eq!(4, vec_3.x);
    assert_eq!(6, vec_3.y);
}

#[test]
fn test_sub_vector2d() {
    let vec_1 = Vector2D::build_vector(1,2);
    let vec_2 = Vector2D::build_vector(3,4);
    let vec_3 = vec_1.sub_vector(vec_2);
    assert_eq!(-2, vec_3.x);
    assert_eq!(-2, vec_3.y);
}

#[test]
fn test_get_length_vector2d() {
    let vec = Vector2D::build_vector(6,8);
    assert_eq!(10.0, vec.get_length());
}

#[test]
fn test_multiply_with_scalar_vector2d() {
    let vec = Vector2D::build_vector(4, -3);
    let res = vec.multiply_with_scalar(6);
    assert_eq!(24, res.x);
    assert_eq!(-18, res.y);
}

#[test]
fn test_get_scalar_product_2d_1() {
    let vec_1 = Vector2D::build_vector(3, 2);
    let vec_2 = Vector2D::build_vector(-1, 5);
    assert_eq!(7, vec_1.get_scalar_product(vec_2));
}

#[test]
fn test_get_scalar_product_2d_2() {
    let vec_1 = Vector2D::build_vector(1, 1);
    let vec_2 = Vector2D::build_vector(-1, 1);
    assert_eq!(0, vec_1.get_scalar_product(vec_2));
}

#[test]
fn test_get_cut_angle_vector2d_1() {
    let vec_1 = Vector2D::build_vector(2, 1);
    let vec_2 = Vector2D::build_vector(1, 0);
    let vec_3 = Vector2D::build_vector(0, 1);
    assert_eq!(0.46364760900080615, vec_1.get_cut_angle(vec_2));
    assert_eq!(1.1071487177940904, vec_1.get_cut_angle(vec_3));
}

#[test]
fn test_get_cut_angle_vector2d_2() {
    let vec_1 = Vector2D::build_vector(4, 3);
    let vec_2 = Vector2D::build_vector(-3, 2);
    assert_eq!(1.9100889412489412, vec_1.get_cut_angle(vec_2));
}

#[test]
fn test_build_empty_matrice() {
    let matrice = Matrice::build_empty_matrice(3, 3);
    let vec = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(vec, matrice.data);
}

#[test]
fn test_build_matrice() {
    let matrice = Matrice::build_matrice(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], matrice.data);
}

//#[test]
//fn test_build_matrice_panic_1() {
//    assert_eq!(Err("Vector is not the same length as the product of rows and columns"), Matrice::build_matrice(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8]));
//}

//#[test]
//fn test_build_matrice_panic_2() {
//    assert_eq!(Err("Vector is not the same length as the product of rows and columns"), Matrice::build_matrice(3, 3, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
//}

#[test]
fn test_get_element() {
    let matrice = Matrice::build_matrice(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    assert_eq!(Ok(1), matrice.get_element(0, 0));
    assert_eq!(Ok(2), matrice.get_element(0, 1));
    assert_eq!(Ok(3), matrice.get_element(0, 2));
    assert_eq!(Ok(4), matrice.get_element(1, 0));
    assert_eq!(Ok(5), matrice.get_element(1, 1));
    assert_eq!(Ok(6), matrice.get_element(1, 2));
    assert_eq!(Ok(7), matrice.get_element(2, 0));
    assert_eq!(Ok(8), matrice.get_element(2, 1));
    assert_eq!(Ok(9), matrice.get_element(2, 2));
}

//#[test]
//fn test_get_element_panic() {
//    let matrice = Matrice::build_matrice(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
//    assert_eq!(Err("Row is out of bounds"), matrice.get_element(3, 2));
//    assert_eq!(Err("Column is out of bounds"), matrice.get_element(2, 3));
//}

#[test]
fn test_get_trace() {
    let matrice = Matrice::build_matrice(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    assert_eq!(Ok(15), matrice.get_trace());
}

//#[test]
//fn test_get_trace_panic_1() {
//    let matrice = Matrice::build_matrice(3, 4, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]).unwrap();
//    assert_eq!(Err("The matrice is not quadratic"), matrice.get_trace());
//}

//#[test]
//fn test_get_trace_panic_2() {
//    let matrice = Matrice::build_matrice(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
//    assert_eq!(Err("The matrice is not quadratic"), matrice.get_trace());
//}

#[test]
fn test_add_matrices() {
    let m1 = Matrice::build_matrice(2, 3, vec![1, 5, -3, 4, 0, 8]).unwrap();
    let m2 = Matrice::build_matrice(2, 3, vec![5, 1, 3, -1, 4, 7]).unwrap();
    let m3 = m1.add_matrice(&m2).unwrap();
    assert_eq!(vec![6, 6, 0, 3, 4, 15], m3.data);
}

//#[test]
//fn test_add_matrices_panic_1() {
//    let m1 = Matrice::build_matrice(2, 3, vec![1, 5, -3, 4, 0, 8]).unwrap();
//    let m2 = Matrice::build_matrice(2, 2, vec![5, 1, 3, -1]).unwrap();
//    assert_eq!(Err("The two matrices do not have the same number of rows or columns"), m1.add_matrice(&m2))
//}

//#[test]
//fn test_add_matrices_panic_2() {
//    let m1 = Matrice::build_matrice(3, 3, vec![1, 5, -3, 4, 0, 8, 1, 2, 3]).unwrap();
//    let m2 = Matrice::build_matrice(2, 3, vec![5, 1, 3, -1, 4, 7]).unwrap();
//    assert_eq!(Err("The two matrices do not have the same number of rows or columns"), m1.add_matrice(&m2))
//}

#[test]
fn test_subtract_matrices() {
    let m1 = Matrice::build_matrice(2, 3, vec![1, 5, -3, 4, 0, 8]).unwrap();
    let m2 = Matrice::build_matrice(2, 3, vec![5, 1, 3, -1, 4, 7]).unwrap();
    let m3 = m1.subtract_matrice(&m2).unwrap();
    assert_eq!(vec![-4, 4, -6, 5, -4, 1], m3.data);
}

//#[test]
//fn test_subtract_matrices_panic_1() {
//    let m1 = Matrice::build_matrice(2, 3, vec![1, 5, -3, 4, 0, 8]).unwrap();
//    let m2 = Matrice::build_matrice(2, 2, vec![5, 1, 3, -1]).unwrap();
//    assert_eq!(Err("The two matrices do not have the same number of rows or columns"), m1.subtract_matrice(&m2))
//}

//#[test]
//fn test_subtract_matrices_panic_2() {
//    let m1 = Matrice::build_matrice(3, 3, vec![1, 5, -3, 4, 0, 8, 1, 2, 3]).unwrap();
//    let m2 = Matrice::build_matrice(2, 3, vec![5, 1, 3, -1, 4, 7]).unwrap();
//    assert_eq!(Err("The two matrices do not have the same number of rows or columns"), m1.subtract_matrice(&m2))
//}

#[test]
fn test_multiply_with_scalar() {
    let matrice = Matrice::build_matrice(2, 3, vec![1, -5, 3, 4, 1, 0]).unwrap();
    assert_eq!(vec![4, -20, 12, 16, 4, 0], matrice.multiply_with_scalar(4).data);
    assert_eq!(vec![-3, 15, -9, -12, -3, 0], matrice.multiply_with_scalar(-3).data);
}

#[test]
fn test_get_row() {
    let a = Matrice::build_matrice(3, 3, vec![1, 4, -2, 0, 1, 1, -3, 2, 5]).unwrap();
    assert_eq!(Ok(vec![1, 4, -2]), a.get_row(0));
    assert_eq!(Ok(vec![0, 1, 1]), a.get_row(1));
    assert_eq!(Ok(vec![-3, 2, 5]), a.get_row(2));
}

//#[test]
//fn test_get_row_panic() {
//    let a = Matrice::build_matrice(3, 3, vec![1, 4, -2, 0, 1, 1, -3, 2, 5]).unwrap();
//    assert_eq!(Err("Row is out of bounds"), a.get_row(3));
//}

#[test]
fn test_get_column() {
    let b = Matrice::build_matrice(3, 3, vec![3, 0, 1, -2, 1, 5, 2, 3, 8]).unwrap();
    assert_eq!(Ok(vec![3, -2, 2]), b.get_column(0));
    assert_eq!(Ok(vec![0, 1, 3]), b.get_column(1));
    assert_eq!(Ok(vec![1, 5, 8]), b.get_column(2));
}

//#[test]
//fn test_get_column_panic() {
//    let b = Matrice::build_matrice(3, 3, vec![3, 0, 1, -2, 1, 5, 2, 3, 8]).unwrap();
//    assert_eq!(Err("Column is out of bounds"), b.get_column(3));
//}

#[test]
fn test_multiply_with_matrice_1() {
    let a = Matrice::build_matrice(3, 3, vec![1, 4, -2, 0, 1, 1, -3, 2, 5]).unwrap();
    let b = Matrice::build_matrice(3, 3, vec![3, 0, 1, -2, 1, 5, 2, 3, 8]).unwrap();
    assert_eq!(vec![-9, -2, 5, 0, 4, 13, -3, 17, 47], a.multiply_with_matrice(&b).unwrap().data);
}

#[test]
fn test_multiply_with_matrice_2() {
    let a = Matrice::build_matrice(2, 3, vec![1, -3, 2, 0, 2, 1]).unwrap();
    let b = Matrice::build_matrice(3, 1, vec![1, 5, 4]).unwrap();
    assert_eq!(vec![-6, 14], a.multiply_with_matrice(&b).unwrap().data);
}

//#[test]
//fn test_multiply_with_matrice_panic() {
//    let a = Matrice::build_matrice(3, 1, vec![1, 5, 4]).unwrap();
//    let b = Matrice::build_matrice(3, 3, vec![3, 0, 1, -2, 1, 5, 2, 3, 8]).unwrap();
//    assert_eq!(Err("Matrix A must have the same number of columns as Matrix B has number of rows"), a.multiply_with_matrice(&b));
//}

#[test]
fn test_get_main_diagonal_product() {
    let m = Matrice::build_matrice(3, 3, vec![1, -2, 3, 2, 0, 1, 6, 5, 1]).unwrap();
    assert_eq!(Ok(0), m.get_main_diagonal_product(0));
    assert_eq!(Ok(-12), m.get_main_diagonal_product(1));
    assert_eq!(Ok(30), m.get_main_diagonal_product(2));
}

//#[test]
//fn test_get_main_diagonal_product_panic() {
//    let m = Matrice::build_matrice(3, 3, vec![1, -2, 3, 2, 0, 1, 6, 5, 1]).unwrap();
//    assert_eq!(Err("Column is out of bounds"), m.get_main_diagonal_product(3));
//}

#[test]
fn test_get_side_diagonal_product() {
    let m = Matrice::build_matrice(3, 3, vec![1, -2, 3, 2, 0, 1, 6, 5, 1]).unwrap();
    assert_eq!(Ok(0), m.get_side_diagonal_product(0));
    assert_eq!(Ok(5), m.get_side_diagonal_product(1));
    assert_eq!(Ok(-4), m.get_side_diagonal_product(2));
}

//#[test]
//fn test_get_side_diagonal_product_panic() {
//    let m = Matrice::build_matrice(3, 3, vec![1, -2, 3, 2, 0, 1, 6, 5, 1]).unwrap();
//    assert_eq!(Err("Column is out of bounds"), m.get_side_diagonal_product(3));
//}

#[test]
fn test_get_determinant_1() {
    let m = Matrice::build_matrice(2, 2, vec![4, 7, -3, 8]).unwrap();
    assert_eq!(Ok(53), m.get_determinant());
}

#[test]
fn test_get_determinant_2() {
    let m = Matrice::build_matrice(3, 3, vec![1, -2, 3, 2, 0, 1, 6, 5, 1]).unwrap();
    assert_eq!(Ok(17), m.get_determinant());
}

#[test]
fn test_get_determinant_3() {
    let m = Matrice::build_matrice(4, 4, vec![1, 2, 0, -1, 4, 0, -3, 2, 9, 0, 0, 4, 8, 1, 3, 1]).unwrap();
    assert_eq!(Ok(87), m.get_determinant());
}

#[test]
fn test_get_determinant_5() {
    let m = Matrice::build_matrice(3, 3, vec![1.0, -2.0, 3.0, 2.0, 0.0, 1.0, 6.0, 5.0, 1.0]).unwrap();
    assert_eq!(Ok(17.0), m.get_determinant());
}

#[test]
fn test_get_determinant_4() {
    let m = Matrice::build_matrice(5, 5, vec![-1, 1, 0, -2, 0,
                                                                0, 2, 1, -1, 4,
                                                                1, 0, 0, -3, 1,
                                                                1, 2, 0, 0, 3,
                                                                0, -2, 1, 2, 2]).unwrap();
    assert_eq!(Ok(26), m.get_determinant());
}

//#[test]
//fn test_get_determinant_panic() {
//    let m = Matrice::build_matrice(2, 3, vec![4, 7, -3, 8, 1, 2]).unwrap();
//    assert_eq!(Err("The matrice is not quadratic"), m.get_determinant());
//}

#[test]
fn get_submatrice_1() {
    let m = Matrice::build_matrice(4, 4, vec![1, 2, 0, -1, 4, 0, -3, 2, 9, 0, 0, 4, 8, 1, 3, 1]).unwrap();
    assert_eq!(vec![2, 0, -1, 0, -3, 2, 1, 3, 1], m.get_submatrice(2, 0).data);
}

#[test]
fn get_submatrice_2() {
    let m = Matrice::build_matrice(4, 4, vec![1, 2, 0, -1, 4, 0, -3, 2, 9, 0, 0, 4, 8, 1, 3, 1]).unwrap();
    assert_eq!(vec![1, 2, 0, 4, 0, -3, 8, 1, 3], m.get_submatrice(2, 3).data);
}

#[test]
fn test_get_inverse_matrice_1() {
    let m = Matrice::build_matrice(3, 3, vec![1, 0, -1, -8, 4, 1, -2, 1, 0]).unwrap();
    assert_eq!(vec![1, 1, -4, 2, 2, -7, 0, 1, -4], m.get_inverse_matrice().unwrap().data);
}

#[test]
fn test_get_inverse_matrice_2() {
    let m = Matrice::build_matrice(3, 3, vec![1.0, 0.0, -1.0, -8.0, 4.0, 1.0, -2.0, 1.0, 0.0]).unwrap();
    assert_eq!(vec![1.0, 1.0, -4.0, 2.0, 2.0, -7.0, 0.0, 1.0, -4.0], m.get_inverse_matrice().unwrap().data);
}

//#[test]
//fn test_get_inverse_matrice_panic() {
//    let m = Matrice::build_matrice(2, 3, vec![4, 7, -3, 8, 1, 2]).unwrap();
//    assert_eq!(Err("The matrice is not quadratic"), m.get_inverse_matrice());
//}

#[test]
fn test_insert_row() {
    let mut m = Matrice::build_matrice(2, 2, vec![1, 2, 3, 4]).unwrap();
    m.insert_row(&vec![5, 6]).unwrap();
    assert_eq!(3, m.rows);
    assert_eq!(vec![1, 2, 3, 4, 5, 6], m.data);
}

#[test]
fn test_insert_columns() {
    let mut m = Matrice::build_matrice(2, 2, vec![1, 2, 3, 4]).unwrap();
    m.insert_column(&vec![5, 6]).unwrap();
    assert_eq!(3, m.columns);
    assert_eq!(vec![1, 2, 5, 3, 4, 6], m.data);
}

//#[test]
//fn test_cut_matrices_1() {
//    let m = Matrice::build_matrice(2, 3, vec![2, 3, 1, 0, 4, 2]).unwrap();
//    let vec = m.cut_matrices(2);
//    assert_eq!(vec![2, 3, 0, 4], vec[0].data);
//    assert_eq!(vec![3, 1, 4, 2], vec[1].data);
//}
//
//#[test]
//fn test_cut_matrices_2() {
//    let m = Matrice::build_matrice(3, 2, vec![2, 3, 1, 0, 4, 2]).unwrap();
//    let vec = m.cut_matrices(2);
//    assert_eq!(vec![2, 3, 1, 0], vec[0].data);
//    assert_eq!(vec![1, 0, 4, 2], vec[1].data);
//}
//
//#[test]
//fn test_cut_matrices_3() {
//    let m = Matrice::build_matrice(2, 3, vec![2, 3, 1, 0, 4, 2]).unwrap();
//    let vec = m.cut_matrices(1);
//    assert_eq!(vec![2], vec[0].data);
//    assert_eq!(vec![3], vec[1].data);
//    assert_eq!(vec![1], vec[2].data);
//    assert_eq!(vec![0], vec[3].data);
//    assert_eq!(vec![4], vec[4].data);
//    assert_eq!(vec![2], vec[5].data);
//}
//
//#[test]
//fn test_cut_matrices_4() {
//    let m = Matrice::build_matrice(3, 2, vec![2, 3, 1, 0, 4, 2]).unwrap();
//    let vec = m.cut_matrices(1);
//    assert_eq!(vec![2], vec[0].data);
//    assert_eq!(vec![3], vec[1].data);
//    assert_eq!(vec![1], vec[2].data);
//    assert_eq!(vec![0], vec[3].data);
//    assert_eq!(vec![4], vec[4].data);
//    assert_eq!(vec![2], vec[5].data);
//}
//
//#[test]
//fn test_cut_matrices_5() {
//    let m = Matrice::build_matrice(3, 4, vec![3, 9, 7, 8, 4, 9, 5, 5, 2, 3, 8, 7]).unwrap();
//    let s3 = m.cut_matrices(3);
//    assert_eq!(vec![3, 9, 7, 4, 9, 5, 2, 3, 8], s3[0].data);
//    assert_eq!(vec![9, 7, 8, 9, 5, 5, 3, 8, 7], s3[1].data);
//    let s2 = m.cut_matrices(2);
//    assert_eq!(vec![3, 9, 4, 9], s2[0].data);
//    assert_eq!(vec![9, 7, 9, 5], s2[1].data);
//    assert_eq!(vec![7, 8, 5, 5], s2[2].data);
//    assert_eq!(vec![4, 9, 2, 3], s2[3].data);
//    assert_eq!(vec![9, 5, 3, 8], s2[4].data);
//    assert_eq!(vec![5, 5, 8, 7], s2[5].data);
//    let s1 = m.cut_matrices(1);
//    assert_eq!(vec![3], s1[0].data);
//    assert_eq!(vec![9], s1[1].data);
//    assert_eq!(vec![7], s1[2].data);
//    assert_eq!(vec![8], s1[3].data);
//    assert_eq!(vec![4], s1[4].data);
//    assert_eq!(vec![9], s1[5].data);
//    assert_eq!(vec![5], s1[6].data);
//    assert_eq!(vec![5], s1[7].data);
//    assert_eq!(vec![2], s1[8].data);
//    assert_eq!(vec![3], s1[9].data);
//    assert_eq!(vec![8], s1[10].data);
//    assert_eq!(vec![7], s1[11].data);
//}

#[test]
fn test_get_rank_1() {
    let m = Matrice::build_matrice(2, 3, vec![2, 3, 1, 0, 4, 2]).unwrap();
    assert_eq!(Ok(2), m.get_rank());
}

#[test]
fn test_get_rank_2() {
    let m = Matrice::build_matrice(3, 4, vec![1, 3, -5, 0, 2, 7, -8, 7, -1, 0, 11, 21]).unwrap();
    assert_eq!(Ok(2), m.get_rank());
}

#[test]
fn test_get_rank_3() {
    let m = Matrice::build_matrice(3, 4, vec![1, 1, 1, 0, 2, -1, 1, 3, 1, -2, 0, 3]).unwrap();
    assert_eq!(Ok(2), m.get_rank());
}

#[test]
fn test_get_rank_4() {
    let m = Matrice::build_matrice(2, 2, vec![1, 3, 0, 3]).unwrap();
    assert_eq!(Ok(2), m.get_rank());
}

#[test]
fn test_is_solvable_1() {
    let m = Matrice::build_matrice(2, 3, vec![1, -2, 1, 1, 1, -4]).unwrap();
    let c = vec![1, 8];
    assert_eq!(Solvable::InfiniteSolutions, m.is_solvable(&c));
}

#[test]
fn test_is_solvable_2() {
    let m = Matrice::build_matrice(3, 2, vec![1, 2, 5, 9, 2, -3]).unwrap();
    let c = vec![4, 9, -10];
    assert_eq!(Solvable::NoSolution, m.is_solvable(&c));
}

#[test]
fn test_is_solvable_3() {
    let m = Matrice::build_matrice(3, 3, vec![1, -2, 1, 2, 1, -1, -1, -4, 3]).unwrap();
    let c = vec![6, -3, 14];
    assert_eq!(Solvable::OneSolution, m.is_solvable(&c));
}

//#[test]
//fn test_shuffle() {
//    let m = Matrice::build_matrice(4, 4, vec![0, 2, 4, 3, 3, 1, 0, 2, 0, 0, 0, 1, 0, 0, 2, 2]).unwrap();
//    let (vecs, c) = m.shuffle(&vec![1, 2, 3, 4]);
//    assert_eq!(vec![vec![3, 1, 0, 2], vec![0, 2, 4, 3], vec![0, 0, 2, 2], vec![0, 0, 0, 1]], vecs);
//    assert_eq!(vec![2, 1, 4, 3], c);
//}

#[test]
fn test_solve_1() {
    let m = Matrice::build_matrice(4, 4, vec![2, 1, 4, 3, -1, 2, 1, -1, 3, 4, -1, -2, 4, 3, 2, 1]).unwrap();
    let (vecs, c) = m.solve(&vec![0, 4, 0, 0]).unwrap();
    assert_eq!(vec![vec![1.0, 0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0, 0.0], vec![0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 0.0, 1.0]], vecs);
    assert_eq!(vec![2.0, -4.0, 6.0, -8.0], c);
}

//#[test]
//fn test_solve_4() {
//    let m = Matrice::build_matrice(4, 4, vec![0, 3, -5, 1, 1, -3, 0, -1, -2, 1, 2, 2, -3, 4, 2, 2]).unwrap();
//    let (vecs, c) = m.solve(&vec![0, -5, 2, 8]).unwrap();
//    assert_eq!(vec![vec![1.0, 0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0, 0.0], vec![0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 0.0, 1.0]], vecs);
//    assert_eq!(vec![0.0, 2.0, 1.0, -1.0], c);
//}

//#[test]
//fn test_solve_2() {
//    let m = Matrice::build_matrice(4, 4, vec![1.0, -3.0, 1.5, -1.0, -2.0, 1.0, 3.5, 2.0, 1.0, -2.0, 1.2, 2.0, 3.0, 1.0, -1.0, -3.0]).unwrap();
//    let (vecs, c) = m.solve(&vec![-10.4, -16.5, 0.0, -0.7]).unwrap();
//    assert_eq!(vec![vec![1.0, 0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0, 0.0], vec![0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 0.0, 1.0]], vecs);
//    assert_eq!(vec![0.808, -0.184, -5.88, 2.94], c);
//}

//#[test]
//fn test_solve_3() {
//    let m = Matrice::build_matrice(3, 3, vec![1, 1, -2, 1, -1, -2, 2, 3, -4]).unwrap();
//    assert_eq!(Err("The linear system is not solvable or it has infinite solutions"), m.solve(&vec![0, 0, 0]));
//}