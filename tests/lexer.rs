use rust_monkey::lexer;
use rust_monkey::token::TokenType;

struct Test {
    expected_type: TokenType,
    expected_literal: String,
}

#[test]
fn test_next_token() {
    let input = r#"
let five = 5;
let ten = 10;
let add = fn(x, y){
   x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#
    .to_string();

    let tests = vec![
        Test {
            expected_type: TokenType::Let,
            expected_literal: "let".to_string(),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: "five".to_string(),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: "=".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "5".to_string(),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: TokenType::Let,
            expected_literal: "let".to_string(),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: "ten".to_string(),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: "=".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: TokenType::Let,
            expected_literal: "let".to_string(),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: "add".to_string(),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: "=".to_string(),
        },
        Test {
            expected_type: TokenType::Function,
            expected_literal: "fn".to_string(),
        },
        Test {
            expected_type: TokenType::LParen,
            expected_literal: "(".to_string(),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: "x".to_string(),
        },
        Test {
            expected_type: TokenType::Comma,
            expected_literal: ",".to_string(),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: "y".to_string(),
        },
        Test {
            expected_type: TokenType::RParen,
            expected_literal: ")".to_string(),
        },
        Test {
            expected_type: TokenType::LBrace,
            expected_literal: "{".to_string(),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: "x".to_string(),
        },
        Test {
            expected_type: TokenType::Plus,
            expected_literal: "+".to_string(),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: "y".to_string(),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: TokenType::RBrace,
            expected_literal: "}".to_string(),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: TokenType::Let,
            expected_literal: "let".to_string(),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: "result".to_string(),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: "=".to_string(),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: "add".to_string(),
        },
        Test {
            expected_type: TokenType::LParen,
            expected_literal: "(".to_string(),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: "five".to_string(),
        },
        Test {
            expected_type: TokenType::Comma,
            expected_literal: ",".to_string(),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: "ten".to_string(),
        },
        Test {
            expected_type: TokenType::RParen,
            expected_literal: ")".to_string(),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: TokenType::Bang,
            expected_literal: "!".to_string(),
        },
        Test {
            expected_type: TokenType::Minus,
            expected_literal: "-".to_string(),
        },
        Test {
            expected_type: TokenType::Slash,
            expected_literal: "/".to_string(),
        },
        Test {
            expected_type: TokenType::Asterisk,
            expected_literal: "*".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "5".to_string(),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "5".to_string(),
        },
        Test {
            expected_type: TokenType::Lt,
            expected_literal: "<".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        Test {
            expected_type: TokenType::Gt,
            expected_literal: ">".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "5".to_string(),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: TokenType::If,
            expected_literal: "if".to_string(),
        },
        Test {
            expected_type: TokenType::LParen,
            expected_literal: "(".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "5".to_string(),
        },
        Test {
            expected_type: TokenType::Lt,
            expected_literal: "<".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        Test {
            expected_type: TokenType::RParen,
            expected_literal: ")".to_string(),
        },
        Test {
            expected_type: TokenType::LBrace,
            expected_literal: "{".to_string(),
        },
        Test {
            expected_type: TokenType::Return,
            expected_literal: "return".to_string(),
        },
        Test {
            expected_type: TokenType::True,
            expected_literal: "true".to_string(),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: TokenType::RBrace,
            expected_literal: "}".to_string(),
        },
        Test {
            expected_type: TokenType::Else,
            expected_literal: "else".to_string(),
        },
        Test {
            expected_type: TokenType::LBrace,
            expected_literal: "{".to_string(),
        },
        Test {
            expected_type: TokenType::Return,
            expected_literal: "return".to_string(),
        },
        Test {
            expected_type: TokenType::False,
            expected_literal: "false".to_string(),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: TokenType::RBrace,
            expected_literal: "}".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        Test {
            expected_type: TokenType::Eq,
            expected_literal: "==".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        Test {
            expected_type: TokenType::NotEq,
            expected_literal: "!=".to_string(),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: "9".to_string(),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: ";".to_string(),
        },
    ];

    let mut l = lexer::Lexer::new(input);

    for (i, tt) in tests.iter().enumerate() {
        let tok = l.next_token().unwrap();

        if tok.token_type != tt.expected_type {
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
