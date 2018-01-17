use std::f64::consts::PI;

pub trait Planimetry {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
}

pub struct Triangulum {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub struct Rectangle {
    pub a: f64,
    pub b: f64,
}

pub struct Parallelogram {
    pub a: f64,
    pub b: f64,
    pub h: f64,
}

pub struct Trapeze {
    pub a: f64,
    pub b: f64,
    pub h: f64,
}

pub struct Circle {
    pub r: f64
}

impl Planimetry for Triangulum {
    fn get_area(&self) -> f64 {
        let s = self.get_perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    fn get_perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

impl Planimetry for Rectangle {
    fn get_area(&self) -> f64 {
        self.a * self.b
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * self.a + 2.0 * self.b
    }
}

impl Planimetry for Parallelogram {
    fn get_area(&self) -> f64 {
        self.a * self.h
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * self.a + 2.0 * self.b
    }
}

impl Planimetry for Trapeze {
    fn get_area(&self) -> f64 {
        0.5 * (self.a + self.b) * self.h
    }

    fn get_perimeter(&self) ->f64 {
        0.0
    }
}

impl Planimetry for Circle {
    fn get_area(&self) -> f64 {
        PI * self.r * self.r
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * PI * self.r
    }
}