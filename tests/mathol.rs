extern crate mathol;
use mathol::geometrics::planimetry::{Planimetry, Triangulum, Rectangle, Parallelogram, Trapeze, Circle};

#[test]
fn test_triangulum_area() {
    let triangulum = Triangulum {a: 3.0, b: 4.0, c: 5.0};
    assert_eq!(6.0, triangulum.get_area());
}

#[test]
fn test_triangulum_perimeter() {
    let triangulum = Triangulum {a: 3.0, b: 4.0, c: 5.0};
    assert_eq!(12.0, triangulum.get_perimeter());
}

#[test]
fn test_rectangle_area() {
    let rectangle = Rectangle {a: 4.0, b: 7.0};
    assert_eq!(28.0, rectangle.get_area());
}

#[test]
fn test_rectangle_perimeter() {
    let rectangle = Rectangle {a: 4.0, b: 7.0};
    assert_eq!(22.0, rectangle.get_perimeter());
}

#[test]
fn test_parallelogram_area() {
    let parallelogram = Parallelogram {a: 6.0, b: 3.5, h: 2.0};
    assert_eq!(12.0, parallelogram.get_area());
}

#[test]
fn test_parallelogram_perimeter() {
    let parallelogram = Parallelogram {a: 6.0, b: 3.5, h: 2.0};
    assert_eq!(19.0, parallelogram.get_perimeter());
}

#[test]
fn test_trapeze_area() {
    let trapeze = Trapeze {a: 5.0, b: 3.0, h: 2.5};
    assert_eq!(10.0, trapeze.get_area());
}

#[test]
fn test_circle_area() {
    let circle = Circle {r: 4.0};
    assert_eq!(50.26548245743669, circle.get_area());
}

#[test]
fn test_circle_perimeter() {
    let circle = Circle {r: 4.0};
    assert_eq!(25.132741228718345, circle.get_perimeter());
}