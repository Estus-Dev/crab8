use crate::token::Token;
use crab8::registers::Register;

pub fn lex(input: &str) -> Vec<Token> {
    // This is a bit overkill on preallocation, but at least it won't have to keep reallocating.
    let mut tokens = Vec::with_capacity(input.len());

    for line in input.lines() {
        let mut iter = line.chars().peekable();

        while let Some(&char) = iter.peek() {
            if char.is_whitespace() {
                iter.next();
                continue;
            }
            let token: String = iter
                .clone()
                .take_while(|next| !next.is_whitespace())
                .collect();
            let length = token.len();

            tokens.push(match token.as_str() {
                // "v0" => Token::Register(Register::V0),
                // "v1" => Token::Register(Register::V1),
                // "v2" => Token::Register(Register::V2),
                // "v3" => Token::Register(Register::V3),
                // "v4" => Token::Register(Register::V4),
                // "v5" => Token::Register(Register::V5),
                // "v6" => Token::Register(Register::V6),
                // "v7" => Token::Register(Register::V7),
                // "v8" => Token::Register(Register::V8),
                // "v9" => Token::Register(Register::V9),
                // "va" => Token::Register(Register::VA),
                // "vb" => Token::Register(Register::VB),
                // "vc" => Token::Register(Register::VC),
                // "vd" => Token::Register(Register::VD),
                // "ve" => Token::Register(Register::VE),
                // "vf" => Token::Register(Register::VF),
                ":=" => Token::Assign,
                "+=" => Token::Add,
                "-=" => Token::Sub,
                "=-" => Token::SubFrom,
                "&=" => Token::And,
                "|=" => Token::Or,
                "^=" => Token::Xor,
                "<<=" => Token::LShift,
                ">>=" => Token::RShift,
                "==" => Token::Eq,
                "!=" => Token::Neq,
                "<" => Token::Lt,
                ">" => Token::Gt,
                "<=" => Token::Lte,
                ">=" => Token::Gte,
                "key" => Token::Key,
                "-key" => Token::NKey,
                _ => Token::Unknown,
            });

            for _ in 0..length {
                iter.next();
            }
        }
    }

    tokens
}

#[cfg(test)]
mod test {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_lex_registers() {
        let input = "v0 v1 v2 v3 v4 v5 v6 v7\nv8 v9 va vb vc vd ve vf";
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
    fn test_lex_assign() {
        let input = ":=";
        let expected = vec![Token::Assign];

        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_lex_basic_math_ops() {
        let input = "+= -= =-";
        let expected = vec![Token::Add, Token::Sub, Token::SubFrom];

        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_lex_bitwise_ops() {
        let input = "&= |= ^=";
        let expected = vec![Token::And, Token::Or, Token::Xor];

        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_lex_shift_ops() {
        let input = "<<= >>=";
        let expected = vec![Token::LShift, Token::RShift];

        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_lex_comparison_ops() {
        let input = "== != <  >  <= >=";
        let expected = vec![
            Token::Eq,
            Token::Neq,
            Token::Lt,
            Token::Gt,
            Token::Lte,
            Token::Gte,
        ];

        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_lex_input_ops() {
        let input = "key -key";
        let expected = vec![Token::Key, Token::NKey];

        assert_eq!(lex(input), expected);
    }
}
