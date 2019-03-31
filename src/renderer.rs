use specs::prelude::*;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;

const TEXT_HORI_PADDING: u32 = 2; // pixels
const TEXT_VERT_PADDING: u32 = 4; // pixels

// Type alias for the data needed by the renderer
pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Block>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    number_textures: &[Texture],
    (positions, sprites): SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(Color { r: 117, g: 117, b: 117 , a: 1 });
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
        // Preserve aspect ratio of text while keeping it in bounds
        let (text_width, text_height) = if texture_info.width >= texture_info.height {
            let text_height = block.height - TEXT_VERT_PADDING * 2;
            let text_width = text_height * texture_info.width / texture_info.height;
            (text_width, text_height)
        } else {
            let text_width = block.width - TEXT_HORI_PADDING * 2;
            let text_height = text_width * texture_info.height / texture_info.width;
            (text_width, text_height)
        };

        canvas.copy(&number, None, Rect::from_center(screen_position, text_width, text_height))?;
    }

    canvas.present();

    Ok(())
}
