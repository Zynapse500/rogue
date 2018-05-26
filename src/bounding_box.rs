
use trap::{
    Vector3
};

use graphics_3d::{
    Draw,
    Triangles,
    Vertex
};

pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}

impl BoundingBox {
    pub fn cube(center: Vector3, size: f64) -> BoundingBox {
        BoundingBox {
            min: center - Vector3::new(size, size, size),
            max: center + Vector3::new(size, size, size),
        }
    }


    pub fn intersect(&self, other: &BoundingBox) -> bool {
        self.max.x > other.min.x && other.max.x > self.min.x &&
            self.max.y > other.min.y && other.max.y > self.min.y &&
            self.max.z > other.min.z && other.max.z > self.min.z
    }


    pub fn overlap(&self, other: &BoundingBox) -> Option<Vector3> {
        if self.intersect(other) {
            let a = other.min - self.max;
            let b = other.max - self.min;

            let c = Vector3::new(
                if a.x.abs() < b.x.abs() { a.x } else { b.x },
                if a.y.abs() < b.y.abs() { a.y } else { b.y },
                if a.z.abs() < b.z.abs() { a.z } else { b.z },
            );

            let x = c.x.abs();
            let y = c.y.abs();
            let z = c.z.abs();

            if x < y && x < z {
                Some(Vector3::new(c.x, 0.0, 0.0))
            } else if y < x && y < z {
                Some(Vector3::new(0.0, c.y, 0.0))
            } else {
                Some(Vector3::new(0.0, 0.0, c.z))
            }
        } else {
            None
        }
    }
}


impl Draw for BoundingBox {
    fn triangulate(&self) -> Triangles {
        let lx = self.min.x as f32;
        let gx = self.max.x as f32;
        let ly = self.min.y as f32;
        let gy = self.max.y as f32;
        let lz = self.min.z as f32;
        let gz = self.max.z as f32;

        Triangles::IndexedList {
            vertices: vec![
                Vertex {position: [lx, ly, lz], color: [0.0, 0.0, 0.0, 1.0]},
                Vertex {position: [gx, ly, lz], color: [1.0, 0.0, 0.0, 1.0]},
                Vertex {position: [lx, gy, lz], color: [0.0, 1.0, 0.0, 1.0]},
                Vertex {position: [gx, gy, lz], color: [1.0, 1.0, 0.0, 1.0]},

                Vertex {position: [lx, ly, gz], color: [0.0, 0.0, 1.0, 1.0]},
                Vertex {position: [gx, ly, gz], color: [1.0, 0.0, 1.0, 1.0]},
                Vertex {position: [lx, gy, gz], color: [0.0, 1.0, 1.0, 1.0]},
                Vertex {position: [gx, gy, gz], color: [1.0, 1.0, 1.0, 1.0]},
            ],
            indices: vec![
                0, 1, 2, 0, 2, 4, 0, 4, 1,
                3, 2, 1, 3, 1, 7, 3, 7, 2,
                5, 4, 7, 5, 7, 1, 5, 1, 4,
                6, 7, 4, 6, 4, 2, 6, 2, 7,
            ]
        }
    }


    /*fn triangulate(&self) -> Triangles {
        let mut vertices = Vec::new();
        vertices.reserve(8);

        let mut indices = Vec::new();
        indices.reserve(36);


        for i in 0..8 {
            let x = i % 2;
            let y = (i >> 1) % 2;
            let z = (i >> 2) % 2;

            vertices.push(Vertex {
                position: [
                    if x == 0 { self.min.x } else { self.max.x } as f32,
                    if y == 0 { self.min.y } else { self.max.y } as f32,
                    if z == 0 { self.min.z } else { self.max.z } as f32
                ],
                color: [
                    x as f32,
                    y as f32,
                    z as f32,
                    1.0
                ],
            });

            if i == 0 || i == 3 || i == 5 || i == 6 {
                // The indices of the adjacent points
                let n = [
                    (1 - x) + 2 * (y) + 4 * (z),
                    (x) + 2 * (1 - y) + 4 * (z),
                    (x) + 2 * (y) + 4 * (1 - z),
                ];

                for j in 0..3 {
                    let a = n[j];
                    let b = n[if j == 2 { 0 } else { j + 1 }];

                    indices.push(i);
                    indices.push(a);
                    indices.push(b);
                }
            }
        }

        println!("indices: {:?}\n\n", indices);


        Triangles::IndexedList {
            vertices,

            indices,
        }
    }*/
}

