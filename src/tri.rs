extern crate nalgebra;

use std::fmt;
use nalgebra::Vec3;
use nalgebra::cross;

#[derive(Debug,Copy,Clone)]
pub struct Tri{
	pub vs : (Vec3<f64>, Vec3<f64>, Vec3<f64>),
	pub es : (Vec3<f64>, Vec3<f64>, Vec3<f64>),
	pub n : Vec3<f64>,
}

impl Tri {
	///Constructs a new `Tri` from 3 Vec3<f64>s.
	///
	///#Examples
	///```
	///# use trimesh::tri::Tri;
	///# use trimesh::Vec3<f64>::Vec3<f64>;
	///let p1 = Vec3<f64>::new(0.,0.,0.);
	///let p2 = Vec3<f64>::new(3.,0.,0.);
	///let p3 = Vec3<f64>::new(0.,4.,0.);
	///
	///let t1 = Tri::new_raw(p1,p2,p3);
	///```
	///
	pub fn new_raw(a: Vec3<f64>, b: Vec3<f64>, c: Vec3<f64>) -> Tri{
		Tri::new(a,b,c, nalgebra::cross(&(b-a),&(c-a)))
	}
	///Constructs a new `Tri` from 3 Vec3<f64>s.
	///
	///#Examples
	///```
	///# use trimesh::tri::Tri;
	///# use trimesh::Vec3<f64>::Vec3<f64>;
	///let p1 = Vec3<f64>::new(0.,0.,0.);
	///let p2 = Vec3<f64>::new(3.,0.,0.);
	///let p3 = Vec3<f64>::new(0.,4.,0.);
	///let n = Vec3<f64>::new(0.,0.,1.);
	///
	///let t1 = Tri::new(p1,p2,p3,n);
	///```
	///
	pub fn new(a: Vec3<f64>, b: Vec3<f64>, c: Vec3<f64>, n: Vec3<f64>) -> Tri{
		Tri{vs: (a,b,c),
			es: (b-a,c-b,a-c),
			n: n}
	}
	///#Examples
	///```
	///# use trimesh::tri::Tri;
	///# use trimesh::Vec3<f64>::Vec3<f64>;
	///let p1 = Vec3<f64>::new(0.,0.,0.);
	///let p2 = Vec3<f64>::new(3.,0.,0.);
	///let p3 = Vec3<f64>::new(0.,4.,0.);
	///# let n = Vec3<f64>::new(0.,0.,1.);
	///
	///let t1 = Tri::new(p1,p2,p3,n);
	///assert_eq!(t1.has_point(&p3),true);
	///let p4 = Vec3<f64>::new(5.,5.,5.);
	///assert_eq!(t1.has_point(&p4),false);
	///```
	pub fn has_point (&self, p : Vec3<f64>) -> bool{
		let vs = self.vs;
		return p == vs.0 || p == vs.1 || p == vs.2;
	}
}

impl fmt::Display for Tri {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (a,b,c) = self.vs;
        write!(f, "Tri({:?}\n{:?}\n{:?}\n{:?}\n)", a, b, c, self.n)
    }
}