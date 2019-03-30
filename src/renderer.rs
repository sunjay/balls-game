use specs::prelude::*;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;

const TEXT_PADDING: u32 = 2; // pixels

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
        // keep the aspect ratio of the text (assuming height >= width)
        let number_height = block.height - TEXT_PADDING;
        let number_width = number_height * texture_info.width / texture_info.height;
        canvas.copy(&number, None, Rect::from_center(
            screen_position,
            number_width,
            number_height,
        ))?;
    }

    canvas.present();

    Ok(())
}
