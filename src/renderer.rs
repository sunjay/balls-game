use specs::prelude::*;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::components::*;

// Type alias for the data needed by the renderer
pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, ColoredRect>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    (positions, sprites): SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(Color { r: 117, g: 117, b: 117 , a: 1 });
    canvas.clear();

    let (width, height) = canvas.logical_size();

    for (&Position(pos), colored_rect) in (&positions, &sprites).join() {
        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = pos + Point::new(width as i32 / 2, height as i32 / 2);

        let rect_area = Rect::from_center(screen_position, colored_rect.width, colored_rect.height);
        canvas.set_draw_color(colored_rect.color);
        canvas.fill_rect(rect_area)?;
    }

    canvas.present();

    Ok(())
}
