mod primitives;
mod mesh;
mod cube_rotations;

use mesh::Mesh;
use cube_rotations::CubeRotation;

fn make_quad_twist() -> Mesh {
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

    result
}

fn is_twist(i: u32, j: u32, k: u32) -> bool {
    let parities = (i % 2, j % 2, k % 2);

    parities == (0, 0, 1) || parities == (1, 1, 0)
}

fn main() {
    const N: u32 = 7;
    const M: u32 = 11;
    const P: u32 = 13;
    let twist_tile = make_quad_twist();

    let mut grid = Mesh::new();
    for i in 0..N {
        for j in 0..M {
            for k in 0..P {
                if is_twist(i, j, k) {
                    let twist_cell = twist_tile.translate(
                        &[i as f32, j as f32, k as f32]);
                    grid.add_geometry(&twist_cell);
                }
            }
        }
    }

    grid.save_obj_file("grid.obj");
}
