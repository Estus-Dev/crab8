use crab8::registers::Register;

#[derive(PartialEq, Eq)]
pub struct Position {
    // The line number where this token is found.
    line: usize,

    // The column number where this token is found.
    column: usize,

    // The length of the token, in chars.
    length: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, length: usize) -> Self {
        Self {
            line,
            column,
            length,
        }
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos({self})")
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.line;
        let col = self.column;
        let end = col + self.length;
        write!(f, "Pos({line}:{col}-{end})")
    }
}

#[derive(PartialEq, Eq)]
pub enum Token {
    // Used for tokens we don't know how to parse yet.
    Unknown(Position, String),

    // A register identifier.
    Register(Position, Register),

    // The assignment operator.
    Assign(Position),

    // The addition assignment operator
    Add(Position),

    // The subtraction assignment operator
    Sub(Position),

    // The reverse subtraction assignment operator
    SubFrom(Position),
}

impl Token {
    pub fn new_register(position: Position, register: Register) -> Self {
        Self::Register(position, register)
    }

    pub fn new_assign(position: Position) -> Self {
        Self::Assign(position)
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(position, token) => {
                write!(f, "Token::Unknown({position}: {token})")
            }

            Self::Register(position, register) => {
                write!(f, "Token::Register({position}: {register:?})")
            }

            Self::Assign(position) => {
                write!(f, "Token::Assign({position})")
            }

            Self::Add(position) => {
                write!(f, "Token::Add({position})")
            }

            Self::Sub(position) => {
                write!(f, "Token::Sub({position})")
            }

            Self::SubFrom(position) => {
                write!(f, "Token::SubFrom({position})")
            }
        }
    }
}
