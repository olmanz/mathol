extern crate num;
use self::num::Num;
use basic::{Convert, Amount, pow};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Vector<T>
    where T: Num + Copy + Convert + Amount<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    pub fn build_vector(x: T, y: T, z: T) -> Vector<T> {
        Vector {x, y, z}
    }
}

pub trait Vectoroperations<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    fn add_vector(&self, vec: &Vector<T>) -> Vector<T>;
    fn sub_vector(&self, vec: &Vector<T>) -> Vector<T>;
    fn get_length(&self) -> f64;
    fn get_direction_angle(&self) -> (f64, f64, f64);
    fn multiply_with_scalar(&self, lambda: T) -> Vector<T>;
    fn get_scalar_product(&self, vec: &Vector<T>) -> T;
    fn get_cut_angle(&self, vec: &Vector<T>) -> f64;
    fn get_vector_product(&self, vec: &Vector<T>) -> Vector<T>;
    fn get_triple_product(&self, vec_1: &Vector<T>, vec_2: &Vector<T>) -> T;
}

impl<T> Vectoroperations<T> for Vector<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    fn add_vector(&self, vec: &Vector<T>) -> Vector<T> {
        Vector::build_vector(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }

    fn sub_vector(&self, vec: &Vector<T>) -> Vector<T> {
        Vector::build_vector(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }

    fn get_length(&self) -> f64 {
        (pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2)).to_f64().sqrt()
    }

    fn get_direction_angle(&self) -> (f64, f64, f64) {
        let n = self.get_length().to_f64();

        let alpha = (self.x.to_f64() / n).acos();
        let beta = (self.y.to_f64() / n).acos();
        let gamma = (self.z.to_f64() / n).acos();

        (alpha, beta, gamma)
    }

    fn multiply_with_scalar(&self, lambda: T) -> Vector<T> {
        Vector::build_vector(lambda * self.x, lambda * self.y, lambda * self.z)
    }

    fn get_scalar_product(&self, vec: &Vector<T>) -> T {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    fn get_cut_angle(&self, vec: &Vector<T>) -> f64 {
        self.get_scalar_product(vec).to_f64() / (self.get_length().to_f64() * vec.get_length().to_f64())
    }

    fn get_vector_product(&self, vec: &Vector<T>) -> Vector<T> {
        Vector::build_vector(self.y * vec.z - self.z * vec.y, self.z * vec.x - self.x * vec.z, self.x * vec.y - self.y * vec.x)
    }

    fn get_triple_product(&self, vec_1: &Vector<T>, vec_2: &Vector<T>) -> T {
        self.x * (vec_1.y * vec_2.z - vec_1.z * vec_2.y) + self.y * (vec_1.z * vec_2.x - vec_1.x * vec_2.z) + self.z * (vec_1.x * vec_2.y - vec_1.y * vec_2.x)
    }
}

#[derive(Debug)]
pub struct Line<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    pub r: Vector<T>,
    pub a: Vector<T>,
}

impl<T> Line<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    pub fn build_line_from_two_points(p: &Vector<T>, q: &Vector<T>) -> Line<T> {
        Line {
            r: Vector::build_vector(p.x, p.y, p.z),
            a: q.sub_vector(&p),
        }
    }

    pub fn distance_from_point(&self, p: &Vector<T>) -> f64 {
        let r = p.sub_vector(&self.r);
        self.a.get_vector_product(&r).get_length() / self.a.get_length()
    }

    pub fn are_parallel(&self, l: &Line<T>) -> bool {
        if self.a.get_vector_product(&l.a).get_length() == 0.0 {
            true
        } else {
            false
        }
    }

    pub fn do_cross(&self, l: &Line<T>) -> bool {
        if !self.are_parallel(&l) && self.a.get_triple_product(&l.a, &l.r.sub_vector(&self.r)).to_f64() == 0.0 {
            true
        } else {
            false
        }
    }

    pub fn are_askew(&self, l: &Line<T>) -> bool {
        if !self.are_parallel(&l) && self.a.get_triple_product(&l.a, &l.r.sub_vector(&self.r)).to_f64() != 0.0 {
            true
        } else {
            false
        }
    }

    pub fn distance_from_line(&self, line: &Line<T>) -> Result<f64, &str> {
        let r = line.r.sub_vector(&self.r);
        if self.are_parallel(&line) {
            Ok(self.a.get_vector_product(&r).get_length() / self.a.get_length())
        } else if self.are_askew(&line) {
            Ok(self.a.get_triple_product(&line.a, &r).to_f64().get_amount() / self.a.get_vector_product(&line.a).get_length())
        } else {
            return Err("Lines do cross");
        }
    }
}

#[derive(Debug)]
pub struct Plane<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    pub r: Vector<T>,
    pub a: Vector<T>,
    pub b: Vector<T>,
}

impl<T> Plane<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    pub fn build_plane_from_three_points(p: &Vector<T>, q: &Vector<T>, r: &Vector<T>) -> Plane<T> {
        Plane {
            r: Vector::build_vector(p.x, p.y, p.z),
            a: q.sub_vector(&p),
            b: r.sub_vector(&p),
        }
    }

    pub fn get_distance_from_point(&self, p: &Vector<T>) -> f64 {
        let n = self.a.get_vector_product(&self.b);
        let r = p.sub_vector(&self.r);
        println!("n: {:?}", n);
        println!("r: {:?}", r);
        let d = n.get_scalar_product(&r).to_f64().get_amount() / n.get_length().get_amount();
        d
    }

    pub fn is_parallel_to_line(&self, l: &Line<T>) -> bool {
        if l.a.get_scalar_product(&self.a.get_vector_product(&self.b)).to_f64() == 0.0 {
            true
        } else {
            false
        }
    }

    pub fn get_distance_from_line(&self, l: &Line<T>) -> Result<f64, &str> {
        if !self.is_parallel_to_line(&l) {
            return Err("Line is not parallel to plane");
        }

        let n = self.a.get_vector_product(&self.b);
        let r = l.r.sub_vector(&self.r);
        let d = n.get_scalar_product(&r).to_f64().get_amount() / n.get_length().get_amount();
        Ok(d)
    }

    pub fn is_parallel_to_plane(&self, p: &Plane<T>) -> bool {
        let n1 = self.a.get_vector_product(&self.b);
        let n2 = p.a.get_vector_product(&p.b);
        if n1.get_vector_product(&n2).get_length().to_f64() == 0.0 {
            true
        } else {
            false
        }
    }

    pub fn get_distance_from_plane(&self, p: &Plane<T>) -> Result<f64, &str> {
        if !self.is_parallel_to_plane(&p) {
            return Err("The planes are not parallel");
        }

        let n = self.a.get_vector_product(&self.b);
        let r = self.r.sub_vector(&p.r);
        let d = n.get_scalar_product(&r).to_f64().get_amount() / n.get_length().get_amount();
        Ok(d)
    }

//    // TODO: Implementation
//    pub fn get_cutting_point_with_line(&self, l: &Line<T>) {
//        unimplemented!()
//    }
//
//    // TODO: Implementation
//    pub fn get_cutting_angle_with_line(&self, l: &Line<T>) {
//        unimplemented!()
//    }
//
//    // TODO: Implementation
//    pub fn get_cutting_line_with_plane(&self, p: &Plane<T>) {
//        unimplemented!()
//    }
//
//    // TODO: Implementation
//    pub fn get_cutting_angle_with_plane(&self, p: &Plane<T>) {
//        unimplemented!()
//    }
}