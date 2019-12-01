mod primitives;
mod mesh;
mod cube_rotations;

use mesh::Mesh;
use cube_rotations::CubeRotation;

fn main() {
    let twist = Mesh::from_obj_file("data/basic/one_twist.obj");

    let rz = CubeRotation::rz();
    let twist2 = twist.rotate(&rz);

    let twist3 = twist2.rotate(&rz);
    let twist4 = twist3.rotate(&rz);

    let mut result = Mesh::new();
    result.add_geometry(&twist);
    result.add_geometry(&twist2);
    result.add_geometry(&twist3);
    result.add_geometry(&twist4);
    result.save_obj_file("four_twists.obj");

    /*
    let i = cube_rotations::CubeRotation::identity();
    let ry = cube_rotations::CubeRotation::ry();
    println!("(Rz) {:?} * (Ry) {:?} = {:?}", rz, ry, &rz * &ry);

    let point = primitives::Vertex([1.0, 2.0, 3.0]);
    println!("(I) {:?} * {:?} = {:?}", i, point, &i * &point); 
    println!("(Rz) {:?} * {:?} = {:?}", rz, point, &rz * &point); 
    println!("(Ry) {:?} * {:?} = {:?}", ry, point, &ry * &point); 
    */
}
