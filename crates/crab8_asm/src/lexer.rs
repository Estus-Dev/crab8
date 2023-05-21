use crab8::registers::Register;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq)]
#[logos(skip r"[ \t\f]")]
pub enum Token {
    // A register identifier.
    #[regex("v([0-9a-f])", |r| r.slice().parse().ok())]
    Register(Register),

    // The delay register
    #[token("delay")]
    Delay,

    // The sound register
    #[token("buzzer")]
    Buzzer,

    // The address register
    #[token("i")]
    I,

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

    // A keyword to return from a function
    #[token("return")]
    Return,

    // A keyword for clearing the screen
    #[token("clear")]
    Clear,

    // A keyword to store a value in memory in Binary Coded Decimal
    #[token("bcd")]
    Bcd,

    // A keyword to save a value in memory
    #[token("save")]
    Save,

    // A keyword to load a value from memory
    #[token("load")]
    Load,

    // A keyword to jump the program counter
    #[token("jump")]
    Jump,

    // A keyword to jump the program counter with an offset in V0
    #[token("jump0")]
    Jump0,

    // A keyword to specify a number in hex
    #[token("hex")]
    Hex,

    // Track newlines because most statements end with one
    #[token("\n")]
    Newline,

    // Used for tokens we don't know how to parse yet.
    #[regex(r"\S*")]
    Unknown,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lex_registers() {
        let input = "v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 va vb vc vd ve vf";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V0))));
        assert_eq!(lexer.slice(), "v0");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V1))));
        assert_eq!(lexer.slice(), "v1");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V2))));
        assert_eq!(lexer.slice(), "v2");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V3))));
        assert_eq!(lexer.slice(), "v3");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V4))));
        assert_eq!(lexer.slice(), "v4");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V5))));
        assert_eq!(lexer.slice(), "v5");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V6))));
        assert_eq!(lexer.slice(), "v6");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V7))));
        assert_eq!(lexer.slice(), "v7");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V8))));
        assert_eq!(lexer.slice(), "v8");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V9))));
        assert_eq!(lexer.slice(), "v9");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::VA))));
        assert_eq!(lexer.slice(), "va");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::VB))));
        assert_eq!(lexer.slice(), "vb");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::VC))));
        assert_eq!(lexer.slice(), "vc");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::VD))));
        assert_eq!(lexer.slice(), "vd");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::VE))));
        assert_eq!(lexer.slice(), "ve");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::VF))));
        assert_eq!(lexer.slice(), "vf");
    }

    #[test]
    fn test_lex_special_registers() {
        let input = "delay buzzer i";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Delay)));
        assert_eq!(lexer.slice(), "delay");

        assert_eq!(lexer.next(), Some(Ok(Token::Buzzer)));
        assert_eq!(lexer.slice(), "buzzer");

        assert_eq!(lexer.next(), Some(Ok(Token::I)));
        assert_eq!(lexer.slice(), "i");
    }

    #[test]
    fn test_lex_assign() {
        let input = ":=";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Assign)));
        assert_eq!(lexer.slice(), ":=");
    }

    #[test]
    fn test_lex_basic_math_ops() {
        let input = "+= -= =-";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Add)));
        assert_eq!(lexer.slice(), "+=");

        assert_eq!(lexer.next(), Some(Ok(Token::Sub)));
        assert_eq!(lexer.slice(), "-=");

        assert_eq!(lexer.next(), Some(Ok(Token::SubFrom)));
        assert_eq!(lexer.slice(), "=-");
    }

    #[test]
    fn test_lex_bitwise_ops() {
        let input = "&= |= ^=";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::And)));
        assert_eq!(lexer.slice(), "&=");

        assert_eq!(lexer.next(), Some(Ok(Token::Or)));
        assert_eq!(lexer.slice(), "|=");

        assert_eq!(lexer.next(), Some(Ok(Token::Xor)));
        assert_eq!(lexer.slice(), "^=");
    }

    #[test]
    fn test_lex_shift_ops() {
        let input = "<<= >>=";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::LShift)));
        assert_eq!(lexer.slice(), "<<=");

        assert_eq!(lexer.next(), Some(Ok(Token::RShift)));
        assert_eq!(lexer.slice(), ">>=");
    }

    #[test]
    fn test_lex_comparison_ops() {
        let input = "== != <  >  <= >=";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Eq)));
        assert_eq!(lexer.slice(), "==");

        assert_eq!(lexer.next(), Some(Ok(Token::Neq)));
        assert_eq!(lexer.slice(), "!=");

        assert_eq!(lexer.next(), Some(Ok(Token::Lt)));
        assert_eq!(lexer.slice(), "<");

        assert_eq!(lexer.next(), Some(Ok(Token::Gt)));
        assert_eq!(lexer.slice(), ">");

        assert_eq!(lexer.next(), Some(Ok(Token::Lte)));
        assert_eq!(lexer.slice(), "<=");

        assert_eq!(lexer.next(), Some(Ok(Token::Gte)));
        assert_eq!(lexer.slice(), ">=");
    }

    #[test]
    fn test_lex_input_ops() {
        let input = "key -key";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Key)));
        assert_eq!(lexer.slice(), "key");

        assert_eq!(lexer.next(), Some(Ok(Token::NKey)));
        assert_eq!(lexer.slice(), "-key");
    }

    #[test]
    fn test_lex_newline() {
        let input = "v0 v1\nv2";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V0))));
        assert_eq!(lexer.slice(), "v0");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V1))));
        assert_eq!(lexer.slice(), "v1");

        assert_eq!(lexer.next(), Some(Ok(Token::Newline)));
        assert_eq!(lexer.slice(), "\n");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V2))));
        assert_eq!(lexer.slice(), "v2");
    }

    #[test]
    fn text_lex_keywords() {
        let input = "return clear bcd save load jump jump0 hex";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Return)));
        assert_eq!(lexer.slice(), "return");

        assert_eq!(lexer.next(), Some(Ok(Token::Clear)));
        assert_eq!(lexer.slice(), "clear");

        assert_eq!(lexer.next(), Some(Ok(Token::Bcd)));
        assert_eq!(lexer.slice(), "bcd");

        assert_eq!(lexer.next(), Some(Ok(Token::Save)));
        assert_eq!(lexer.slice(), "save");

        assert_eq!(lexer.next(), Some(Ok(Token::Load)));
        assert_eq!(lexer.slice(), "load");

        assert_eq!(lexer.next(), Some(Ok(Token::Jump)));
        assert_eq!(lexer.slice(), "jump");

        assert_eq!(lexer.next(), Some(Ok(Token::Jump0)));
        assert_eq!(lexer.slice(), "jump0");

        assert_eq!(lexer.next(), Some(Ok(Token::Hex)));
        assert_eq!(lexer.slice(), "hex");
    }
}
