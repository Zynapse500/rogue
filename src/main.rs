extern crate graphics_3d;

use graphics_3d::{
    glutin::{
        EventsLoop,
        WindowBuilder,
        ContextBuilder,

        CursorState
    },


    Display,
};

extern crate rand;

mod bounding_box;

mod camera;


mod game;
use game::{
    GameState
};

mod stopwatch;
use stopwatch::Stopwatch;


mod frame_counter;
use frame_counter::FrameCounter;


fn main() {
    let mut events_loop = EventsLoop::new();

    let mut display = {
        let window = WindowBuilder::new()
            .with_fullscreen(Some(events_loop.get_primary_monitor()))
            .with_title("Window");

        let context = ContextBuilder::new()
            .with_vsync(false)
            .with_multisampling(8);

        Display::new(window, context, &events_loop)
    };

    display.set_cursor_state(CursorState::Grab);


    let mut game = GameState::new();


    let mut stopwatch = Stopwatch::new();

    let mut frame_counter = FrameCounter::new();

    while game.running() {
        if let Some(fps) = frame_counter.tick() {
            println!("FPS: {}", fps.ceil());
        }

        events_loop.poll_events(|e| {
            game.handle_event(e);
        });

        let delta_time = stopwatch.tick();
        game.update(delta_time);

        let mut frame = display.render();

        game.draw(&mut frame);

        display.submit(frame);
    }
}

