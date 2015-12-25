use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error;
use std::fmt;
use std::io;
use std::num;

use scale::*;
use tri::Tri;
use vector::Vector;

enum LineType {
	Solid,
	Facet(f64, f64, f64),
	Outer,
	Vertex(f64, f64, f64),
	Endloop,
	Endfacet,
	Endsolid,
}

#[derive(Debug)]
pub enum StlError{
	Io(io::Error),
	Parse(num::ParseFloatError),
	BadLine(String),
	BadStl(String),
}

impl fmt::Display for StlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            StlError::Io(ref err) => write!(f, "IO error: {}", err),
            StlError::Parse(ref err) => write!(f, "Parse error: {}", err),
            StlError::BadLine(ref s) => write!(f,
            	".stl file has invalid line start: {}", s),
            StlError::BadStl(ref s) => write!(f,"Bad .stl: {}", s),
        }
    }
}

impl error::Error for StlError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            StlError::Io(ref err) => err.description(),
            // Normally we can just write `err.description()`, but the error
            // type has a concrete method called `description`, which conflicts
            // with the trait method. For now, we must explicitly call
            // `description` through the `Error` trait.
            StlError::Parse(ref err) => error::Error::description(err),
            StlError::BadLine(ref s) => s,
            StlError::BadStl(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            StlError::Io(ref err) => Some(err),
            StlError::Parse(ref err) => Some(err),
            StlError::BadLine(_) => None,
            StlError::BadStl(_) => None
        }
    }
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
	let mut normal : Option<Vector> = None;
	let mut tris : Vec<Tri> = Vec::new();
	let mut ps : Vec<Vector> = Vec::with_capacity(3);
	for line in reader.lines().map(line_to_enum) {
		let tok = match line{
					Ok(t) => t,
					Err(e) => return Err(e)
		};
		match tok {
			LineType::Facet(x,y,z) => match normal {
				None=> {
					normal = Some(Vector::new(up(x),up(y),up(z)));
				},
				Some(_) => return Err(StlError::BadStl(
					"Found consecutive normal vectors.".to_string())),
			},
			LineType::Vertex(x,y,z) => if ps.len() < 3 {
					ps.push(Vector::new(up(x),up(y),up(z)));
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