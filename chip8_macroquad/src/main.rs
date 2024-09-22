use chip8::Chip8;
use macroquad::prelude::*;
use miniquad::window::screen_size;

#[macroquad::main("chip8")]
async fn main() {
    let mut chip8 = Chip8::initialize();
    chip8.load_program("./test_opcode.ch8").unwrap();

    let mut chip8_display = Image::gen_image_color(Chip8::SCREEN_WIDTH as _, Chip8::SCREEN_HEIGHT as _, BLACK);
    let chip8_display_texture = Texture2D::from_image(&chip8_display);
    let chip8_display_size = vec2(Chip8::SCREEN_WIDTH as _, Chip8::SCREEN_HEIGHT as _) * 4.0;

    loop {
        clear_background(LIGHTGRAY);

        for y in 0..chip8_display.height as u32 {
            for x in 0..chip8_display.width as u32 {
                let color = if chip8.is_pixel_white(y as usize, x as usize) {
                    WHITE
                } else {
                    BLACK
                };
                chip8_display.set_pixel(x, y, color);
            }
        }

        let screen_origin = Vec2::from(screen_size()) / 2.0;
        let chip8_display_position = screen_origin - (chip8_display_size / 2.0);
        draw_texture_ex(
            &chip8_display_texture,
            chip8_display_position.x,
            chip8_display_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(chip8_display_size),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None
            },
        );
            
        chip8.step_execution();

        next_frame().await;
    }
}
