use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::collections::{HashMap};

use crate::primitives::{Vertex, Face};
use Face::{Triangle, Quad};
use crate::cube_rotations::CubeRotation;

pub struct Mesh {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new()
        }
    }

    pub fn from_obj_file(fname: &str) -> Self {
        let mut file = File::open(fname).expect("Couldn't open file");
        let (vertices, faces) = Mesh::parse_obj(&mut file);
        
        Self {
            vertices, 
            faces
        }
    }

    fn parse_obj(file: &mut File) -> (Vec<Vertex>, Vec<Face>) {
        let reader = BufReader::new(file);
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        for line in reader.lines() { 
            let line_str = line.expect("could not read OBJ file line");
            let tokens: Vec<&str> = line_str.split(" ").collect();
            let tag = tokens[0];

            if tag == "v" {
                let vertex = Mesh::parse_vertex(&tokens[1..]);
                vertices.push(vertex);
            } else if tag == "f" {
                let face = Mesh::parse_face(&tokens[1..]);
                faces.push(face);
            } else {
                // TODO: Support normals someday. But not today.
            }
        }

        (vertices, faces)
    }

    fn parse_vertex(tokens: &[&str]) -> Vertex {
        if tokens.len() != 3 {
            panic!("vertices must have 3 components");
        }

        let x: f32 = tokens[0].parse().expect("invalid x coordinate");
        let y: f32 = tokens[1].parse().expect("invalid x coordinate");
        let z: f32 = tokens[2].parse().expect("invalid x coordinate");

        Vertex([x, y, z])
    }

    fn parse_face(tokens: &[&str]) -> Face {
        if tokens.len() < 3 || tokens.len() > 4 {
            panic!("only triagles and quads are supported")
        }

        let v1 = Mesh::parse_face_index(&tokens[0]);
        let v2 = Mesh::parse_face_index(&tokens[1]);
        let v3 = Mesh::parse_face_index(&tokens[2]);

        if tokens.len() == 3 {
            return Face::Triangle([v1, v2, v3])
        }

        let v4 = Mesh::parse_face_index(tokens[3]);
        Face::Quad([v1, v2, v3, v4])
    }

    fn parse_face_index(index_str: &str) -> usize {
        let indices: Vec<&str> = index_str.split("/").collect(); 
        let vertex: usize = indices[0].parse().expect("invalid face index");
        // TODO: maybe someday uvs and normals?

        // 1-indexed values -> 0-indexed values
        vertex - 1
    }

    pub fn rotate(&self, rotation: &CubeRotation) -> Self {
        let rotated_vertices: Vec<Vertex> = self.vertices.iter().map(|v| {
            rotation * v
        }).collect();

        Self {
            vertices: rotated_vertices,
            faces: self.faces.clone()
        }
    }

    pub fn translate(&self, translation: &[f32; 3]) -> Self {
        let translated_vertices: Vec<Vertex> = self.vertices.iter().map(|v| {
            let [dx, dy, dz] = translation;
            let Vertex([x, y, z]) = v;

            Vertex([x + dx, y + dy, z + dz])
        }).collect();
            
        Self {
            vertices: translated_vertices,
            faces: self.faces.clone()
        }
    }

    pub fn add_geometry(&mut self, other: &Self) {
        let n = self.vertices.len();
            
        self.vertices.extend_from_slice(&other.vertices[..]);
        
        for face in other.faces.iter() {
            let new_face = match face {
                Quad([v1, v2, v3, v4]) => Quad([v1 + n, v2 + n, v3 + n, v4 + n]),
                Triangle([v1, v2, v3]) => Triangle([v1 + n, v2 + n, v3 + n])
            };
            self.faces.push(new_face);
        }
    }

    /**
     * Simplified clipping based on a plane that cuts through the origin and
     * does not intersect the geometry. this is used for creating the
     * connector end caps
     */
    pub fn simple_clip(&self, normal: [f32; 3]) -> Self {
        // This program doesn't have a Vector class, but I added dot() to
        // Vertex
        let normal_vertex = Vertex(normal);

        // Mark the vertices to keep and their new indices
        let mut keepers: HashMap<usize, usize> = HashMap::new();
        let mut new_vertices: Vec<Vertex> = Vec::new();
        for (i, vertex) in self.vertices.iter().enumerate() {
            let product = vertex.dot(&normal_vertex);
            // If we're "inside" the boundary, keep the vertex but give
            // it a new index
            if product <= 0.0 {
                new_vertices.push(vertex.clone());
                keepers.insert(i, new_vertices.len() - 1);
            }
        }

        // Go through the faces and see which ones to keep
        let mut new_faces: Vec<Face> = Vec::new();
        for face in self.faces.iter() {
            let new_face = reindex_face(&face, &keepers);

            if let Some(f) = new_face {
                new_faces.push(f);
            }
        }

        Self {
            vertices: new_vertices,
            faces: new_faces,
        }
    }

    pub fn save_obj_file(&self, fname: &str) {
        let mut file = File::create(fname)
            .expect("Could not open output OBJ file");

        for Vertex([x, y, z]) in self.vertices.iter() {
            let line = format!("v {} {} {}\n", x, y, z);
            file.write_all(line.as_bytes()).expect("Could not write vertex");
        }

        for face in self.faces.iter() {
            let line = match face {
                Quad([v1, v2, v3, v4]) => format!(
                    "f {} {} {} {}\n", v1 + 1, v2 + 1, v3 + 1, v4 + 1),
                Triangle([v1, v2, v3]) => format!(
                    "f {} {} {}\n", v1 + 1, v2 + 1, v3 + 1)
            };
            file.write_all(line.as_bytes()).expect("Could not write face");
        }
    }
}

fn reindex_face(face: &Face, keepers: &HashMap<usize, usize>) -> Option<Face> {
    let accept = match face {
        Triangle(vertices) => accept_face(&vertices[..], &keepers),
        Quad(vertices) => accept_face(&vertices[..], &keepers),
    };

    if !accept {
        return None;
    }

    match face {
        Triangle([v1, v2, v3]) 
            => Some(Triangle([keepers[v1], keepers[v2], keepers[v3]])),
        Quad([v1, v2, v3, v4]) 
            => Some(Quad([keepers[v1], keepers[v2], keepers[v3], keepers[v4]])),
    }
}

fn accept_face(face_indices: &[usize], keepers: &HashMap<usize, usize>) -> bool {
    for index in face_indices.iter() {
        if let None = keepers.get(index) {
            return false;
        }
    }

    return true;
}
