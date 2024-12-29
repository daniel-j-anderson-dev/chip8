# A [Chip8](https://en.wikipedia.org/wiki/CHIP-8) interpreter library

## Usage

The main api is `chip8::Interpreter`.

- load a rom: `interpreter.load_program_from_path("my_chip8.ch8")?`
- execute an instruction: `interpreter.execute_current_instruction()`
- get the display's state: `interpreter.display()`

## Resources used

- [Awesome Chip8](https://chip-8.github.io/links/)
- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator)
- [Building a CHIP-8 Emulator [C++]](https://austinmorlan.com/posts/chip8_emulator/)
- [Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)

## Example

- Run the IBM logo ROM in a terminal with `cargo test ibm_logo -- --nocapture`
  ![ibm_logo_example](ibm_logo.gif)
