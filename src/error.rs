use ::pos::Pos;

#[derive(Debug)]
pub enum Error {
    GenericError,
    IllegalCharacter(char, Pos),
}
