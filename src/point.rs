use std::fmt;
use scale::*;

#[derive(Debug, Copy, Clone)]
/// A 3d point represented with i64 coordinates.
pub struct Point {
	x : i64,
	y : i64,
	z : i64
}

impl Point {
	///Constructs a new `Point` from three i64 coordinates.
	///
	///#Examples
	///
	///```
	///# use trimesh::point::Point;
	///let p1 = Point::new(3,4,5);
	///```
	pub fn new(x:i64, y:i64, z:i64) -> Point{
		Point { x:x, y:y, z:z}
	}
}



impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({},{},{})", down(self.x), down(self.y), down(self.z))
    }
}