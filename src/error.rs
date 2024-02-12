#[derive(Debug)]
pub enum Chip8Error {
    ProgramTooLarge,
    Io(std::io::Error),
}
impl std::fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // unimplemented!();
        write!(
            f,
            "Chip8 Error: {}",
            match self {
                Chip8Error::ProgramTooLarge => String::from(""),
                Chip8Error::Io(e) => format!("{}", e),
            }
        )
    }
}
impl std::error::Error for Chip8Error {}

impl From<std::io::Error> for Chip8Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
