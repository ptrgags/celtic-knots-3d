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

fn make_end_cap() -> Mesh {
    Mesh::from_obj_file("data/basic/end_cap.obj")
}

#[derive(Copy, Clone)]
struct CellID(u32, u32, u32);

enum RangeComparison {
    Min,
    Between,
    Max
}

struct BoundsClassification(
    RangeComparison, 
    RangeComparison,
    RangeComparison
);
struct Range(u32, u32);
struct Bounds(Range, Range, Range);

fn compare_range(x: u32, range: Range) -> RangeComparison {
    let Range(min_val, max_val) = range;
    use RangeComparison::{Min, Max, Between};
    if x == min_val {
        Min
    } else if x == max_val - 1 {
        Max
    } else {
        Between
    }
}

fn compare_bounds(cell_id: CellID, bounds: Bounds) -> BoundsClassification {
    let CellID(i, j, k) = cell_id;
    let Bounds(x_range, y_range, z_range) = bounds;
    BoundsClassification(
        compare_range(i, x_range),
        compare_range(j, y_range),
        compare_range(k, z_range),
    )
}

fn is_twist(cell_id: CellID) -> bool {
    let CellID(i, j, k) = cell_id;
    let parities = (i % 2, j % 2, k % 2);

    parities == (0, 0, 1) || parities == (1, 1, 0)
}

fn twist_rotation(k: u32) -> CubeRotation {
    let layer_parity = k % 2;
    if layer_parity == 0 {
        CubeRotation::rx()
    } else {
        CubeRotation::identity()
    }
}

fn orient_twist_cell(cell_id: CellID) -> Mesh {
    let twist_tile =  make_quad_twist();
    let CellID(i, j, k) = cell_id;
    let rotation = twist_rotation(k);
    let transformed = twist_tile
        .rotate(&rotation)
        .translate(&[i as f32, j as f32, k as f32]);

    transformed
}

fn select_twist_cell(cell_id: CellID) -> Mesh {
    orient_twist_cell(cell_id)
}

fn generate_cell(cell_id: CellID) -> Option<Mesh> {
    if is_twist(cell_id) {
        Some(select_twist_cell(cell_id))
    } else {
        None
    }
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
                let cell_id = CellID(i, j, k);
                match generate_cell(cell_id) {
                    Some(mesh) => {
                        grid.add_geometry(&mesh);
                    },
                    None => {}
                }
            }
        }
    }

    grid.save_obj_file("grid.obj");
}
