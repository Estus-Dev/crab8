use crab8::registers::Register;

use crate::token::{Position, Token};

pub fn parse(input: String) -> Vec<Token> {
    // This is a bit overkill on preallocation, but at least it won't have to keep reallocating.
    let mut tokens = Vec::with_capacity(input.len());

    for (line_num, line) in input.lines().enumerate() {
        let mut iter = line.chars().enumerate().peekable();

        while let Some(&(col_num, char)) = iter.peek() {
            if char.is_whitespace() {
                iter.next();
                continue;
            }
            let token: String = iter
                .clone()
                .take_while(|next| !next.1.is_whitespace())
                .map(|next| next.1)
                .collect();
            let length = token.len();

            let position = Position::new(line_num, col_num, length);

            tokens.push(match token.as_str() {
                "v0" => Token::Register(position, Register::V0),
                "v1" => Token::Register(position, Register::V1),
                "v2" => Token::Register(position, Register::V2),
                "v3" => Token::Register(position, Register::V3),
                "v4" => Token::Register(position, Register::V4),
                "v5" => Token::Register(position, Register::V5),
                "v6" => Token::Register(position, Register::V6),
                "v7" => Token::Register(position, Register::V7),
                "v8" => Token::Register(position, Register::V8),
                "v9" => Token::Register(position, Register::V9),
                "va" => Token::Register(position, Register::VA),
                "vb" => Token::Register(position, Register::VB),
                "vc" => Token::Register(position, Register::VC),
                "vd" => Token::Register(position, Register::VD),
                "ve" => Token::Register(position, Register::VE),
                "vf" => Token::Register(position, Register::VF),
                _ => Token::Unknown(position, token),
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
    use super::*;
    use crate::token::{Position, Token};
    use crab8::registers::Register;

    #[test]
    #[allow(clippy::identity_op, clippy::erasing_op)]
    fn test_parse_registers() {
        let cases = [(
            "v0 v1 v2 v3 v4 v5 v6 v7\nv8 v9 va vb vc vd ve vf",
            vec![
                Token::new_register(Position::new(0, 0 * 3, 2), Register::V0),
                Token::new_register(Position::new(0, 1 * 3, 2), Register::V1),
                Token::new_register(Position::new(0, 2 * 3, 2), Register::V2),
                Token::new_register(Position::new(0, 3 * 3, 2), Register::V3),
                Token::new_register(Position::new(0, 4 * 3, 2), Register::V4),
                Token::new_register(Position::new(0, 5 * 3, 2), Register::V5),
                Token::new_register(Position::new(0, 6 * 3, 2), Register::V6),
                Token::new_register(Position::new(0, 7 * 3, 2), Register::V7),
                Token::new_register(Position::new(1, 0 * 3, 2), Register::V8),
                Token::new_register(Position::new(1, 1 * 3, 2), Register::V9),
                Token::new_register(Position::new(1, 2 * 3, 2), Register::VA),
                Token::new_register(Position::new(1, 3 * 3, 2), Register::VB),
                Token::new_register(Position::new(1, 4 * 3, 2), Register::VC),
                Token::new_register(Position::new(1, 5 * 3, 2), Register::VD),
                Token::new_register(Position::new(1, 6 * 3, 2), Register::VE),
                Token::new_register(Position::new(1, 7 * 3, 2), Register::VF),
            ],
        )];

        for (input, expected) in cases {
            assert_eq!(parse(input.into()), expected, "{input}");
        }
    }
}
