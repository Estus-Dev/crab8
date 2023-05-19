use crab8::registers::Register;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq)]
#[logos(skip r"[ \t\n\f]")]
pub enum Token {
    // A register identifier.
    #[regex("v([0-9a-f])", |r| r.slice().parse().ok())]
    Register(Register),

    // The assignment operator.
    #[token(":=")]
    Assign,

    // The addition assignment operator
    #[token("+=")]
    Add,

    // The subtraction assignment operator
    #[token("-=")]
    Sub,

    // The reverse subtraction assignment operator
    #[token("=-")]
    SubFrom,

    // The bitwise and assignment operator
    #[token("&=")]
    And,

    // The bitwise or assignment operator
    #[token("|=")]
    Or,

    // The bitwise xor assignment operator
    #[token("^=")]
    Xor,

    // The left shift operator
    #[token("<<=")]
    LShift,

    // The right shift operator
    #[token(">>=")]
    RShift,

    // The equality comparison operator
    #[token("==")]
    Eq,

    // The not equal comparison operator
    #[token("!=")]
    Neq,

    // The less than comparison operator
    #[token("<")]
    Lt,

    // The greater than comparison operator
    #[token(">")]
    Gt,

    // The less than or equal comparison operator
    #[token("<=")]
    Lte,

    // The greater than or equal comparison operator
    #[token(">=")]
    Gte,

    // A unary conditional operator for whether the key in a specified register is pressed
    #[token("key")]
    Key,

    // A unary conditional operator for whether the key in a specified register is not pressed
    #[token("-key")]
    NKey,

    // Used for tokens we don't know how to parse yet.
    #[regex(r"\S*")]
    Unknown,
}
