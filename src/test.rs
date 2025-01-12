use crate::interpreter::{Builder, Interpreter};

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

    interpreter.execute_program_terminal();
}

macros::generate_terminal_tests!();
