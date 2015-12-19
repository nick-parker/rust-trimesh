use point::Point;

#[derive(Debug)]
pub struct Tri {
	a : Point,
	b : Point,
	c : Point
}

impl Tri {
	///Constructs a new `Tri` from 3 points.
	///
	///#Examples
	///```
	///let p1 = Point::new(0,0,0);
	///let p2 = Point::new(3,0,0);
	///let p3 = Point::new(0,4,0);
	///
	///let t1 = Tri::new(p1,p2,p3);
	///```
	///
	pub fn new(a: Point, b: Point, c: Point) -> Tri{
		Tri{ a: a, b: b, c: c}
	}
}