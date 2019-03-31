use specs::prelude::*;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;

const TEXT_HORI_PADDING: u32 = 1; // pixels
const TEXT_VERT_PADDING: u32 = 4; // pixels

// Type alias for the data needed by the renderer
pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Block>,
    ReadStorage<'a, Ball>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    number_textures: &[Texture],
    (positions, sprites, balls): SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(Color {r: 97, g: 97, b: 97, a: 255});
    canvas.clear();

    let (width, height) = canvas.logical_size();

    for (&Position(pos), block) in (&positions, &sprites).join() {
        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = pos + Point::new(width as i32 / 2, height as i32 / 2);

        let rect_area = Rect::from_center(screen_position, block.width, block.height);
        canvas.set_draw_color(block.color);
        canvas.fill_rect(rect_area)?;

        let number = &number_textures[block.value];
        let texture_info = number.query();
        // Want to preserve aspect ratio of text while keeping it in bounds
        // https://stackoverflow.com/a/1106367/551904
        let max_num_width = block.width - TEXT_HORI_PADDING * 2;
        let max_num_height = block.height - TEXT_VERT_PADDING * 2;

        // We aren't using floating point, so we multiply by this extra number p to avoid
        // truncating down to zero or one. The larger the p, the more accurate this calculation.
        // Too large risks wrapping during the multiplciation.
        // Taking advantage of the fact that x * p / p ~= x
        let p = 1000;

        // Find out what scale factor we would have to use to get up to the max width and max
        // height and then scale by the lower of those factors to get within the box.
        let width_scale = max_num_width * p / texture_info.width;
        let height_scale = max_num_height * p / texture_info.height;
        let scale = width_scale.min(height_scale);

        let text_width = texture_info.width * scale / p;
        let text_height = texture_info.height * scale / p;

        canvas.copy(&number, None, Rect::from_center(screen_position, text_width, text_height))?;
    }

    for (&Position(pos), &Ball {radius, color}) in (&positions, &balls).join() {
        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = pos + Point::new(width as i32 / 2, height as i32 / 2);

        let diameter = radius * 2;
        let rect_area = Rect::from_center(screen_position, diameter, diameter);
        canvas.set_draw_color(color);
        canvas.fill_rect(rect_area)?;
    }

    canvas.present();

    Ok(())
}
