mod primitives;
mod mesh;
mod cube_rotations;

use mesh::Mesh;

fn main() {
    let twist = Mesh::from_obj_file("data/basic/one_twist.obj");

    let i = cube_rotations::CubeRotation::identity();
    let rz = cube_rotations::CubeRotation::rz();
    let ry = cube_rotations::CubeRotation::ry();
    println!("(Rz) {:?} * (Ry) {:?} = {:?}", rz, ry, &rz * &ry);

    let point = primitives::Vertex([1.0, 2.0, 3.0]);
    println!("(I) {:?} * {:?} = {:?}", i, point, &i * &point); 
    println!("(Rz) {:?} * {:?} = {:?}", rz, point, &rz * &point); 
    println!("(Ry) {:?} * {:?} = {:?}", ry, point, &ry * &point); 
}
