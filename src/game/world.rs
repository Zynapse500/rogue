use graphics_3d::{
    trap::{
        Vector3,
        Vector2,
    },
    Draw,
    DrawCommand,
    Color
};

use rand::{
    thread_rng,
    Rng
};

use bounding_box::{
    BoundingBox,
    Rectangle,
};




const PASSAGE_WIDTH: f64 = 3.0;
const WALL_THICKNESS: f64 = 0.5;
const WALL_HEIGHT: f64 = 5.0;



/// A world made up of rooms
pub struct World {
    rooms: Vec<Room>,
    passages: Vec<Room>
}


impl World {
    pub fn new() -> World {
        World {
            rooms: vec![
                Room::new(Rectangle {
                    min: Vector2::new(-8.0, -8.0),
                    max: Vector2::new(8.0, 8.0),
                })
            ],
            passages: Vec::new()
        }
    }

    pub fn get_colliders<'a>(&'a self) -> impl Iterator<Item=&'a BoundingBox> {
        self.rooms.iter()
            .chain(self.passages.iter())
            .flat_map(|room| { room.boxes.iter() })
    }


    pub fn explore(&mut self, position: Vector3) {
        if let Some(room_index) = self.find_room_index(position) {
            if self.rooms[room_index].explored {
                return;
            } else {
                self.rooms[room_index].explored = true;
                if let Some(ref mut b) = self.rooms[room_index].boxes.first_mut() {
                    b.color = Some(Color::new(0.0, 1.0, 0.0, 1.0));
                }
            }

            let mut new_rooms = Vec::new();
            let mut rng = thread_rng();


            // Attempt to add 4 new rooms
            for _ in 0..4 {
                let (dx, dy) = {
                    let direction = rng.gen_range(0, 4);

                    match direction {
                        0 => (1, 0),
                        1 => (0, 1),
                        2 => (-1, 0),
                        3 => (0, -1),

                        _ => panic!()
                    }
                };


                let floor = self.rooms[room_index].floor;
                let distance = rng.gen_range(
                    2.0 * WALL_THICKNESS + 2.0,
                    2.0 * WALL_THICKNESS + 6.0
                );

                let new_size = Vector2::new(
                    rng.gen_range(8.0, 24.0),
                    rng.gen_range(8.0, 24.0)
                );
                let new_center = floor.center() +
                    Vector2::new(dx as f64, dy as f64) *
                        (0.5 * (floor.size() + new_size) + Vector2::new(distance, distance));


                let new_floor = if dy == 0 {
                    let min_y = floor.min.y + PASSAGE_WIDTH + 1.0;
                    let max_y = floor.max.y - PASSAGE_WIDTH - 1.0;

                    let min = rng.gen_range(min_y - new_size.y, max_y);
                    let max = min + new_size.y;

                    Rectangle::centered(
                        Vector2::new(new_center.x, 0.5 * (min + max)),
                        new_size
                    )
                } else {
                    let min_x = floor.min.x + PASSAGE_WIDTH + 1.0;
                    let max_x = floor.max.x - PASSAGE_WIDTH - 1.0;

                    let min = rng.gen_range(min_x - new_size.x, max_x);
                    let max = min + new_size.x;

                    Rectangle::centered(
                        Vector2::new(0.5 * (min + max), new_center.y),
                        new_size
                    )
                };


                let margin = 2.0 * WALL_THICKNESS + 0.8;
                let tmp_floor = Rectangle::centered(
                    new_floor.center(),
                    new_size + 2.0 * Vector2::new(margin, margin)
                );

                if !self.rooms.iter()
                    .any(|room|{tmp_floor.intersects(&room.floor)}) {
                    new_rooms.push(self.rooms.len());
                    self.rooms.push(Room::new(new_floor));
                }
            }


            // Add passages between rooms
            let old = self.rooms[room_index].floor;
            for i in new_rooms.into_iter() {
                let new = self.rooms[i].floor;

                // Calculate two opposite corners of the floor
                let a = Vector2::new(
                    if old.min.x > new.min.x {old.min.x} else {new.min.x},
                    if old.min.y > new.min.y {old.min.y} else {new.min.y}
                );
                let b = Vector2::new(
                    if old.max.x < new.max.x {old.max.x} else {new.max.x},
                    if old.max.y < new.max.y {old.max.y} else {new.max.y}
                );

                // Construct the floor
                let mut floor = Rectangle::new(a, b);

                if old.min.x < new.max.x && new.min.x < old.max.x {
                    // Intersect on x
                    floor.min.x += rng.gen_range(0.0, floor.size().x - PASSAGE_WIDTH);
                    floor.max.x = floor.min.x + PASSAGE_WIDTH;
                } else {
                    // Intersect on y
                    floor.min.y += rng.gen_range(0.0, floor.size().y - PASSAGE_WIDTH);
                    floor.max.y = floor.min.y + PASSAGE_WIDTH;
                }

                let mut passage = Room::new(floor);

                passage.cut_walls(self.rooms[room_index].area());
                passage.cut_walls(self.rooms[i].area());

                self.rooms[room_index].cut_walls(floor);
                self.rooms[i].cut_walls(floor);

                self.passages.push(passage);
            }
        }
    }


    fn find_room_index(&self, position: Vector3) -> Option<usize> {
        let pos = Vector2::new(position.x, position.z);

        for (index, room) in self.rooms.iter().enumerate() {
            if room.floor.contains(pos) {
                return Some(index);
            }
        }

        None
    }
}


