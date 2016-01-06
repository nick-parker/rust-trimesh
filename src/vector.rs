use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
/// A 3d Vector represented with f64 coordinates.
pub struct Vector {
	x : f64,
	y : f64,
	z : f64
}

impl Vector {
	///Constructs a new `Vector` from three f64 coordinates.
	///
	///#Examples
	///
	///```
	///use trimesh::vector::Vector;
	///let v1 = Vector::new(1.,0.,0.);
	///```
	pub fn new(x:f64, y:f64, z:f64) -> Vector{
		Vector { x:x, y:y, z:z}
	}
	pub fn dot(a:Vector,b:Vector) -> f64{
		a.x*b.x+a.y*b.y+a.z*b.z
	}
	pub fn cross(a:Vector,b:Vector) -> Vector{
		Vector::new(a.y*b.z-a.z*b.y,
					a.z*b.x-a.x*b.z,
					a.x*b.y-a.y*b.x)
	}
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}


impl fmt::Display for Vector {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector({},{},{})", self.x, self.y, self.z)
    }
}