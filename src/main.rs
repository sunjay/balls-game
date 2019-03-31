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

#[derive(Debug, Clone)]
enum GameState {
    SelectDiration,
    Simulate {initial_angle: f64},
}

static LEVEL: &[&[usize]] = &[
    &[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    &[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    &[00, 32, 64, 00, 00, 00, 00, 00, 00, 64, 32, 00],
    &[64, 32, 32, 32, 21, 32, 32, 32, 75, 32, 32, 64],
    &[64, 32, 32, 32, 21, 32, 32, 75, 88, 32, 32, 64],
    &[64, 32, 32, 32, 32, 42, 82, 32, 88, 32, 32, 64],
    &[64, 32, 32, 88, 32, 82, 42, 32, 32, 32, 32, 64],
    &[64, 32, 32, 88, 75, 32, 32, 21, 32, 32, 32, 64],
    &[00, 32, 32, 75, 32, 32, 32, 21, 32, 32, 32, 00],
    &[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    &[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    &[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    &[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    &[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    &[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    &[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
];

fn value_color(value: usize) -> Color {
    match value {
        0..=21 => Color {r: 38, g: 166, b: 154, a: 255},
        22..=35 => Color {r: 33, g: 150, b: 243, a: 255},
        36..=52 => Color {r: 102, g: 187, b: 106, a: 255},
        53..=65 => Color {r: 255, g: 152, b: 0, a: 255},
        66..=78 => Color {r: 126, g: 87, b: 194, a: 255},
        79..=91 => Color {r: 77, g: 208, b: 225, a: 255},
        92..=102 => Color {r: 92, g: 107, b: 192, a: 255},
        _ => Color {r: 158, g: 158, b: 158, a: 255},
    }
}

fn main() -> Result<(), String> {
    let box_size = 26; // pixels
    let box_padding = 1; // pixels

    let balls = 50;
    let ball_radius = 3;

    // The maximum value of any given block
    let max_value = 500;

    // Measure the level so we can size the window
    let rows = LEVEL.len() as u32;
    let cols = LEVEL[0].len() as u32;

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

    for i in 0..=max_value {
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

    for _ in 0..balls {
        world.create_entity()
            .with(Position(Point::new(0, logical_height as i32 / 2 - ball_radius as i32)))
            .with(Velocity {angle: 1.0*std::f64::consts::PI/3.0, speed: 2})
            .with(Ball {
                radius: ball_radius,
                color: Color {r: 255, g: 255, b: 255, a: 255},
            })
            .build();
    }

    for (i, level_row) in LEVEL.into_iter().enumerate() {
        for (j, &value) in level_row.into_iter().enumerate() {
            if value == 0 {
                continue;
            }

            let x_offset = j as u32 * total_box_size;
            let y_offset = i as u32 * total_box_size;

            let center = Point::new(
                (x_offset + total_box_size / 2) as i32 - logical_width as i32 / 2,
                (y_offset + total_box_size / 2) as i32 - logical_height as i32 / 2,
            );

            world.create_entity()
                .with(Position(center))
                .with(Block {
                    value,
                    color: value_color(value),
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
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