impl Draw for World {
    fn draw(&self) -> DrawCommand {
        DrawCommand::List(
            self.rooms.iter().map(|room| { room.draw() }).chain(
                self.passages.iter().map(|passage|{passage.draw()})
            ).collect()
        )
    }
}


struct Room {
    boxes: Vec<BoundingBox>,

    floor: Rectangle,

    explored: bool
}


impl Room {
    pub fn new(floor: Rectangle) -> Room {
        Room {
            boxes: vec![
                // Floor
                BoundingBox {
                    min: Vector3::new(floor.min.x, -1.0, floor.min.y),
                    max: Vector3::new(floor.max.x, 0.0, floor.max.y),
                    color: None,
                },

                // Walls
                BoundingBox {
                    min: Vector3::new(floor.min.x, -1.0, floor.max.y),
                    max: Vector3::new(floor.max.x + WALL_THICKNESS, WALL_HEIGHT, floor.max.y + WALL_THICKNESS),
                    color: None,
                },
                BoundingBox {
                    min: Vector3::new(floor.max.x, -1.0, floor.min.y - WALL_THICKNESS),
                    max: Vector3::new(floor.max.x + WALL_THICKNESS, WALL_HEIGHT, floor.max.y),
                    color: None,
                },
                BoundingBox {
                    min: Vector3::new(floor.min.x - WALL_THICKNESS, -1.0, floor.min.y - WALL_THICKNESS),
                    max: Vector3::new(floor.max.x, WALL_HEIGHT, floor.min.y),
                    color: None,
                },
                BoundingBox {
                    min: Vector3::new(floor.min.x - WALL_THICKNESS, -1.0, floor.min.y),
                    max: Vector3::new(floor.min.x, WALL_HEIGHT, floor.max.y + WALL_THICKNESS),
                    color: None,
                },
            ],

            floor,

            explored: false,
        }
    }


    pub fn cut_walls(&mut self, mask: Rectangle) {
        let mut box_count = self.boxes.len();

        let mut i = 0;

        while {
            i += 1;
            i < box_count
        } {
            let floor: Rectangle = self.boxes[i].project_y();

            let mut pieces = floor.cut_by(mask).into_iter().map(|rect|{
                rect.extrude_y(-1.0, WALL_HEIGHT)
            });

            if let Some(piece) = pieces.next() {
                self.boxes[i] = piece;
                self.boxes.extend(pieces);
            } else {
                self.boxes.remove(i);
                i -= 1;
                box_count -= 1;
            }
        }
    }


    pub fn area(&self) -> Rectangle {
        Rectangle {
            min: Vector2::new(self.floor.min.x - WALL_THICKNESS, self.floor.min.y - WALL_THICKNESS),
            max: Vector2::new(self.floor.max.x + WALL_THICKNESS, self.floor.max.y + WALL_THICKNESS),
        }
    }
}

impl Draw for Room {
    fn draw(&self) -> DrawCommand {
        DrawCommand::List(
            self.boxes.iter().map(|b| { b.draw() }).collect()
        )
    }
}