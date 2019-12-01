use std::io::{BufRead, BufReader};
use std::fs::File;

pub struct Vertex([f32; 3]);

pub enum Face {
    Triangle([usize; 3]),
    Quad([usize; 4]), 
}

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
}
