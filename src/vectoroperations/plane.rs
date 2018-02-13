use num::Num;
use std::fmt::Debug;
use basic::{Convert, Amount};
use vectoroperations::vectortraits::Vectoroperations;
use vectoroperations::vector3d::Vector3D;
use vectoroperations::line3d::Line3D;

#[derive(Debug)]
pub struct Plane<T>
    where T: Num + Copy + Convert + Amount<T>
{
    pub r: Vector3D<T>,
    pub a: Vector3D<T>,
    pub b: Vector3D<T>,
}

impl<T> Plane<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    pub fn build_plane_from_three_points(p: &Vector3D<T>, q: &Vector3D<T>, r: &Vector3D<T>) -> Plane<T> {
        Plane {
            r: Vector3D::build_vector(p.x, p.y, p.z),
            a: q.sub_vector(&p),
            b: r.sub_vector(&p),
        }
    }

    pub fn get_distance_from_point(&self, p: &Vector3D<T>) -> f64 {
        let n = self.a.get_vector_product(&self.b);
        let r = p.sub_vector(&self.r);
        println!("n: {:?}", n);
        println!("r: {:?}", r);
        let d = n.get_scalar_product(&r).to_f64().get_amount() / n.get_length().get_amount();
        d
    }

    pub fn is_parallel_to_line(&self, l: &Line3D<T>) -> bool {
        if l.a.get_scalar_product(&self.a.get_vector_product(&self.b)).to_f64() == 0.0 {
            true
        } else {
            false
        }
    }

    pub fn get_distance_from_line(&self, l: &Line3D<T>) -> Result<f64, &str> {
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