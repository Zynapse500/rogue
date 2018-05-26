
use stopwatch::Stopwatch;

use std::{
    collections::{
        HashSet
    }
};

use graphics_3d::*;

use trap::Vector3;

use bounding_box::BoundingBox;

use camera::Camera;



pub struct GameState {
    running: bool,

    pressed_keys: HashSet<VirtualKeyCode>,

    camera: Camera,
    perspective: Projection,
    orthographic: Projection,

    velocity: Vector3,
    grounded: bool,

    boxes: Vec<BoundingBox>,

    crosshair: Crosshair
}


impl GameState {
    pub fn new() -> GameState {

        let size = 8;

        GameState {
            running: true,

            pressed_keys: HashSet::new(),

            camera: Camera::new(Vector3::new(0.0, 1.0, -2.0)),
            perspective: Projection::Perspective {
                fov: 70.0,
                aspect: 1.0,
                near: 0.01,
                far: 100.0,
            },
            orthographic: Projection::Orthographic {
                left: -1.0,
                right: 1.0,
                top: 1.0,
                bottom: -1.0,
                near: -1.0,
                far: 1.0,
            },


            velocity: Vector3::new(0.0, 0.0, 0.0),
            grounded: false,

            boxes: {
                let mut boxes = vec![
                    // BoundingBox::cube(Vector3::new(0.0, 0.0, 0.0), 0.5),

                    // Floor
                    BoundingBox {
                        min: Vector3::new(-size as f64 - 0.0, -0.75, -size as f64 - 0.0),
                        max: Vector3::new(size as f64 + 0.0, -0.5, size as f64 + 0.0),
                    },

                    // Walls
                    BoundingBox {
                        min: Vector3::new(-size as f64 - 0.5, -0.75, -size as f64 - 0.5),
                        max: Vector3::new(-size as f64 - 0.0, 3.0, size as f64 + 0.5),
                    },
                    BoundingBox {
                        min: Vector3::new(size as f64 + 0.0, -0.75, -size as f64 - 0.5),
                        max: Vector3::new(size as f64 + 0.5, 3.0, size as f64 + 0.5),
                    },
                    BoundingBox {
                        min: Vector3::new(-size as f64 - 0.5, -0.75, -size as f64 - 0.5),
                        max: Vector3::new(size as f64 + 0.5, 3.0, -size as f64 - 0.0),
                    },
                    BoundingBox {
                        min: Vector3::new(-size as f64 - 0.5, -0.75, size as f64 + 0.0),
                        max: Vector3::new(size as f64 + 0.5, 3.0, size as f64 + 0.5),
                    }
                ];

                for x in -size + 5..size - 5+1 {
                    for y in 2..7+3 {
                        for z in -size + 5..size - 5 +1 {
                            boxes.push(BoundingBox::cube(
                                Vector3::new(x as f64, y as f64, z as f64),
                                0.25
                            ));
                        }
                    }
                }

                boxes
            },

            crosshair: Crosshair {
                x: 0.0,
                y: 0.0,
                size: 25.0,
                width: 2.0,
            }
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }


    pub fn update(&mut self, dt: f64) {
        let mut move_direction = Vector3::new(0.0, 0.0, 0.0);
        let mut speed = 3.0;

        let right = self.camera.direction().cross(Vector3::new(0.0, 1.0, 0.0));

        if self.key_down(VirtualKeyCode::W) {
            move_direction += Vector3::new(0.0, 1.0, 0.0).cross(right);
        }
        if self.key_down(VirtualKeyCode::S) {
            move_direction -= Vector3::new(0.0, 1.0, 0.0).cross(right);
        }

        if self.key_down(VirtualKeyCode::A) {
            move_direction -= right
        }
        if self.key_down(VirtualKeyCode::D) {
            move_direction += right
        }

        if self.key_down(VirtualKeyCode::LShift) {
            speed *= 2.0;
        }

        if move_direction.dot(move_direction) > 0.0 {
            self.camera.position += speed * dt * move_direction.normal();
        }

        self.velocity.y -= dt * 8.0;
        self.camera.position += dt * self.velocity;


        self.check_collisions();
    }


    fn check_collisions(&mut self) {
        self.grounded = false;

        for b in self.boxes.iter() {
            let hull = BoundingBox {
                min: self.camera.position - Vector3::new(0.4, 1.6, 0.4),
                max: self.camera.position + Vector3::new(0.4, 0.3, 0.4)
            };

            if let Some(resolve) = hull.overlap(b) {
                self.camera.position += resolve;


                if resolve.y * self.velocity.y < 0.0 {
                    self.velocity.y = 0.0;

                    if resolve.y > 0.0 {
                        self.grounded = true;
                    }
                }
            }
        }
    }


    //
    // Rendering
    //

    pub fn draw(&mut self, frame: &mut Frame) {
        frame.clear(Color::new(0.01, 0.01, 0.01, 1.0));

        frame.set_projection(self.perspective);
        frame.set_view(self.camera.view());
//        frame.set_view(View::None);


        // All objects to draw
        let mut drawables: Vec<&Draw> = Vec::new();

        // Add the boxes
        for b in self.boxes.iter() {
            drawables.push(b);
        }

        // Draw all objects
        for drawable in drawables.into_iter() {
            frame.draw(drawable);
        }


        frame.set_projection(self.orthographic);
        frame.set_view(View::None);
        frame.draw(&self.crosshair);
    }



    //
    // Events
    //

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::WindowEvent { event, .. } => {
                self.handle_window_event(event);
            }
            Event::DeviceEvent { event, .. } => {
                self.handle_device_event(event);
            }

            _ => ()
        }
    }


    fn handle_window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::Closed => { self.close(); }

            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state, virtual_keycode: Some(virtual_keycode), ..
                }, ..
            } => {
                match state {
                    ElementState::Pressed => {
                        if self.pressed_keys.insert(virtual_keycode) {
                            self.key_pressed(virtual_keycode);
                        }
                    }
                    ElementState::Released => {
                        self.pressed_keys.remove(&virtual_keycode);
                        self.key_released(virtual_keycode);
                    }
                }
            }

            WindowEvent::Resized(w, h) => {
                self.size_changed(w, h);
            }

            _ => ()
        }
    }

    fn handle_device_event(&mut self, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                let sensitivity = 0.001;
                self.camera.rotate(-dx * sensitivity, -dy * sensitivity);
            }

            _ => ()
        }
    }


    fn close(&mut self) {
        self.running = false;
    }


    fn size_changed(&mut self, width: u32, height: u32) {
        self.perspective = Projection::Perspective {
            fov: 70.0,
            aspect: width as f64 / height as f64,
            near: 0.01,
            far: 100.0,
        };

        self.orthographic = Projection::Orthographic {
            left: 0.0,
            right: width as f64,
            top: 0.0,
            bottom: height as f64,
            near: -1.0,
            far: 1.0
        };

        self.crosshair.x = width as f64 / 2.0;
        self.crosshair.y = height as f64 / 2.0;
    }


    fn key_pressed(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Escape => self.close(),

            VirtualKeyCode::Space => {
                if self.grounded {
                    self.velocity.y += 4.5;
                }
            }

            _ => ()
        }
    }

    fn key_released(&mut self, key: VirtualKeyCode) {}

    fn key_down(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }
}





