
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


mod world;
use self::world::*;



pub struct GameState {
    time: f64,
    running: bool,

    pressed_keys: HashSet<VirtualKeyCode>,

    camera: Camera,
    perspective: Projection,
    orthographic: Projection,

    velocity: Vector3,
    grounded: bool,

    boxes: Vec<BoundingBox>,
    size: f64,

    crosshair: Crosshair,


    world: World,
    particles: Vec<Particle>
}


impl GameState {
    pub fn new() -> GameState {

        GameState {
            time: 0.0,
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

            boxes: vec![],
            size: 1.0,

            crosshair: Crosshair {
                x: 0.0,
                y: 0.0,
                size: 25.0,
                width: 2.0,
            },

            world: World::new(),
            particles: Vec::new()
        }
    }

    fn get_boxes(&self) -> Vec<BoundingBox> {
        let count = 4;

        let mut boxes = Vec::new();

        let margin: i32 = 0;

        for x in -count + margin..count - margin+1 {
            for y in 3..count*2+4 {
                for z in -count + margin..count - margin +1 {
                    boxes.push(BoundingBox::cube(
                        Vector3::new(x as f64, y as f64, z as f64),
                        ((self.time + (x + y + z) as f64).sin() * (self.time * 0.5).cos() * 0.5 + 0.5) * 0.35 + 0.05
                    ));
                }
            }
        }

        boxes
    }


    pub fn running(&self) -> bool {
        self.running
    }


    pub fn update(&mut self, dt: f64) {
        self.time += dt;

        self.boxes = Self::get_boxes(self);

        self.check_player_movement(dt);

        self.world.explore(self.camera.position);


        self.update_particles(dt);


        if self.key_down(VirtualKeyCode::Q) {
            self.size -= 4.0 * dt;

            if self.size < 0.1 {
                self.size = 0.1;
            }
        }

        if self.key_down(VirtualKeyCode::E) {
            self.size += 4.0 * dt;

            if self.size > 4.0 {
                self.size = 4.0;
            }
        }


        self.check_collisions();
    }


    fn check_player_movement(&mut self, dt: f64) {
        let mut move_direction = Vector3::new(0.0, 0.0, 0.0);
        let mut speed = 6.0 * self.size;

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
    }

