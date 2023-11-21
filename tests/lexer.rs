use rustMonkey::lexer;
use rustMonkey::token::Token;
use std::string;
use std::thread::panicking;

struct Test {
    expected_type: Token,
    expected_literal: String,
}

#[test]
fn test_next_token() {
    let input = "=+(){},;".to_string();
    let tests = vec![
        Test {
            expected_type: Token::Assign,
            expected_literal: "=".to_string(),
        },
        Test {
            expected_type: Token::Plus,
            expected_literal: "+".to_string(),
        },
        Test {
            expected_type: Token::LParen,
            expected_literal: "(".to_string(),
        },
        Test {
            expected_type: Token::RParen,
            expected_literal: ")".to_string(),
        },
        Test {
            expected_type: Token::LBrace,
            expected_literal: "{".to_string(),
        },
        Test {
            expected_type: Token::RBrace,
            expected_literal: "}".to_string(),
        },
        Test {
            expected_type: Token::Comma,
            expected_literal: ",".to_string(),
        },
        Test {
            expected_type: Token::Semicolon,
            expected_literal: ";".to_string(),
        },
    ];

    let mut l = lexer::Lexer::new(input);

    for (i, tt) in tests.iter().enumerate() {
        let tok = l.next_token();

        if tok != tt.expected_type {
            panic!(
                "tests[{}] - token type wrong. expected {:?} got {:?}",
                i, tt.expected_type, tok
            )
        }

        if tok.to_string() != tt.expected_literal {
            panic!(
                "tests[{} - literal wrong. Expected {}, got {}]",
                i,
                tt.expected_literal,
                tok.to_string()
            )
        }
    }
}
