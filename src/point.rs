#[derive(Debug)]
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
	///let p1 = Point::new(3,4,5);
	///```
	pub fn new(x:i64, y:i64, z:i64) -> Point{
		Point { x:x, y:y, z:z}
	}
}