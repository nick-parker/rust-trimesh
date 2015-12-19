use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error;
use std::fmt;
use std::io;
use std::num;

use point::Point;
use tri::Tri;

enum LineType {
	Solid(String),
	Facet(f32, f32, f32),
	Outer,
	Vertex(f32, f32, f32),
	Endloop,
	Endfacet,
	Endsolid(String),
}

#[derive(Debug)]
enum StlError{
	Io(io::Error),
	Parse(error::Error)
}

impl fmt::Display for StlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            StlError::Io(ref err) => write!(f, "IO error: {}", err),
            StlError::Parse(ref err) => write!(f, "Parse error: {}", err),
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
        }
    }
}

// fn line_to_enum(line: &str) -> Result<LineType,StlError> {
// 	let l = line.split(" ").map_err(StlError::Io);
// 	match l.[0] {
// 		"solid" => 
// 	}
// }

pub fn read_stl(filename: &str) {
	let f = File::open(filename).unwrap();
	let reader = BufReader::new(f);
	let tris : Vec<Tri> = Vec::new();
	let ps : Vec<Point> = Vec::with_capacity(3);
	for line in reader.lines() {
		let line = line.unwrap();
		println!("{}",line);
	}
}