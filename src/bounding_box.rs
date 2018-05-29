
use std::f64::{
    INFINITY
};

use graphics_3d::{
    trap::{
        Vector3,
        Vector2
    },

    Draw,
    DrawCommand,
    Vertex,
    Color
};

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,

    pub color: Option<Color>
}

impl BoundingBox {
    pub fn cube(center: Vector3, size: f64) -> BoundingBox {
        BoundingBox {
            min: center - Vector3::new(size, size, size),
            max: center + Vector3::new(size, size, size),

            color: None
        }
    }


    #[allow(dead_code)]
    pub fn center(&self) -> Vector3 {
        0.5 * (self.min + self.max)
    }

    pub fn size(&self) -> Vector3 {
        self.max - self.min
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


    /// Return a rectangle as if this box was viewed along the y-axis
    pub fn project_y(&self) -> Rectangle {
        Rectangle {
            min: Vector2::new(self.min.x, self.min.z),
            max: Vector2::new(self.max.x, self.max.z),
        }
    }


    /// Return the distance a ray has to travel to hit this box and the normal if it hit
    pub fn hit_scan(&self, origin: Vector3, direction: Vector3) -> Option<(f64, Vector3)> {
        macro_rules! min { ($a:expr, $b:expr) => {if $a < $b {$a} else {$b}}; }
        macro_rules! max { ($a:expr, $b:expr) => {if $a > $b {$a} else {$b}}; }

        let time_min = (self.min - origin) * (1.0 / direction);
        let time_max = (self.max - origin) * (1.0 / direction);

        let mut time_entry = Vector3 {
            x: min!(time_min.x, time_max.x),
            y: min!(time_min.y, time_max.y),
            z: min!(time_min.z, time_max.z),
        };

        let mut time_exit = Vector3 {
            x: max!(time_min.x, time_max.x),
            y: max!(time_min.y, time_max.y),
            z: max!(time_min.z, time_max.z),
        };


        macro_rules! check_infinity {
            ($d:ident) => {
                if direction.$d == 0.0 {
                    if self.min.$d < origin.$d && origin.$d < self.max.$d {
                        time_entry.$d = -INFINITY;
                        time_exit.$d = INFINITY;
                    } else {
                        return None;
                    }
                }
            };
        }

        check_infinity!(x);
        check_infinity!(y);
        check_infinity!(z);

        let entry = {
            if time_entry.x > time_entry.y && time_entry.x > time_entry.z {
                time_entry.x
            } else if time_entry.y > time_entry.x && time_entry.y > time_entry.z {
                time_entry.y
            } else {
                time_entry.z
            }
        };

        let exit = {
            if time_exit.x < time_exit.y && time_exit.x < time_exit.z {
                time_exit.x
            } else if time_exit.y < time_exit.x && time_exit.y < time_exit.z {
                time_exit.y
            } else {
                time_exit.z
            }
        };


        if 0.0 < entry && entry < exit {
            if time_entry.x > time_entry.y && time_entry.x > time_entry.z {
                Some((entry, Vector3::new(if direction.x > 0.0 {-1.0} else {1.0}, 0.0, 0.0)))
            } else if time_entry.y > time_entry.x && time_entry.y > time_entry.z {
                Some((entry, Vector3::new(0.0, if direction.y > 0.0 {-1.0} else {1.0}, 0.0)))
            } else {
                Some((entry, Vector3::new(0.0, 0.0, if direction.z > 0.0 {-1.0} else {1.0})))
            }
        } else {
            None
        }
    }
}


impl Draw for BoundingBox {
    fn draw(&self) -> DrawCommand {
        let lx = self.min.x as f32;
        let gx = self.max.x as f32;
        let ly = self.min.y as f32;
        let gy = self.max.y as f32;
        let lz = self.min.z as f32;
        let gz = self.max.z as f32;

        DrawCommand::IndexedVertices {
            vertices: vec![
                Vertex {
                    position: [lx, ly, lz],
                    color: if let Some(color) = self.color { color.into() } else { [0.0, 0.0, 0.0, 1.0] }
                },
                Vertex {
                    position: [gx, ly, lz],
                    color: if let Some(color) = self.color { color.into() } else { [1.0, 0.0, 0.0, 1.0] }
                },
                Vertex {
                    position: [lx, gy, lz],
                    color: if let Some(color) = self.color { color.into() } else { [0.0, 1.0, 0.0, 1.0] }
                },
                Vertex {
                    position: [gx, gy, lz],
                    color: if let Some(color) = self.color { color.into() } else { [1.0, 1.0, 0.0, 1.0] }
                },

                Vertex {
                    position: [lx, ly, gz],
                    color: if let Some(color) = self.color { color.into() } else { [0.0, 0.0, 1.0, 1.0] }
                },
                Vertex {
                    position: [gx, ly, gz],
                    color: if let Some(color) = self.color { color.into() } else { [1.0, 0.0, 1.0, 1.0] }
                },
                Vertex {
                    position: [lx, gy, gz],
                    color: if let Some(color) = self.color { color.into() } else { [0.0, 1.0, 1.0, 1.0] }
                },
                Vertex {
                    position: [gx, gy, gz],
                    color: if let Some(color) = self.color { color.into() } else { [1.0, 1.0, 1.0, 1.0] }
                },
            ],
            indices: vec![
                0, 1, 2, 0, 2, 4, 0, 4, 1,
                3, 2, 1, 3, 1, 7, 3, 7, 2,
                5, 4, 7, 5, 7, 1, 5, 1, 4,
                6, 7, 4, 6, 4, 2, 6, 2, 7,
            ]
        }
    }
}



#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Rectangle {
    pub min: Vector2,
    pub max: Vector2
}


