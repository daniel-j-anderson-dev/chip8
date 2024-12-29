use crate::interpreter::{Interpreter, BLACK_DISPLAY, DISPLAY_HEIGHT, DISPLAY_WIDTH};

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

    interpreter.execute_program_stdout();
}

#[test]
fn ibm_logo() {
    let mut interpreter = Interpreter::new();

    interpreter
        .load_program_from_path("assets/roms/ibm_logo.ch8")
        .unwrap();

    interpreter.execute_program_stdout();
}