    fn update_particles(&mut self, dt: f64) {
        let mut i = 0;
        while i < self.particles.len() {
            let remove = {
                let particle = &mut self.particles[i];
                particle.position += dt * particle.velocity;

                particle.size -= 0.5 * dt;
                if particle.size < 0.0 {
                    true
                } else {
                    false
                }
            };

            if remove {
                self.particles.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn check_collisions(&mut self) {
        self.grounded = false;

        for collider in self.world.get_colliders() {
            let hull = self.get_hull();

            if let Some(resolve) = hull.overlap(collider) {
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


    fn get_hull(&self) -> BoundingBox {
        let width = 0.4 * self.size;
        let height = 1.8 * self.size;

        BoundingBox {
            min: self.camera.position - Vector3::new(width, height * 5.0 / 6.0, width),
            max: self.camera.position + Vector3::new(width, height / 6.0, width),
            color: Some(Color::new(1.0, 0.0, 0.0, 1.0))
        }
    }


    //
    // Rendering
    //

    pub fn draw(&mut self, frame: &mut Frame) {
        frame.clear(Color::new(0.01, 0.01, 0.01, 1.0));

        //frame.render_pass(|mut pass|{
            frame.set_projection(self.perspective);
            frame.set_view(self.camera.view());

            self.draw_scene(frame);
        //});

        //frame.render_pass(|mut pass| {
            self.draw_ui(frame);
        //});
    }


    fn draw_scene(&mut self, pass: &mut Frame) {
        // All objects to draw
        let mut drawables: Vec<&Draw> = Vec::new();

        // Add the boxes
        for b in self.boxes.iter() {
            drawables.push(b);
        }

        // Draw the world
        pass.draw(&self.world);

        for particle in self.particles.iter() {
            pass.draw(particle);
        }

        // Draw all objects
        for drawable in drawables.into_iter() {
            pass.draw(drawable);
        }
    }


    fn draw_ui(&mut self, pass: &mut Frame) {
        pass.set_projection(self.orthographic);
        pass.set_view(View::None);

        pass.draw(&self.crosshair);


        self.draw_minimap(pass, 300, 300);
    }


    fn draw_minimap(&mut self, pass: &mut Frame, width: u32, height: u32) {
        let distance = 100.0;

        pass.set_viewport(Some(Rect {
            left: 0,
            bottom: 0,
            width,
            height
        }));

        pass.clear(Color::new(0.0, 0.0, 0.0, 1.0));

        pass.set_projection(Projection::Perspective {
            fov: 70.0,
            aspect: width as f64 / height as f64,
            near: 0.01,
            far: distance + 10.0,
        });

        let up = Vector3::new(0.0, 1.0, 0.0);

        pass.set_view(View::LookAt {
            eye: Vector3::new(self.camera.position.x, distance, self.camera.position.z),
            target: self.camera.position,
            up: up.cross(self.camera.direction()).cross(up)
        });

        self.draw_scene(pass);

        pass.draw(&self.get_hull());
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

            WindowEvent::MouseInput {
                state, button, ..
            } => {
                match state {
                    ElementState::Pressed => {
                        self.mouse_pressed(button);
                    }

                    ElementState::Released => {
                        self.mouse_released(button);
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
                    self.velocity.y += 4.5 * self.size.sqrt();
                }
            }

            VirtualKeyCode::Tab => {
                self.world.explore(self.camera.position);
            }

            VirtualKeyCode::R => {
                self.camera.position.x = 0.0;
                self.camera.position.y = self.get_hull().size().y;
                self.camera.position.z = 0.0;
            }

            _ => ()
        }
    }

    fn key_released(&mut self, key: VirtualKeyCode) {}

    fn key_down(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }


    fn mouse_pressed(&mut self, button: MouseButton) {
        let result = self.world.get_colliders()
            .filter_map(|c|{
                c.hit_scan(self.camera.position, self.camera.direction())
            }).min_by(|a, b| {
            a.0.partial_cmp(&b.0).unwrap()
        });

        if let Some((distance, normal)) = result {
            let hit = self.camera.position + distance * self.camera.direction();
            let reflection = self.camera.direction().reflect(normal);


            use rand::{
                thread_rng,
                Rng
            };

            let mut rng = thread_rng();

            let perp_x = reflection.cross(Vector3::new(
                rng.gen_range(0.01, 1.0),
                rng.gen_range(0.01, 1.0),
                rng.gen_range(0.01, 1.0)
            )).normal();

            let perp_y = perp_x.cross(reflection);

            for _ in 0..10 {
                let theta = rng.gen_range(0.0, 2.0 * PI);
                let eta = rng.gen_range(0.0, PI / 4.0);

                let r = rng.gen_range(0.0, 0.4);
                let (dy, dx) = theta.sin_cos();

                let particle = Particle {
                    position: hit,
                    velocity: 8.0 * (reflection + r * dx * perp_x + r * dy * perp_y).normal(),
                    size: rng.gen_range(0.05, 0.2),
                };

                self.particles.push(particle);
            }
        }
    }

    fn mouse_released(&mut self, button: MouseButton) {

    }
}





struct Crosshair {
    x: f64,
    y: f64,
    size: f64,
    width: f64,
}


impl Draw for Crosshair {
    fn draw(&self) -> DrawCommand {
        let r = self.width as f32 / 2.0;
        let s = self.size as f32 / 2.0;

        const COLOR: [f32; 4] = [0.1, 0.3, 0.4, 1.0];

        DrawCommand::IndexedVertices {
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



struct Particle {
    position: Vector3,
    velocity: Vector3,
    size: f64
}

impl Draw for Particle {
    fn draw(&self) -> DrawCommand {
        BoundingBox::cube(self.position, self.size).draw()
    }
}
