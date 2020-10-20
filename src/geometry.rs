use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

pub struct Point<T> {
    pub x: T,
    pub y: T, 
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point<i32> {
    pub fn convert_to_f32(&self) -> Point<f32> {
        Point {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

pub struct Triangle {
    pub p1: Point<i32>,
    pub p2: Point<i32>,
    pub p3: Point<i32>,
    pub color: u32,
    pub draw_method: DrawMethod,
}

pub enum DrawMethod {
    WireFrame,
    Fill,
}

pub struct Point3d {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

pub struct Mesh {
    pub points: Vec<Point3d>,
    pub position: Point3d,
}

pub fn read_obj(path: &str) -> Mesh {
    let mut points: Vec<Point3d> = Vec::new();

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    for line in file.lines() {
        let line = line.unwrap();
        
        if line.trim().is_empty() { continue; }

        let id = &line[0..2];
        let tokens: Vec<&str> = line[2..].split(' ').collect();

        match id {
            "f " => { }
            "v " => {
                // TODO: Should probably provide line number here.
                assert!(tokens.len() == 3 || tokens.len() == 4, "Vertex data in OBJ file bad!");
                
                let point = Point3d {
                    x: tokens[0].parse().unwrap(),
                    y: tokens[1].parse().unwrap(),
                    z: tokens[2].parse().unwrap(),
                    w: if tokens.len() == 4 { tokens[3].parse().unwrap() } else { 1.0 },
                };

                points.push(point);
            },
            "vn" => { },
            "vp" => { },
            "vt" => { },
            _ => (),
        }
    }

    Mesh {
        points,
        position: Point3d { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
    }
}
