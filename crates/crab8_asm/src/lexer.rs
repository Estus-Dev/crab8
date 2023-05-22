use std::num::ParseIntError;

use crab8::registers::Register;
use logos::{Lexer, Logos};

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
    #[token(";")]
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

    // A keyword to specify a 16 bit address
    #[token("long")]
    Long,

    // A keyword to generate a random number
    #[token("random")]
    Random,

    // A keyword to test a condition
    #[token("if")]
    If,

    // A keyword for the start of an if-else block
    #[token("begin")]
    Begin,

    // A keyword for the true branch of a conditional statement
    #[token("then")]
    Then,

    // A keyword for the else branch of a conditonal statement
    #[token("else")]
    Else,

    // A keyword for the end of an if-else block
    #[token("end")]
    End,

    // A keyword to begin a loop
    #[token("loop")]
    Loop,

    // A keyword for a loop-breaking conditional
    #[token("while")]
    While,

    // A keyword to mark the end of a loop
    #[token("again")]
    Again,

    // Track newlines because most statements end with one
    #[token("\n")]
    Newline,

    // A comment consumes the rest of the line
    #[regex(r"#.*")]
    Comment,

    // An 8-bit numeric literal
    #[regex(r"0x([0-9a-fA-F]{1,2})", hex_byte, priority = 2)]
    #[regex(r"0b([0,1]{1,8})", binary_byte, priority = 2)]
    #[regex(r"(25[0-5])|(2[0-4]\d)|([0,1]?\d{1,2})", byte, priority = 2)]
    Byte(u8),

    // A 16-bit numeric literal
    #[regex(r"0x([0-9a-fA-F]{1,4})", hex_number, priority = 1)]
    #[regex(r"0b([0,1]{1,16})", binary_number, priority = 1)]
    #[regex(r"\d+", number, priority = 1)]
    Number(u16),

    // A label, used for jumps, macros, and builtins.
    // Made up of any (unicode) alphanumeric character, '-', or '_'.
    #[regex(r":\S+", label, priority = 2)]
    Label(String),

    // Used for tokens we don't know how to parse yet.
    #[regex(r"\S*", priority = 0)]
    Unknown,
}

fn hex_byte(n: &mut Lexer<Token>) -> Option<u8> {
    let n = n.slice();
    let n = &n[2..];

    u8::from_str_radix(n, 16).ok()
}

fn binary_byte(n: &mut Lexer<Token>) -> Option<u8> {
    let n = n.slice();
    let n = &n[2..];

    u8::from_str_radix(n, 2).ok()
}

fn byte(n: &mut Lexer<Token>) -> Option<u8> {
    n.slice().parse().ok()
}

fn hex_number(n: &mut Lexer<Token>) -> Option<u16> {
    let n = n.slice();
    let n = &n[2..];

    u16::from_str_radix(n, 16).ok()
}

fn binary_number(n: &mut Lexer<Token>) -> Option<u16> {
    let n = n.slice();
    let n = &n[2..];

    u16::from_str_radix(n, 2).ok()
}

fn number(n: &mut Lexer<Token>) -> Option<u16> {
    n.slice().parse().ok()
}

