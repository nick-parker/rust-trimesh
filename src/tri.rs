use vector::Vector;
use std::fmt;

#[derive(Debug,Copy,Clone)]
pub struct Tri{
	vs : (Vector, Vector, Vector),
	es : (Vector, Vector, Vector),
	n : Vector,
}

impl Tri {
	///Constructs a new `Tri` from 3 Vectors.
	///
	///#Examples
	///```
	///# use trimesh::Vector::Vector;
	///# use trimesh::tri::Tri;
	///let p1 = Vector::new(0,0,0);
	///let p2 = Vector::new(3,0,0);
	///let p3 = Vector::new(0,4,0);
	///
	///let t1 = Tri::new_raw(p1,p2,p3);
	///```
	///
	pub fn new_raw(a: Vector, b: Vector, c: Vector) -> Tri{
		Tri::new(a,b,c,Vector::new(0,0,0))
	}
	///Constructs a new `Tri` from 3 Vectors.
	///
	///#Examples
	///```
	///# use trimesh::Vector::Vector;
	///# use trimesh::tri::Tri;
	///# use trimesh::vector::Vector;
	///let p1 = Vector::new(0,0,0);
	///let p2 = Vector::new(3,0,0);
	///let p3 = Vector::new(0,4,0);
	///let n = Vector::new(0,0,1);
	///
	///let t1 = Tri::new(p1,p2,p3,n);
	///```
	///
	pub fn new(a: Vector, b: Vector, c: Vector, n: Vector) -> Tri{
		Tri{vs: (a,b,c),
			es: (b-a,c-b,a-c),
			n: n}
	}
}

impl fmt::Display for Tri {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (a,b,c) = self.vs;
        write!(f, "Tri({}\n{}\n{}\n{}\n)", a, b, c, self.n)
    }
}