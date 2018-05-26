
use trap::Vector3;
use graphics_3d::{
    PI,
    View
};

pub struct Camera {
    pub position: Vector3,

    yaw: f64,
    pitch: f64,
}

impl Camera {
    pub fn new(position: Vector3) -> Camera {
        Camera {
            position,
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    pub fn rotate(&mut self, dx: f64, dy: f64) {
        self.yaw += dx;
        self.pitch += dy;

        if self.pitch > PI * 0.49 { self.pitch = PI * 0.49 }
        if self.pitch < PI * -0.49 { self.pitch = PI * -0.49 }

        if self.yaw > PI * 2.0 { self.yaw -= PI * 2.0 };
        if self.yaw < PI * -2.0 { self.yaw += PI * 2.0 };
    }


    pub fn direction(&self) -> Vector3 {
        Vector3::new(
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.cos(),
        )
    }

    pub fn view(&self) -> View {
        View::LookAt {
            eye: self.position,
            target: (self.position + self.direction()),
            up: Vector3::new(0.0, 1.0, 0.0),
        }
    }
}