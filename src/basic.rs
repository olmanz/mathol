pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn get_distance(p: &Point, q: &Point) -> f64 {
    ((p.x - q.x) * (p.x - q.x) + (p.y - q.y) * (p.y - q.y)).sqrt()
}

pub fn pow(base: i64, mut exponent: i64) -> i64 {
    if exponent == 0 {
        1
    } else {
        base * pow(base, exponent - 1)
    }
}

pub fn pythagoras2d(a: f64, b: f64) -> f64 {
    ((a * a) + (b * b)).sqrt()
}

pub fn pythagoras3d(a: f64, b: f64, c: f64) -> f64 {
    ((a * a) + (b * b) + (c * c)).sqrt()
}