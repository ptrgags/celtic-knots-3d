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

fn make_connector_cap(_rotation: CubeRotation) -> Mesh {
    let rz = CubeRotation::rz();
    let corner = Mesh::from_obj_file("data/basic/one_corner.obj");
    let corner2 = corner.rotate(&rz);
    let corner3 = corner2.rotate(&rz);
    let corner4 = corner3.rotate(&rz);

    let mut result = Mesh::new();
    result.add_geometry(&corner);
    result.add_geometry(&corner2);
    result.add_geometry(&corner3);
    result.add_geometry(&corner4);

    result
}

fn make_end_cap() -> Mesh {
    Mesh::from_obj_file("data/basic/end_cap.obj")
}

fn make_edge_cap() -> Mesh {
    Mesh::from_obj_file("data/basic/one_edge.obj")
}

#[derive(Copy, Clone, Debug)]
struct CellID(u32, u32, u32);

#[derive(Debug)]
enum RangeComparison {
    Min,
    Between,
    Max
}

#[derive(Debug)]
struct BoundsClassification(
    RangeComparison, 
    RangeComparison,
    RangeComparison
);

#[derive(Copy, Clone)]
struct Range(u32, u32);

#[derive(Copy, Clone)]
struct Bounds(Range, Range, Range);

impl Bounds {
    pub fn new(n: u32, m: u32, p: u32) -> Self {
        Bounds(
            Range(0, n - 1),
            Range(0, m - 1),
            Range(0, p - 1)
        )
    }
}

fn compare_range(x: u32, range: Range) -> RangeComparison {
    let Range(min_val, max_val) = range;
    use RangeComparison::{Min, Max, Between};
    if x == min_val {
        Min
    } else if x == max_val {
        Max
    } else {
        Between
    }
}

