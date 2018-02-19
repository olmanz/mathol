extern crate mathol;
use mathol::vectoroperations::vector3d::*;
use mathol::vectoroperations::plane::*;

fn main() {
    let p = Plane {r: Vector3D {x: 3, y: 1, z: -2}, n: Vector3D {x: 2, y: -1, z: 4}};
    let q = Plane {r: Vector3D {x: -4, y: 3, z: 0}, n: Vector3D {x: -4, y: 2, z: -8}};
    let boo = p.is_parallel_to_plane(q);
    let dist = p.get_distance_from_plane(q);
}