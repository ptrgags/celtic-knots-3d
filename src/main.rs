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

fn make_connector() -> Mesh {
    let corner = Mesh::from_obj_file("data/basic/one_corner.obj");
    let rx = CubeRotation::rx();
    let rx2 = &rx * &rx;
    let rz = CubeRotation::rz();
    let rz2 = &rz * &rz;

    let mut result = Mesh::new();
    result.add_geometry(&corner);

    let one_rotated = corner.rotate(&rx);
    result.add_geometry(&one_rotated);

    let two_rotated = result.rotate(&rx2);
    result.add_geometry(&two_rotated);

    let four_rotated = result.rotate(&rz2);
    result.add_geometry(&four_rotated);

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

fn generate_twist_cell(cell_id: CellID) -> Mesh {
    orient_twist_cell(cell_id)
}

fn generate_connector(cell_id: CellID, rotation: CubeRotation) -> Mesh {
    let CellID(i, j, k) = cell_id;
    make_connector()
        .rotate(&rotation)
        .translate(&[i as f32, j as f32, k as f32])
}

fn generate_cell(cell_id: CellID) -> Mesh {
    let CellID(i, j, k) = cell_id;
    let parities = (i % 2, j % 2, k % 2);

    match parities {
        (1, 1, 0) | (0, 0, 1) => generate_twist_cell(cell_id),
        (1, 0, 1) | (0, 1, 0) 
            => generate_connector(cell_id, CubeRotation::identity()),
        (0, 0, 0) | (1, 1, 1)
            => generate_connector(cell_id, CubeRotation::ry()),
        (1, 0, 0) | (0, 1, 1) 
            => generate_connector(cell_id, CubeRotation::rz()),
        _ => panic!("Invalid cell parity")
    }
}

fn main() {
    const N: u32 = 11;
    const M: u32 = 7;
    const P: u32 = 5;
    let twist_tile = make_quad_twist();

    let mut grid = Mesh::new();
    for i in 0..N {
        for j in 0..M {
            for k in 0..P {
                let cell_id = CellID(i, j, k);
                let mesh = generate_cell(cell_id);
                grid.add_geometry(&mesh);
            }
        }
    }

    grid.save_obj_file("grid.obj");
}