struct Crosshair {
    x: f64,
    y: f64,
    size: f64,
    width: f64,
}


impl Draw for Crosshair {
    fn triangulate(&self) -> Triangles {
        let r = self.width as f32 / 2.0;
        let s = self.size as f32 / 2.0;

        const COLOR: [f32; 4] = [0.1, 0.3, 0.4, 1.0];

        Triangles::IndexedList {
            vertices: vec![
                Vertex { position: [self.x as f32 - r, self.y as f32 - s, 0.0], color: COLOR },
                Vertex { position: [self.x as f32 + r, self.y as f32 - s, 0.0], color: COLOR },
                Vertex { position: [self.x as f32 + r, self.y as f32 + s, 0.0], color: COLOR },
                Vertex { position: [self.x as f32 - r, self.y as f32 + s, 0.0], color: COLOR },
                Vertex { position: [self.x as f32 - s, self.y as f32 - r, 0.0], color: COLOR },
                Vertex { position: [self.x as f32 - s, self.y as f32 + r, 0.0], color: COLOR },
                Vertex { position: [self.x as f32 + s, self.y as f32 + r, 0.0], color: COLOR },
                Vertex { position: [self.x as f32 + s, self.y as f32 - r, 0.0], color: COLOR },
            ],

            indices: vec![
                0, 1, 2, 2, 3, 0,
                4, 5, 6, 6, 7, 4
            ],
        }
    }
}
