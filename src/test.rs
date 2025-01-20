use crate::interpreter::{ConfigurationBuilder, Interpreter};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::{io::Write, time::Duration};

fn execute_program_terminal(mut chip8: Interpreter) -> Result<(), std::io::Error> {
    // prepare the terminal
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    stdout
        .execute(Hide)?
        .execute(Clear(ClearType::All))?
        .execute(MoveTo(0, 0))?;

    loop {
        // handle input
        let keypad = chip8.keypad_mut();
        *keypad = [false; 16];
        if event::poll(Duration::from_nanos(1))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        break
                    }
                    KeyCode::Char('1') => keypad[0x0] = true,
                    KeyCode::Char('2') => keypad[0x1] = true,
                    KeyCode::Char('3') => keypad[0x2] = true,
                    KeyCode::Char('4') => keypad[0x3] = true,
                    KeyCode::Char('q') => keypad[0x4] = true,
                    KeyCode::Char('w') => keypad[0x5] = true,
                    KeyCode::Char('e') => keypad[0x6] = true,
                    KeyCode::Char('r') => keypad[0x7] = true,
                    KeyCode::Char('a') => keypad[0x8] = true,
                    KeyCode::Char('s') => keypad[0x9] = true,
                    KeyCode::Char('d') => keypad[0xA] = true,
                    KeyCode::Char('f') => keypad[0xB] = true,
                    KeyCode::Char('z') => keypad[0xC] = true,
                    KeyCode::Char('x') => keypad[0xD] = true,
                    KeyCode::Char('c') => keypad[0xE] = true,
                    KeyCode::Char('v') => keypad[0xF] = true,
                    _ => {}
                }
            }
        }

        // print display
        for (y, row) in chip8.display().iter().enumerate() {
            stdout.execute(MoveTo(0, y as u16))?;
            for pixel in row.iter().map(|&pixel| if pixel { "█" } else { " " }) {
                stdout.write_all(pixel.as_bytes())?;
            }
        }
        stdout.execute(MoveTo(0, chip8.configuration().display_height() as u16));

        // execute instruction
        if !chip8.execute_current_instruction() {
            break;
        }
    }

    // reset the terminal
    stdout.execute(Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    {
        use std::io::Write;
        let mut stdout = std::io::stdout();
        stdout.write_all(prompt.as_bytes())?;
        stdout.flush()?;
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input.truncate(input.trim_end().len());

    Ok(input)
}

#[test]
fn user_program() {
    let mut interpreter = Interpreter::default();

    let program_path = get_input("Enter path to a Chip8 program: ").unwrap();
    interpreter.load_program_from_path(program_path).unwrap();

    execute_program_terminal(interpreter).unwrap();
}

macros::generate_terminal_tests!();
