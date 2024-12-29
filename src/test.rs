use crate::interpreter::Interpreter;

const RESET_TERMINAL: &str = "\x1B[2J\x1B[1;1H\x1B[?25l";

fn display_to_string(
    display: &[[bool; Interpreter::DISPLAY_WIDTH]; Interpreter::DISPLAY_HEIGHT],
) -> String {
    let mut display_string = String::new();

    for row in display {
        for &pixel in row {
            display_string.push(if pixel { '█' } else { ' ' });
        }
        display_string.push('\n');
    }

    display_string
}

#[test]
pub fn ibm_logo() {
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
