#[derive(Debug)]
pub enum Chip8Error {
    ProgramTooLarge,
}
impl std::fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chip8 Error: {}", match self {
            Chip8Error::ProgramTooLarge => String::from(""),
        })
    }
}
impl std::error::Error for Chip8Error {}
