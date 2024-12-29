use crate::interpreter::{Interpreter, BLACK_DISPLAY, DISPLAY_HEIGHT, DISPLAY_WIDTH};

const RESET_TERMINAL: &str = "\x1B[2J\x1B[1;1H\x1B[?25l";

fn display_to_string(display: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) -> String {
    let mut display_string = String::new();

    for row in display {
        for &pixel in row {
            display_string.push(if pixel { '█' } else { ' ' });
        }
        display_string.push('\n');
    }

    display_string
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
    let mut interpreter = Interpreter::new();

    let program_path = get_input("Enter path to a Chip8 program: ").unwrap();

    interpreter.load_program_from_path(program_path).unwrap();

    loop {
        print!(
            "{}{}",
            RESET_TERMINAL,
            display_to_string(interpreter.display())
        );

        if !interpreter.execute_current_instruction() {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[test]
fn ibm_logo() {
    let mut interpreter = Interpreter::new();
    interpreter
        .load_program_from_path("assets/roms/ibm_logo.ch8")
        .unwrap();

    loop {
        print!(
            "{}{}",
            RESET_TERMINAL,
            display_to_string(interpreter.display())
        );

        if !interpreter.execute_current_instruction() {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
