use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io;

use tri::Tri;
use nalgebra::Vec3;
use stlerror::StlError;

enum LineType {
	Solid,
	Facet(f64, f64, f64),
	Outer,
	Vertex(f64, f64, f64),
	Endloop,
	Endfacet,
	Endsolid,
}

fn parse_f64(f: &str) -> Result<f64,StlError> { 
	f.parse().map_err(StlError::Parse)
}

fn line_to_enum(line: Result<String,io::Error>) -> 
Result<LineType,StlError> {
	match line {
		Err(e) => Err(StlError::Io(e)),
		Ok(ln) => {
			let l : Vec<&str> = ln.split(" ").collect::<Vec<&str>>();
			match l[0] {
				"solid" => Ok(LineType::Solid),
				"facet" => Ok(LineType::Facet(try!(parse_f64(l[2])),
								 		   try!(parse_f64(l[3])),
								 		   try!(parse_f64(l[4])))),
				"outer" => Ok(LineType::Outer),
				"vertex" => Ok(LineType::Vertex(try!(parse_f64(l[1])),
								 		   try!(parse_f64(l[2])),
								 		   try!(parse_f64(l[3])))),
				"endloop" => Ok(LineType::Endloop),
				"endfacet" => Ok(LineType::Endfacet),
				"endsolid" => Ok(LineType::Endsolid),
				_ => Err(StlError::BadLine(l[0].to_string())),
			}
		}
	}
}

pub fn read_stl(filename: &str) -> Result<Vec<Tri>,StlError>{
	let f = File::open(filename).unwrap();
	let reader = BufReader::new(f);
	let mut normal : Option<Vec3<f64>> = None;
	let mut tris : Vec<Tri> = Vec::new();
	let mut ps : Vec<Vec3<f64>> = Vec::with_capacity(3);
	for line in reader.lines().map(line_to_enum) {
		let tok = match line{
					Ok(t) => t,
					Err(e) => return Err(e)
		};
		match tok {
			LineType::Facet(x,y,z) => match normal {
				None=> {
					normal = Some(Vec3::new(x,y,z));
				},
				Some(_) => return Err(StlError::BadStl(
					"Found consecutive normal Vec3<f64s.".to_string())),
			},
			LineType::Vertex(x,y,z) => if ps.len() < 3 {
					ps.push(Vec3::new(x,y,z));
					if ps.len()==3 {
						match normal {
							None => return Err(StlError::BadStl(
								"Facet entry is missing its normal".to_string())),
							Some(v) => {
								tris.push(Tri::new(ps[0],
														ps[1],
														ps[2],
														v));
								ps.clear();
								normal = None;
							},
						}
					}
				} else {
					return Err(StlError::BadStl(
						"Found 4+ consecutive vertex entries".to_string()));
				},
			_ => continue,
		}
	}
	Ok(tris)
}