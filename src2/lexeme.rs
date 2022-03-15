use crate::{lexeme_kind::LexemeKind, position::Position};

#[derive(Clone, Debug)]
pub struct Lexeme {
    pub kind:     LexemeKind,
    pub raw:      String,
    pub position: Position,
}

impl std::fmt::Display for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} {:?}", self.position, self.kind, self.raw)
    }
}
