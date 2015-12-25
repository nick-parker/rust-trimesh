extern crate trimesh;

fn main() {
  let ts : Vec<trimesh::tri::Tri> = match trimesh::stl::read_stl("vtop.stl") {
  	Ok(x) => x,
  	Err(_) => return
  };
  println!("{} Tris:",ts.len());
  for t in ts {
  	println!("{}",t);
  }
}
