use std::fmt;
use scale::*;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
/// A 3d Vector represented with i64 coordinates.
pub struct Vector {
	x : i64,
	y : i64,
	z : i64
}

impl Vector {
	///Constructs a new `Vector` from three i64 coordinates.
	///
	///#Examples
	///
	///```
	///use trimesh::vector::Vector;
	///let v1 = Vector::new(1,0,0);
	///```
	pub fn new(x:i64, y:i64, z:i64) -> Vector{
		Vector { x:x, y:y, z:z}
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
        write!(f, "Vector({},{},{})", down(self.x), down(self.y), down(self.z))
    }
}