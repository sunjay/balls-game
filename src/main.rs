#![deny(unused_must_use)]

mod components;
mod physics;
mod renderer;

use std::env;

use sdl2::ttf;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use specs::prelude::*;

use std::time::Duration;

use crate::components::*;

fn main() -> Result<(), String> {
    let rows = 16;
    let cols = 12;

    let box_rows = 5;
    let box_cols = 12;
    let empty_top_box_rows = 2;
    let box_size = 26; // pixels
    let box_padding = 2; // pixels

    // The maximum value of any given block
    let max_value = 300;

    // For high DPI displays
    let window_scale = env::var("DISPLAY_SCALE")
        .map(|scale| scale.parse().expect("unable to parse DISPLAY_SCALE"))
        .unwrap_or(2);

    let total_box_size = box_size + box_padding * 2;
    let logical_width = cols * total_box_size;
    let logical_height = rows * total_box_size;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem.window(
        "Balls Game",
        window_scale * logical_width,
        window_scale * logical_height,
    ).position_centered().build().expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    canvas.set_logical_size(logical_width, logical_height).expect("unable to set logical size");

    let texture_creator = canvas.texture_creator();
    let font = ttf_context.load_font("fonts/Roboto_Mono/RobotoMono-Regular.ttf", 40)?;

    let mut number_textures = Vec::new();
    number_textures.reserve_exact(max_value);

    for i in 0..max_value {
        let surface = font.render(&format!("{}", i))
            .blended(Color::RGBA(255, 255, 255, 255)).map_err(|e| e.to_string())?;
        let texture = texture_creator.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        number_textures.push(texture);
    }

    let mut dispatcher = DispatcherBuilder::new()
        .with(physics::Physics, "Physics", &[])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world.res);
    renderer::SystemData::setup(&mut world.res);

    for i in 0..box_rows {
        for j in 0..box_cols {
            let x_offset = j * total_box_size;
            let y_offset = (empty_top_box_rows + i) * total_box_size;

            let center = Point::new(
                (x_offset + total_box_size / 2) as i32 - logical_width as i32 / 2,
                (y_offset + total_box_size / 2) as i32 - logical_height as i32 / 2,
            );

            world.create_entity()
                .with(Position(center))
                .with(Block {
                    value: 222,
                    color: Color {
                        r: 255,
                        g: 32,
                        b: 32,
                        a: 255,
                    },
                    width: box_size,
                    height: box_size,
                })
                .build();
        }
    }

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Update
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        // Render
        renderer::render(&mut canvas, &number_textures, world.system_data())?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