fn label(s: &mut Lexer<Token>) -> Option<String> {
    let s = s.slice();
    let s = &s[1..];

    for c in s.chars() {
        if !c.is_alphanumeric() && !['-', '_'].contains(&c) {
            return None;
        }
    }

    Some(s.to_owned())
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
    fn test_lex_keywords() {
        let input = "return ; clear bcd save load jump jump0 hex long random";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Return)));
        assert_eq!(lexer.slice(), "return");

        assert_eq!(lexer.next(), Some(Ok(Token::Return)));
        assert_eq!(lexer.slice(), ";");

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

        assert_eq!(lexer.next(), Some(Ok(Token::Long)));
        assert_eq!(lexer.slice(), "long");

        assert_eq!(lexer.next(), Some(Ok(Token::Random)));
        assert_eq!(lexer.slice(), "random");
    }

    #[test]
    fn test_lex_conditional_keywords() {
        let input = "if begin then else end";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::If)));
        assert_eq!(lexer.slice(), "if");

        assert_eq!(lexer.next(), Some(Ok(Token::Begin)));
        assert_eq!(lexer.slice(), "begin");

        assert_eq!(lexer.next(), Some(Ok(Token::Then)));
        assert_eq!(lexer.slice(), "then");

        assert_eq!(lexer.next(), Some(Ok(Token::Else)));
        assert_eq!(lexer.slice(), "else");

        assert_eq!(lexer.next(), Some(Ok(Token::End)));
        assert_eq!(lexer.slice(), "end");
    }

    #[test]
    fn test_lex_loop_keywords() {
        let input = "loop while end";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Loop)));
        assert_eq!(lexer.slice(), "loop");

        assert_eq!(lexer.next(), Some(Ok(Token::While)));
        assert_eq!(lexer.slice(), "while");

        assert_eq!(lexer.next(), Some(Ok(Token::End)));
        assert_eq!(lexer.slice(), "end");
    }

    #[test]
    fn test_lex_comments() {
        let input = "
            v0 v1 # v2 v3 v4
            #v5 v6 v7
            v8
        "
        .trim();

        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V0))));
        assert_eq!(lexer.slice(), "v0");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V1))));
        assert_eq!(lexer.slice(), "v1");

        assert_eq!(lexer.next(), Some(Ok(Token::Comment)));
        assert_eq!(lexer.slice(), "# v2 v3 v4");

        assert_eq!(lexer.next(), Some(Ok(Token::Newline)));
        assert_eq!(lexer.slice(), "\n");

        assert_eq!(lexer.next(), Some(Ok(Token::Comment)));
        assert_eq!(lexer.slice(), "#v5 v6 v7");

        assert_eq!(lexer.next(), Some(Ok(Token::Newline)));
        assert_eq!(lexer.slice(), "\n");

        assert_eq!(lexer.next(), Some(Ok(Token::Register(Register::V8))));
        assert_eq!(lexer.slice(), "v8");
    }

    #[test]
    fn test_lex_bytes() {
        let input = "0 1 2 12 32 52 123 249 255 0xFF 0xAC 0x3D 0x00 0x12
        0b0 0b1 0b10 0b00000000 0b10101010 0b11111110";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0))));
        assert_eq!(lexer.slice(), "0");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(1))));
        assert_eq!(lexer.slice(), "1");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(2))));
        assert_eq!(lexer.slice(), "2");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(12))));
        assert_eq!(lexer.slice(), "12");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(32))));
        assert_eq!(lexer.slice(), "32");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(52))));
        assert_eq!(lexer.slice(), "52");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(123))));
        assert_eq!(lexer.slice(), "123");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(249))));
        assert_eq!(lexer.slice(), "249");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(255))));
        assert_eq!(lexer.slice(), "255");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0xFF))));
        assert_eq!(lexer.slice(), "0xFF");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0xAC))));
        assert_eq!(lexer.slice(), "0xAC");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0x3D))));
        assert_eq!(lexer.slice(), "0x3D");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0x00))));
        assert_eq!(lexer.slice(), "0x00");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0x12))));
        assert_eq!(lexer.slice(), "0x12");

        assert_eq!(lexer.next(), Some(Ok(Token::Newline)));
        assert_eq!(lexer.slice(), "\n");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0b0))));
        assert_eq!(lexer.slice(), "0b0");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0b1))));
        assert_eq!(lexer.slice(), "0b1");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0b10))));
        assert_eq!(lexer.slice(), "0b10");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0b00000000))));
        assert_eq!(lexer.slice(), "0b00000000");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0b10101010))));
        assert_eq!(lexer.slice(), "0b10101010");

        assert_eq!(lexer.next(), Some(Ok(Token::Byte(0b11111110))));
        assert_eq!(lexer.slice(), "0b11111110");
    }

    #[test]
    fn test_lex_numbers() {
        let input = "256 300 512 768 999 1024 0xFFF 0x123 0x234 0x001 0x000 0xA4C 0x100
        0b000000000 0b0000000000000001 0b1010101010101010 0b1110001100011100 0b1111111111111111";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Number(256))));
        assert_eq!(lexer.slice(), "256");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(300))));
        assert_eq!(lexer.slice(), "300");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(512))));
        assert_eq!(lexer.slice(), "512");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(768))));
        assert_eq!(lexer.slice(), "768");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(999))));
        assert_eq!(lexer.slice(), "999");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(1024))));
        assert_eq!(lexer.slice(), "1024");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0xFFF))));
        assert_eq!(lexer.slice(), "0xFFF");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0x123))));
        assert_eq!(lexer.slice(), "0x123");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0x234))));
        assert_eq!(lexer.slice(), "0x234");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0x001))));
        assert_eq!(lexer.slice(), "0x001");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0x000))));
        assert_eq!(lexer.slice(), "0x000");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0xA4C))));
        assert_eq!(lexer.slice(), "0xA4C");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0x100))));
        assert_eq!(lexer.slice(), "0x100");

        assert_eq!(lexer.next(), Some(Ok(Token::Newline)));
        assert_eq!(lexer.slice(), "\n");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0b000000000))));
        assert_eq!(lexer.slice(), "0b000000000");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0b0000000000000001))));
        assert_eq!(lexer.slice(), "0b0000000000000001");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0b1010101010101010))));
        assert_eq!(lexer.slice(), "0b1010101010101010");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0b1110001100011100))));
        assert_eq!(lexer.slice(), "0b1110001100011100");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(0b1111111111111111))));
        assert_eq!(lexer.slice(), "0b1111111111111111");
    }

    #[test]
    fn test_lex_labels() {
        let input = ":label :another-label :_yet_another_label_ :could_we_do_その他の言語
        :but-not⭐ :and-not💩 :and-not\u{2066}\u{2069} :and-also-not\u{2044}";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Label("label".into()))));
        assert_eq!(lexer.slice(), ":label");

        assert_eq!(lexer.next(), Some(Ok(Token::Label("another-label".into()))));
        assert_eq!(lexer.slice(), ":another-label");

        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Label("_yet_another_label_".into())))
        );
        assert_eq!(lexer.slice(), ":_yet_another_label_");

        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Label("could_we_do_その他の言語".into())))
        );
        assert_eq!(lexer.slice(), ":could_we_do_その他の言語");

        assert_eq!(lexer.next(), Some(Ok(Token::Newline)));
        assert_eq!(lexer.slice(), "\n");

        assert_eq!(lexer.next(), Some(Err(())));
        assert_eq!(lexer.slice(), ":but-not⭐");

        assert_eq!(lexer.next(), Some(Err(())));
        assert_eq!(lexer.slice(), ":and-not💩");

        assert_eq!(lexer.next(), Some(Err(())));
        assert_eq!(lexer.slice(), ":and-not\u{2066}\u{2069}");

        assert_eq!(lexer.next(), Some(Err(())));
        assert_eq!(lexer.slice(), ":and-also-not\u{2044}");
    }
}
