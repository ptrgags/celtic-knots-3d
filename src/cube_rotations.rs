use std::ops::Mul;
use std::fmt::{Debug, Formatter, Result};

use crate::primitives::Vertex;

pub struct CubeRotation {
    axes: [usize; 3],
    orientations: [i8; 3],
}

impl CubeRotation {
    pub fn new(axes: [usize; 3], orientations: [i8; 3]) -> Self {
        Self {
            axes,
            orientations
        }
    }

    pub fn identity() -> Self {
        Self::new(
            [0, 1, 2], 
            [1, 1, 1])
    }

    pub fn rz() -> Self {
        Self::new(
            [1, 0, 2],
            [-1, 1, 1]) 
    }

    pub fn rx() -> Self {
        Self::new(
            [0, 2, 1],
            [1, -1, 1])
    }
    
    pub fn ry() -> Self {
        Self::new(
            [2, 1, 0],
            [1, 1, -1])
    }
}

fn axis_label(axis: usize, orientation: i8) -> String {
    const AXES: [&str; 3] = ["x", "y", "z"];
    let axis_label = AXES[axis];

    let sign = if orientation == 1 { "" } else { "-" };
    format!("{}{}", sign, axis_label)
}

impl Debug for CubeRotation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, 
            "[{} {} {}]",
            axis_label(self.axes[0], self.orientations[0]),
            axis_label(self.axes[1], self.orientations[1]),
            axis_label(self.axes[2], self.orientations[2]))
    }
}

impl<'a, 'b> Mul<&'b CubeRotation> for &'a CubeRotation {
    type Output = CubeRotation;
    fn mul(self, other: &'b CubeRotation) -> CubeRotation {
        let mut axes: [usize; 3] = [0; 3];
        let mut orientations: [i8; 3] = [1; 3];

        for i in 0..3 {
            let ax = self.axes[i];

            axes[i] = other.axes[ax];
            orientations[i] = 
                self.orientations[i] * other.orientations[ax];
        }

        CubeRotation {
            axes,
            orientations
        }
    }
}

impl<'a, 'b> Mul<&'b Vertex> for &'a CubeRotation {
    type Output = Vertex;
    fn mul(self, other: &'b Vertex) -> Vertex {
        let Vertex(components) = other;
        let mut new_components: [f32; 3] = [0.0; 3];
        for i in 0..3 {
            let axis = self.axes[i];
            let orientation = self.orientations[i];
            new_components[i] = (orientation as f32) * components[axis];
        }

        Vertex(new_components)
    }
}
