// extern crate trimesh;

// use trimesh::mesh::Mesh;

// fn main() {
//   let ts : Vec<trimesh::tri::Tri> = match trimesh::stl::read_stl("plane.stl") {
//   	Ok(x) => x,
//   	Err(_) => return
//   };
//   let m1 = Mesh::new(&ts).unwrap();
//   println!("{}",m1);
// }

extern crate trimesh;
extern crate kiss3d;
extern crate nalgebra;

use nalgebra::Vec3;
use nalgebra::cross;
use kiss3d::window::Window;

fn main() {
    let v1 = Vec3::new(1.0f64, 0.0, 0.0);
    let v2 = Vec3::new(0.0f64, 1.0, 0.0);
    let v3 = nalgebra::cross(&v1,&v2);
    println!("{:?}",v3);

    // let mut window = Window::new("Kiss3d: cube");
    // let mut c      = window.add_cube(0.1, 0.1, 0.1);

    // c.set_color(1.0, 0.0, 0.0);

    // while window.render() {
    //     c.prepend_to_local_rotation(&Vec3::new(0.0f32, 0.014, 0.0));
    // }
}
