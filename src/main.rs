mod mesh;
use mesh::Mesh;

fn main() {
    let twist = Mesh::from_obj_file("data/basic/one_twist.obj");
}