fn classify_bounds(cell_id: CellID, bounds: Bounds) -> BoundsClassification {
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

fn generate_end_cap(cell_id: CellID, rotation: CubeRotation) -> Mesh {
    let CellID(i, j, k) = cell_id;
    return make_end_cap()
        .rotate(&rotation)
        .translate(&[i as f32, j as f32, k as f32])
}

fn generate_edge_cap(cell_id: CellID, rotation: CubeRotation) -> Mesh {
    let CellID(i, j, k) = cell_id;
    return make_edge_cap()
        .rotate(&rotation)
        .translate(&[i as f32, j as f32, k as f32])
}

fn generate_twist_cell(cell_id: CellID, bounds: Bounds) -> Mesh {
    let classification = classify_bounds(cell_id, bounds);
    use RangeComparison::{Min, Max, Between};
    match classification {
        BoundsClassification(Min, Between, Between) 
            => generate_end_cap(cell_id, CubeRotation::ry3()),
        BoundsClassification(Max, Between, Between)
            => generate_end_cap(cell_id, CubeRotation::ry()),
        BoundsClassification(Between, Min, Between)
            => generate_end_cap(cell_id, CubeRotation::rx()),
        BoundsClassification(Between, Max, Between)
            => generate_end_cap(cell_id, CubeRotation::rx3()),
        BoundsClassification(Between, Between, Min)
            => generate_end_cap(cell_id, CubeRotation::ry2()),
        BoundsClassification(Between, Between, Max)
            => generate_end_cap(cell_id, CubeRotation::identity()),
        BoundsClassification(Min, Min, Between)
            => generate_edge_cap(cell_id, CubeRotation::identity()),
        BoundsClassification(Min, Max, Between)
            => generate_edge_cap(cell_id, CubeRotation::rz3()),
        BoundsClassification(Max, Min, Between)
            => generate_edge_cap(cell_id, CubeRotation::rz()),
        BoundsClassification(Max, Max, Between)
            => generate_edge_cap(cell_id, CubeRotation::rz2()),
        _ => orient_twist_cell(cell_id)
    }
}

fn generate_connector(cell_id: CellID, rotation: CubeRotation) -> Mesh {
    make_connector()
        .rotate(&rotation)
}

fn generate_connector_cell(
        cell_id: CellID, rotation: CubeRotation, bounds: Bounds) -> Mesh {

    let connector = generate_connector(cell_id, rotation);

    let classification = classify_bounds(cell_id, bounds);
    use RangeComparison::{Min, Max, Between};
    let clipped_connector = match classification {
        BoundsClassification(Min, Between, Between) 
            => connector.simple_clip([-1.0, 0.0, 0.0]),
        BoundsClassification(Max, Between, Between)
            => connector.simple_clip([1.0, 0.0, 0.0]),
        BoundsClassification(Between, Min, Between)
            => connector.simple_clip([0.0, -1.0, 0.0]),
        BoundsClassification(Between, Max, Between)
            => connector.simple_clip([0.0, 1.0, 0.0]),
        BoundsClassification(Between, Between, Min)
            => connector.simple_clip([0.0, 0.0, -1.0]),
        BoundsClassification(Between, Between, Max)
            => connector.simple_clip([0.0, 0.0, 1.0]),
        // 12 edges
        BoundsClassification(Min, Between, Min) 
            => connector
                .simple_clip([-1.0, 0.0, 0.0])
                .simple_clip([0.0, 0.0, -1.0]),
        BoundsClassification(Min, Between, Max) 
            => connector
                .simple_clip([-1.0, 0.0, 0.0])
                .simple_clip([0.0, 0.0, 1.0]),
        BoundsClassification(Max, Between, Min) 
            => connector
                .simple_clip([1.0, 0.0, 0.0])
                .simple_clip([0.0, 0.0, -1.0]),
        BoundsClassification(Max, Between, Max) 
            => connector
                .simple_clip([1.0, 0.0, 0.0])
                .simple_clip([0.0, 0.0, 1.0]),
        BoundsClassification(Between, Min, Min) 
            => connector
                .simple_clip([0.0, -1.0, 0.0])
                .simple_clip([0.0, 0.0, -1.0]),
        BoundsClassification(Between, Min, Max) 
            => connector
                .simple_clip([0.0, -1.0, 0.0])
                .simple_clip([0.0, 0.0, 1.0]),
        BoundsClassification(Between, Max, Min) 
            => connector
                .simple_clip([0.0, 1.0, 0.0])
                .simple_clip([0.0, 0.0, -1.0]),
        BoundsClassification(Between, Max, Max) 
            => connector
                .simple_clip([0.0, 1.0, 0.0])
                .simple_clip([0.0, 0.0, 1.0]),
        BoundsClassification(Min, Min, Between) 
            => connector
                .simple_clip([-1.0, 0.0, 0.0])
                .simple_clip([0.0, -1.0, 0.0]),
        BoundsClassification(Min, Max, Between) 
            => connector
                .simple_clip([-1.0, 0.0, 0.0])
                .simple_clip([0.0, 1.0, 0.0]),
        BoundsClassification(Max, Min, Between) 
            => connector
                .simple_clip([1.0, 0.0, 0.0])
                .simple_clip([0.0, -1.0, 0.0]),
        BoundsClassification(Max, Max, Between) 
            => connector
                .simple_clip([1.0, 0.0, 0.0])
                .simple_clip([0.0, 1.0, 0.0]),
        // 8 corners
        BoundsClassification(Min, Min, Min) 
            => connector
                .simple_clip([-1.0, 0.0, 0.0])
                .simple_clip([0.0, -1.0, 0.0])
                .simple_clip([0.0, 0.0, -1.0]),
        BoundsClassification(Min, Min, Max) 
            => connector
                .simple_clip([-1.0, 0.0, 0.0])
                .simple_clip([0.0, -1.0, 0.0])
                .simple_clip([0.0, 0.0, 1.0]),
        BoundsClassification(Min, Max, Min) 
            => connector
                .simple_clip([-1.0, 0.0, 0.0])
                .simple_clip([0.0, 1.0, 0.0])
                .simple_clip([0.0, 0.0, -1.0]),
        BoundsClassification(Min, Max, Max) 
            => connector
                .simple_clip([-1.0, 0.0, 0.0])
                .simple_clip([0.0, 1.0, 0.0])
                .simple_clip([0.0, 0.0, 1.0]),
        BoundsClassification(Max, Min, Min) 
            => connector
                .simple_clip([1.0, 0.0, 0.0])
                .simple_clip([0.0, -1.0, 0.0])
                .simple_clip([0.0, 0.0, -1.0]),
        BoundsClassification(Max, Min, Max) 
            => connector
                .simple_clip([1.0, 0.0, 0.0])
                .simple_clip([0.0, -1.0, 0.0])
                .simple_clip([0.0, 0.0, 1.0]),
        BoundsClassification(Max, Max, Min) 
            => connector
                .simple_clip([1.0, 0.0, 0.0])
                .simple_clip([0.0, 1.0, 0.0])
                .simple_clip([0.0, 0.0, -1.0]),
        BoundsClassification(Max, Max, Max) 
            => connector
                .simple_clip([1.0, 0.0, 0.0])
                .simple_clip([0.0, 1.0, 0.0])
                .simple_clip([0.0, 0.0, 1.0]),
        _ => connector
    };
        

    let CellID(i, j, k) = cell_id;
    clipped_connector.translate(&[i as f32, j as f32, k as f32]) 
}

fn generate_cell(cell_id: CellID, bounds: Bounds) -> Mesh {
    let CellID(i, j, k) = cell_id;
    let parities = (i % 2, j % 2, k % 2);

    match parities {
        (1, 1, 0) | (0, 0, 1) => generate_twist_cell(cell_id, bounds),
        (1, 0, 1) | (0, 1, 0) 
            => generate_connector_cell(
                cell_id, CubeRotation::identity(), bounds),
        (0, 0, 0) | (1, 1, 1)
            => generate_connector_cell(cell_id, CubeRotation::ry(), bounds),
        (1, 0, 0) | (0, 1, 1) 
            => generate_connector_cell(cell_id, CubeRotation::rz(), bounds),
        _ => panic!("Invalid cell parity")
    }
}

fn main() {
    const N: u32 = 11;
    const M: u32 = 7;
    const P: u32 = 5;
    let twist_tile = make_quad_twist();
    let bounds = Bounds::new(N, M, P);

    let mut grid = Mesh::new();
    for i in 0..N {
        for j in 0..M {
            for k in 0..P {
                let cell_id = CellID(i, j, k);
                let mesh = generate_cell(cell_id, bounds);
                grid.add_geometry(&mesh);
            }
        }
    }

    grid.save_obj_file("grid.obj");
}
