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

    // The bitwise and assignment operator
    And(Position),

    // The bitwise or assignment operator
    Or(Position),

    // The bitwise xor assignment operator
    Xor(Position),

    // The left shift operator
    LShift(Position),

    // The right shift operator
    RShift(Position),

    // The equality comparison operator
    Eq(Position),

    // The not equal comparison operator
    Neq(Position),

    // The less than comparison operator
    Lt(Position),

    // The greater than comparison operator
    Gt(Position),

    // The less than or equal comparison operator
    Lte(Position),

    // The greater than or equal comparison operator
    Gte(Position),

    // A unary conditional operator for whether the key in a specified register is pressed
    Key(Position),

    // A unary conditional operator for whether the key in a specified register is not pressed
    NKey(Position),
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

            Self::And(position) => {
                write!(f, "Token::And({position})")
            }

            Self::Or(position) => {
                write!(f, "Token::Or({position})")
            }

            Self::Xor(position) => {
                write!(f, "Token::Xor({position})")
            }

            Self::LShift(position) => {
                write!(f, "Token::LShift({position})")
            }

            Self::RShift(position) => {
                write!(f, "Token::RShift({position})")
            }

            Self::Eq(position) => {
                write!(f, "Token::Eq({position})")
            }

            Self::Neq(position) => {
                write!(f, "Token::Neq({position})")
            }

            Self::Lt(position) => {
                write!(f, "Token::Lt({position})")
            }

            Self::Gt(position) => {
                write!(f, "Token::Gt({position})")
            }

            Self::Lte(position) => {
                write!(f, "Token::Lte({position})")
            }

            Self::Gte(position) => {
                write!(f, "Token::Gte({position})")
            }

            Self::Key(position) => {
                write!(f, "Token::Key({position})")
            }

            Self::NKey(position) => {
                write!(f, "Token::NKey({position})")
            }
        }
    }
}
