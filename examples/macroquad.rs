use chip8::interpreter::{ConfigurationBuilder, Interpreter};
use macroquad::{
    miniquad::window::screen_size,
    prelude::*,
    ui::{hash, root_ui, widgets},
};
use std::ops::DerefMut;

const DEFAULT_CHIP8: ConfigurationBuilder =
    Interpreter::builder().instruction_delay(std::time::Duration::ZERO);
const HIGH_RESOLUTION_CHIP8: ConfigurationBuilder =
    DEFAULT_CHIP8.display_width(128).display_height(64);

#[macroquad::main("chip8")]
async fn main() {
    let mut chip8 = DEFAULT_CHIP8.build();

    let mut load_program_error = None::<String>;

    let mut display_image = Image::gen_image_color(
        chip8.configuration().display_width() as _,
        chip8.configuration().display_height() as _,
        WHITE,
    );
    let mut display_texture = Texture2D::from_image(&display_image);
    display_texture.set_filter(FilterMode::Nearest);

    let programs = get_programs();
    let longest_program_name_width = programs
        .iter()
        .max_by_key(|(_, name, _)| name.len())
        .map(|(_, _, name_dimensions)| name_dimensions.width)
        .unwrap_or_default();
    let mut display_scale = 100.0;

    loop {
        clear_background(WHITE);

        let screen_dimensions = Vec2::from(screen_size());

        let choose_window_size = vec2(longest_program_name_width, screen_dimensions.y);
        let choose_window_position = vec2(screen_dimensions.x - choose_window_size.x, 0.0);

        let display_window_size = vec2(
            screen_dimensions.x - choose_window_size.x,
            screen_dimensions.y,
        );
        let display_texture_size = vec2(
            chip8.configuration().display_width() as f32 * screen_dimensions.x,
            chip8.configuration().display_height() as f32 * screen_dimensions.y,
        ) / display_scale;
        let display_texture_position = (display_window_size - display_texture_size) / 2.0;

        widgets::Window::new(hash!(), Vec2::ZERO, display_window_size)
            .titlebar(false)
            .movable(false)
            .ui(root_ui().deref_mut(), |ui| {
                widgets::Texture::new(display_texture.clone())
                    .size(display_texture_size.x, display_texture_size.y)
                    .position(display_texture_position)
                    .ui(ui);
            });

        widgets::Window::new(hash!(), choose_window_position, choose_window_size)
            .label("Choose program")
            .titlebar(true)
            .movable(false)
            .ui(root_ui().deref_mut(), |ui| {
                for (i, (path, name, name_dimensions)) in programs.iter().enumerate() {
                    if widgets::Button::new(name.as_str())
                        .position(vec2(
                            (choose_window_size.x - name_dimensions.width) / 2.0,
                            i as f32 * 24.0,
                            // i as f32 * name_dimensions.height * 2.0,
                        ))
                        .ui(ui)
                    {
                        chip8 = if name.contains("hires") {
                            display_scale = 200.0;
                            HIGH_RESOLUTION_CHIP8
                        } else {
                            display_scale = 100.0;
                            DEFAULT_CHIP8
                        }
                        .build();

                        display_image = Image::gen_image_color(
                            chip8.configuration().display_width() as _,
                            chip8.configuration().display_height() as _,
                            WHITE,
                        );
                        display_texture = Texture2D::from_image(&display_image);
                        display_texture.set_filter(FilterMode::Nearest);

                        if let Err(e) = chip8.load_program_from_path(path) {
                            load_program_error = Some(e.to_string());
                        }
                    };
                }
            });

        handle_input(chip8.keypad_mut());
        update_display(chip8.display(), &mut display_image, &display_texture);
        chip8.execute_current_instruction();

        next_frame().await;
    }
}

fn get_programs() -> Vec<(std::path::PathBuf, String, TextDimensions)> {
    let mut programs = std::fs::read_dir("roms")
        .map(|dir| {
            dir.filter_map(Result::ok)
                .map(|entry| {
                    let path = entry.path();
                    let name = path.to_string_lossy();
                    let name = name[5..name.len() - 4].to_owned();
                    let name_dimensions = measure_text(&name, None, 16, 1.0);
                    (path, name, name_dimensions)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    programs.sort_by_key(|(_, name, _)| name.chars().next().unwrap());
    programs
}

fn update_display(display: &[Box<[bool]>], display_image: &mut Image, display_texture: &Texture2D) {
    for (y, row) in display.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            display_image.set_pixel(x as _, y as _, if pixel { BLACK } else { WHITE });
        }
    }
    display_texture.update(display_image);
}

fn handle_input(keypad: &mut [bool; 16]) {
    keypad[0x0] = is_key_down(KeyCode::Key1);
    keypad[0x1] = is_key_down(KeyCode::Key2);
    keypad[0x2] = is_key_down(KeyCode::Key3);
    keypad[0x3] = is_key_down(KeyCode::Key4);
    keypad[0x4] = is_key_down(KeyCode::Q);
    keypad[0x5] = is_key_down(KeyCode::W);
    keypad[0x6] = is_key_down(KeyCode::E);
    keypad[0x7] = is_key_down(KeyCode::R);
    keypad[0x8] = is_key_down(KeyCode::A);
    keypad[0x9] = is_key_down(KeyCode::S);
    keypad[0xA] = is_key_down(KeyCode::D);
    keypad[0xB] = is_key_down(KeyCode::F);
    keypad[0xC] = is_key_down(KeyCode::Z);
    keypad[0xD] = is_key_down(KeyCode::X);
    keypad[0xE] = is_key_down(KeyCode::C);
    keypad[0xF] = is_key_down(KeyCode::V);
}
