use num::Num;
use basic::{Convert, Amount};
use std::fmt::Debug;
use vectoroperations::vectortraits::Vectoroperations;
use vectoroperations::vector3d::Vector3D;

#[derive(Debug)]
pub struct Line3D<T>
    where T: Num + Copy + Convert + Amount<T>
{
    pub r: Vector3D<T>,
    pub a: Vector3D<T>,
}

impl<T> Line3D<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    pub fn build_line_from_two_points(p: &Vector3D<T>, q: &Vector3D<T>) -> Line3D<T> {
        Line3D {
            r: Vector3D::build_vector(p.x, p.y, p.z),
            a: q.sub_vector(&p),
        }
    }

    pub fn distance_from_point(&self, p: &Vector3D<T>) -> f64 {
        let r = p.sub_vector(&self.r);
        self.a.get_vector_product(&r).get_length() / self.a.get_length()
    }

    pub fn are_parallel(&self, l: &Line3D<T>) -> bool {
        if self.a.get_vector_product(&l.a).get_length() == 0.0 {
            true
        } else {
            false
        }
    }

    pub fn do_cross(&self, l: &Line3D<T>) -> bool {
        if !self.are_parallel(&l) && self.a.get_triple_product(&l.a, &l.r.sub_vector(&self.r)).to_f64() == 0.0 {
            true
        } else {
            false
        }
    }

    pub fn are_askew(&self, l: &Line3D<T>) -> bool {
        if !self.are_parallel(&l) && self.a.get_triple_product(&l.a, &l.r.sub_vector(&self.r)).to_f64() != 0.0 {
            true
        } else {
            false
        }
    }

    pub fn distance_from_line(&self, line: &Line3D<T>) -> Result<f64, &str> {
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