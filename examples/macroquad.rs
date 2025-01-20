use chip8::interpreter::{Interpreter, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use macroquad::{
    miniquad::window::screen_size,
    prelude::*,
    ui::{hash, root_ui, widgets},
};

fn window_configuration() -> macroquad::window::Conf {
    Conf {
        window_title: String::from("Chip8"),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        fullscreen: false,
        sample_count: 100,
        ..Default::default()
    }
}

enum State {
    ProgramSelect,
    LoadingProgram,
    ExecutingProgram,
}

#[macroquad::main(window_configuration)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = State::ProgramSelect;
    let mut program_path = String::new();
    let mut load_program_error = None::<String>;

    let mut chip8 = Interpreter::new();

    let mut display_image = Image::gen_image_color(DISPLAY_WIDTH as _, DISPLAY_HEIGHT as _, WHITE);
    let display_texture = Texture2D::from_image(&display_image);

    loop {
        clear_background(LIGHTGRAY);

        let screen_dimensions = Vec2::from(screen_size());
        let screen_origin = screen_dimensions / 2.0;

        match state {
            State::ProgramSelect => {
                let label_text = "Enter the path to a chip8 program";
                let label_text_dimensions = measure_text(label_text, None, 16, 1.0);
                let label_position = vec2(
                    screen_origin.x
                        - (label_text_dimensions.width / 2.0)
                        - (screen_dimensions.x / 40.0),
                    screen_origin.y / 8.0,
                );
                let input_size = vec2(
                    screen_dimensions.x / 4.0,
                    label_text_dimensions.height * 2.0,
                );
                let input_position = vec2(label_position.x, label_position.y * 1.5);
                let button_text = "Load Program";
                let button_text_dimensions = measure_text(button_text, None, 16, 1.0);
                let button_size = vec2(
                    button_text_dimensions.width * 1.5,
                    button_text_dimensions.height * 1.5,
                );
                let button_position = vec2(
                    input_position.x + input_size.x + (screen_dimensions.x / 50.0),
                    input_position.y + ((input_size.y - button_size.y) / 2.0),
                );
                let error_text_position = vec2(input_position.x, input_position.y * 1.5);
                root_ui().window(hash!(), vec2(0.0, 0.0), screen_dimensions, |ui| {
                    ui.label(label_position, label_text);

                    widgets::InputText::new(hash!())
                        .position(input_position)
                        .size(input_size)
                        .ui(ui, &mut program_path);

                    if widgets::Button::new(button_text)
                        .position(button_position)
                        .size(button_size)
                        .ui(ui)
                    {
                        load_program_error = None;
                        state = State::LoadingProgram;
                    }

                    if let Some(ref load_program_error) = load_program_error {
                        ui.label(error_text_position, load_program_error);
                    }
                });
            }
            State::LoadingProgram => match chip8.load_program_from_path(&program_path) {
                Ok(()) => state = State::ExecutingProgram,
                Err(io_error) => {
                    load_program_error = Some(format!("Failed to load program: {io_error}"));
                    state = State::ProgramSelect;
                }
            },
            State::ExecutingProgram => {
                handle_input(chip8.keypad_mut());

                update_display(chip8.display(), &mut display_image, &display_texture);
                let display_size = vec2(
                    DISPLAY_WIDTH as f32 * screen_dimensions.x / 100.0,
                    DISPLAY_HEIGHT as f32 * screen_dimensions.y / 100.0,
                );
                draw_texture_ex(
                    &display_texture,
                    (screen_origin.x) - display_size.x / 2.0,
                    (screen_origin.y) - display_size.y / 2.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(display_size),
                        ..Default::default()
                    },
                );

                if !chip8.execute_current_instruction() {
                    break;
                }
            }
        };

        next_frame().await;
    }

    Ok(())
}

fn update_display(
    display: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    display_image: &mut Image,
    display_texture: &Texture2D,
) {
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