impl Rectangle {
    // Construct a rectangle between two points
    pub fn new(a: Vector2, b: Vector2) -> Rectangle {
        Rectangle {
            min: Vector2::new(
                if a.x < b.x {a.x} else {b.x},
                if a.y < b.y {a.y} else {b.y}
            ),
            max: Vector2::new(
                if a.x > b.x {a.x} else {b.x},
                if a.y > b.y {a.y} else {b.y}
            ),
        }
    }


    pub fn centered(center: Vector2, size: Vector2) -> Rectangle {
        Rectangle {
            min: center - size * 0.5,
            max: center + size * 0.5,
        }
    }

    pub fn center(&self) -> Vector2 {
        (self.min + self.max) * 0.5
    }

    pub fn size(&self) -> Vector2 {
        self.max - self.min
    }


    pub fn contains(&self, point: Vector2) -> bool {
        self.min.x < point.x && point.x < self.max.x &&
            self.min.y < point.y && point.y < self.max.y
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.max.x > other.min.x && other.max.x > self.min.x &&
            self.max.y > other.min.y && other.max.y > self.min.y
    }


    /// Splits the rectangle
    pub fn cut_by(self, mask: Rectangle) -> Vec<Rectangle> {
        let mut pieces = vec![self];

        if !self.intersects(&mask) {
            return pieces;
        }

        for i in 0..4 {
            let b = pieces[i];

            if b.min.x < mask.min.x && mask.min.x < b.max.x {
                pieces[i] = Rectangle {
                    min: b.min,
                    max: Vector2::new(mask.min.x, b.max.y),
                };

                pieces.push(Rectangle {
                    min: Vector2::new(mask.min.x, b.min.y),
                    max: b.max,
                });

                continue;
            }

            if b.min.x < mask.max.x && mask.max.x < b.max.x {
                pieces[i] = Rectangle {
                    min: Vector2::new(mask.max.x, b.min.y),
                    max: b.max,
                };

                pieces.push( Rectangle {
                    min: b.min,
                    max: Vector2::new(mask.max.x, b.max.y),
                });

                continue;
            }


            if b.min.y < mask.min.y && mask.min.y < b.max.y {
                pieces[i] = Rectangle {
                    min: b.min,
                    max: Vector2::new(b.max.x, mask.min.y),
                };

                pieces.push(Rectangle {
                    min: Vector2::new(b.min.x, mask.min.y),
                    max: b.max,
                });

                continue;
            }

            if b.min.y < mask.max.y && mask.max.y < b.max.y {
                pieces[i] = Rectangle {
                    min: Vector2::new(b.min.x, mask.max.y),
                    max: b.max,
                };

                pieces.push( Rectangle {
                    min: b.min,
                    max: Vector2::new(b.max.x, mask.max.y),
                });

                continue;
            }

            pieces.remove(i);
            break;
        }

        pieces
    }


    /// Extrudes the rectangle into the third dimension along the y-axis
    pub fn extrude_y(&self, min: f64, max: f64) -> BoundingBox {
        BoundingBox {
            min: Vector3::new(self.min.x, min, self.min.y),
            max: Vector3::new(self.max.x, max, self.max.y),
            color: None,
        }
    }
}
