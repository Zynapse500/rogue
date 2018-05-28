extern crate graphics_3d;

use graphics_3d::{
    glutin::{
        EventsLoop,
        WindowBuilder,
        ContextBuilder,

        Event,
        WindowEvent,

        KeyboardInput,
        VirtualKeyCode
    },

    Display
};


/*extern crate trap;
use trap::Vector3;


extern crate rand;

mod bounding_box;
use bounding_box::BoundingBox;

mod camera;
use camera::Camera;


mod game;
use game::{
    GameState
};

mod stopwatch;
use stopwatch::Stopwatch;


use std::{
    time::{
        Instant
    }
};*/


/*

fn main() {
    let mut events_loop = EventsLoop::new();

    let mut screen = {
        let window = WindowBuilder::new()
            .with_fullscreen(Some(events_loop.get_primary_monitor()))
            .with_title("Window");

        let context = ContextBuilder::new()
            .with_vsync(false)
            .with_multisampling(8);

        Screen::new(window, context, &events_loop)
    };

    screen.set_cursor(CursorState::Grab);


    let mut game = GameState::new();


    let mut running = true;
    let mut stopwatch = Stopwatch::new();

    while game.running() {
        events_loop.poll_events(|e| {
            game.handle_event(e);
        });

        let delta_time = stopwatch.tick();
        game.update(delta_time);

        screen.render(|frame| {
            game.draw(frame);
        });
    }
}

*/


fn main() {
    let mut events_loop = EventsLoop::new();

    let mut display = {
        let window = WindowBuilder::new()
            .with_title("gfx");

        let context = ContextBuilder::new()
            .with_vsync(true);

        Display::new(window, context, &events_loop)
    };


    let mut running = true;

    while running {
        events_loop.poll_events(|event|{
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested |
                        WindowEvent::KeyboardInput { input: KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape), ..
                        }, .. } => {
                            running = false;
                        },

                        _ => ()
                    }
                },

                _ => ()
            }
        });


        display.clear();
    }
}
