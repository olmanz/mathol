use basic::{pythagoras2d, pythagoras3d};
use std::cmp::Ordering;

/// Rust struct for points in the two-dimensional cartesic coordinate system.
pub struct Cartesic2D {
    /// abscissa (distance of the point to x-axis)
    pub x: f64,
    /// ordinate (distance of the point to y-axis)
    pub y: f64,
}

impl Cartesic2D {
    /// Calculates the euclidic distance between two points
    pub fn get_distance(&self, other: Cartesic2D) -> f64 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
    }

    /// Transforms cartesic coordinates to polar coordinates.
    /// Returns an instance of the struct Polar
    pub fn transform_to_polar(&self) -> Polar {
        Polar {
            r: pythagoras2d(self.x, self.y),
            phi: self.y.atan2(self.x).to_degrees(),
        }
    }
}

/// Rust struct for points in the two-dimensional polar coordinate system.
pub struct Polar {
    /// radius (distance of the point from the pole)
    pub r: f64,
    /// azimuth (angle to x-axis)
    pub phi: f64,
}

impl Polar {
    /// Transforms polar coordinates to cartesic coordinates.
    /// Returns an instance of the struct Cartesic
    pub fn transform_to_cartesic2d(&self) -> Cartesic2D {
        Cartesic2D {
            x: self.r * self.phi.to_radians().cos(),
            y: self.r * self.phi.to_radians().sin(),
        }
    }
}

/// Rust struct for points in the three-dimensional cartesic coordinate system.
pub struct Cartesic3D {
    /// Distance of the point to x-axis
    pub x: f64,
    /// Distance of the point to y-axis
    pub y: f64,
    /// Distance of the point to z-axis
    pub z: f64,
}

impl Cartesic3D {
    /// Transforms cartesic coordinates to cylindrical coordinates.
    /// Returns an instance of the struct Cylindrical
    pub fn transform_to_cylindrical(&self) -> Cylindrical {
        Cylindrical {
            rho: pythagoras2d(self.x, self.y),
            phi: get_angle(self.x, self.y),
            z: self.z,
        }
    }

    /// Transforms cartesic coordinates to spherical coordinates.
    /// Returns an instance of the struct Spherical
    pub fn transform_to_spherical(&self) -> Spherical {
        let r = pythagoras3d(self.x, self.y, self.z);
        Spherical {
            r,
            theta: (self.z / r).acos().to_degrees(),
            phi: (self.y / self.x).atan().to_degrees(),
        }
    }
}

/// Calculate the angle of the polar axis in relaion to the x-axis
fn get_angle(x: f64, y: f64) -> f64 {
    match (x, y.partial_cmp(&0.0)) {
        (0.0, Some(Ordering::Less)) => 270.0,
        (0.0, Some(Ordering::Equal)) => 0.0,
        (0.0, Some(Ordering::Greater)) => 90.0,
        (_, _) => (y / x).atan().to_degrees(),
    }
}

/// Rust struct for points in the cylindrical coordinate system.
pub struct Cylindrical {
    /// DIctance from z-axis to the point
    pub rho: f64,
    /// Angle between the reference direction on the chosen plane and the line from the origin to the projection of P on the plane
    pub phi: f64,
    /// Signed distance from the chosen plane to the point P
    pub z: f64,
}

impl Cylindrical {
    /// Transforms cylindrical coordinates to cartesic coordinates.
    /// Returns an instance of the struct Cartesic
    pub fn transform_to_cartesic3d(&self) -> Cartesic3D {
        Cartesic3D {
            x: self.rho * self.phi.to_radians().cos(),
            y: self.rho * self.phi.to_radians().sin(),
            z: self.z,
        }
    }
}

/// Rust struct for points in the spherical coordinate system.
pub struct Spherical {
    /// Distance from the origin point
    pub r: f64,
    /// Signed angle measured from the azimuth reference direction to the orthogonal projection of the line segment on the reference plane
    pub theta: f64,
    /// Angle between the zenith direction and the line segment
    pub phi: f64,
}

impl Spherical {
    /// Transforms spherical coordinates to cartesic coordinates.
    /// Returns an instance of the struct Cartesic
    pub fn transform_to_cartesic3d(&self) -> Cartesic3D {
        Cartesic3D {
            x: self.r * self.theta.to_radians().sin() * self.phi.to_radians().cos(),
            y: self.r * self.theta.to_radians().sin() * self.phi.to_radians().sin(),
            z: self.r * self.theta.to_radians().cos(),
        }
    }
}