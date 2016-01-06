use tri::Tri;
use stlerror::StlError;
use std::fmt;

///A mesh which maintains information about Tris' neighbors to allow
///rapid traversal.
pub type Neighbors<'a> = [Option<&'a Tri>; 3];

pub struct Mesh<'a>{
	pub ts : &'a Vec<Tri>,
	pub ns : Vec<Neighbors<'a>>
}

impl<'a> Mesh<'a>{
	pub fn new (ref ts : &'a Vec<Tri>)-> Result<Mesh<'a>, StlError>{
		let mut ns : Vec<Neighbors<'a>> = Vec::with_capacity(ts.len());
		for _ in 0..ts.len() { ns.push([None,None,None])};
	  for (i, t) in ts.iter().enumerate(){
			for (j, u) in ts.iter().enumerate().filter(|x| x.0>i) {
				let vs = t.vs;
				let verts = (u.has_point(vs.0),u.has_point(vs.1),u.has_point(vs.2));
				match verts {
					(true,true,true) => return Err(StlError::BadStl(
						"Identical Tris found in mesh.".to_string())),
					(true,true,false) => ns[i][0] = Some(&u),
					(false,true,true) => ns[i][1] = Some(&u),
					(true,false,true) => ns[i][2] = Some(&u),
					_ => continue //Tris share a vertex, which we don't care about.
				}
				let vs2 = u.vs;
				let verts2 = (t.has_point(vs2.0),t.has_point(vs2.1),t.has_point(vs2.2));
				match verts2 {
					(true,true,true) => return Err(StlError::BadStl(
						"Identical Tris found in mesh.".to_string())),
					(true,true,false) => ns[j][0] = Some(&t),
					(false,true,true) => ns[j][1] = Some(&t),
					(true,false,true) => ns[j][2] = Some(&t),
					_ => continue //Tris share a vertex, which we don't care about.
				}
			}
		}
		Ok(Mesh{ts: ts, ns: ns})
	}
}

impl<'a> fmt::Display for Mesh<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mesh: {:?}", self.ts)
    }
}

//Crossing edges: (edge x line) x new normal

