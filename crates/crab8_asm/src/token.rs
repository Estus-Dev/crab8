use crab8::registers::Register;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // Used for tokens we don't know how to parse yet.
    Unknown(String),

    // A register identifier.
    Register(Register),

    // The assignment operator.
    Assign,

    // The addition assignment operator
    Add,

    // The subtraction assignment operator
    Sub,

    // The reverse subtraction assignment operator
    SubFrom,

    // The bitwise and assignment operator
    And,

    // The bitwise or assignment operator
    Or,

    // The bitwise xor assignment operator
    Xor,

    // The left shift operator
    LShift,

    // The right shift operator
    RShift,

    // The equality comparison operator
    Eq,

    // The not equal comparison operator
    Neq,

    // The less than comparison operator
    Lt,

    // The greater than comparison operator
    Gt,

    // The less than or equal comparison operator
    Lte,

    // The greater than or equal comparison operator
    Gte,

    // A unary conditional operator for whether the key in a specified register is pressed
    Key,

    // A unary conditional operator for whether the key in a specified register is not pressed
    NKey,
}
