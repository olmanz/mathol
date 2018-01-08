extern crate mathol;
use mathol::basic;

#[test]
fn test_pow_1() {
    assert_eq!(4, basic::pow(2, 2));
}

#[test]
fn test_pow_2() {
    assert_eq!(8, basic::pow(2, 3));
}

#[test]
fn test_pow_3() {
    assert_eq!(9, basic::pow(3, 2));
}

#[test]
fn test_pow_4() {
    assert_eq!(1, basic::pow(2, 0));
}

#[test]
fn test_pow_5() {
    assert_eq!(0, basic::pow(0, 2));
}