use rust_monkey::lexer;
use rust_monkey::token::Token;

struct Test {
    expected_type: Token,
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
            expected_type: Token::Let,
            expected_literal: "let".to_string(),
        },
        Test {
            expected_type: Token::Ident("five".to_string().into_bytes()),
            expected_literal: "five".to_string(),
        },
        Test {
            expected_type: Token::Assign,
            expected_literal: "=".to_string(),
        },
        Test {
            expected_type: Token::Int("5".to_string().into_bytes()),
            expected_literal: "5".to_string(),
        },
        Test {
            expected_type: Token::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: Token::Let,
            expected_literal: "let".to_string(),
        },
        Test {
            expected_type: Token::Ident("ten".to_string().into_bytes()),
            expected_literal: "ten".to_string(),
        },
        Test {
            expected_type: Token::Assign,
            expected_literal: "=".to_string(),
        },
        Test {
            expected_type: Token::Int("10".to_string().into_bytes()),
            expected_literal: "10".to_string(),
        },
        Test {
            expected_type: Token::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: Token::Let,
            expected_literal: "let".to_string(),
        },
        Test {
            expected_type: Token::Ident("add".to_string().into_bytes()),
            expected_literal: "add".to_string(),
        },
        Test {
            expected_type: Token::Assign,
            expected_literal: "=".to_string(),
        },
        Test {
            expected_type: Token::Function,
            expected_literal: "fn".to_string(),
        },
        Test {
            expected_type: Token::LParen,
            expected_literal: "(".to_string(),
        },
        Test {
            expected_type: Token::Ident("x".to_string().into_bytes()),
            expected_literal: "x".to_string(),
        },
        Test {
            expected_type: Token::Comma,
            expected_literal: ",".to_string(),
        },
        Test {
            expected_type: Token::Ident("y".to_string().into_bytes()),
            expected_literal: "y".to_string(),
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
            expected_type: Token::Ident("x".to_string().into_bytes()),
            expected_literal: "x".to_string(),
        },
        Test {
            expected_type: Token::Plus,
            expected_literal: "+".to_string(),
        },
        Test {
            expected_type: Token::Ident("y".to_string().into_bytes()),
            expected_literal: "y".to_string(),
        },
        Test {
            expected_type: Token::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: Token::RBrace,
            expected_literal: "}".to_string(),
        },
        Test {
            expected_type: Token::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: Token::Let,
            expected_literal: "let".to_string(),
        },
        Test {
            expected_type: Token::Ident("result".to_string().into_bytes()),
            expected_literal: "result".to_string(),
        },
        Test {
            expected_type: Token::Assign,
            expected_literal: "=".to_string(),
        },
        Test {
            expected_type: Token::Ident("add".to_string().into_bytes()),
            expected_literal: "add".to_string(),
        },
        Test {
            expected_type: Token::LParen,
            expected_literal: "(".to_string(),
        },
        Test {
            expected_type: Token::Ident("five".to_string().into_bytes()),
            expected_literal: "five".to_string(),
        },
        Test {
            expected_type: Token::Comma,
            expected_literal: ",".to_string(),
        },
        Test {
            expected_type: Token::Ident("ten".to_string().into_bytes()),
            expected_literal: "ten".to_string(),
        },
        Test {
            expected_type: Token::RParen,
            expected_literal: ")".to_string(),
        },
        Test {
            expected_type: Token::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: Token::Bang,
            expected_literal: "!".to_string(),
        },
        Test {
            expected_type: Token::Minus,
            expected_literal: "-".to_string(),
        },
        Test {
            expected_type: Token::Slash,
            expected_literal: "/".to_string(),
        },
        Test {
            expected_type: Token::Asterisk,
            expected_literal: "*".to_string(),
        },
        Test {
            expected_type: Token::Int("5".to_string().into_bytes()),
            expected_literal: "5".to_string(),
        },
        Test {
            expected_type: Token::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: Token::Int("5".to_string().into_bytes()),
            expected_literal: "5".to_string(),
        },
        Test {
            expected_type: Token::Lt,
            expected_literal: "<".to_string(),
        },
        Test {
            expected_type: Token::Int("10".to_string().into_bytes()),
            expected_literal: "10".to_string(),
        },
        Test {
            expected_type: Token::Gt,
            expected_literal: ">".to_string(),
        },
        Test {
            expected_type: Token::Int("5".to_string().into_bytes()),
            expected_literal: "5".to_string(),
        },
        Test {
            expected_type: Token::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: Token::If,
            expected_literal: "if".to_string(),
        },
        Test {
            expected_type: Token::LParen,
            expected_literal: "(".to_string(),
        },
        Test {
            expected_type: Token::Int("5".to_string().into_bytes()),
            expected_literal: "5".to_string(),
        },
        Test {
            expected_type: Token::Lt,
            expected_literal: "<".to_string(),
        },
        Test {
            expected_type: Token::Int("10".to_string().into_bytes()),
            expected_literal: "10".to_string(),
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
            expected_type: Token::Return,
            expected_literal: "return".to_string(),
        },
        Test {
            expected_type: Token::True,
            expected_literal: "true".to_string(),
        },
        Test {
            expected_type: Token::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: Token::RBrace,
            expected_literal: "}".to_string(),
        },
        Test {
            expected_type: Token::Else,
            expected_literal: "else".to_string(),
        },
        Test {
            expected_type: Token::LBrace,
            expected_literal: "{".to_string(),
        },
        Test {
            expected_type: Token::Return,
            expected_literal: "return".to_string(),
        },
        Test {
            expected_type: Token::False,
            expected_literal: "false".to_string(),
        },
        Test {
            expected_type: Token::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: Token::RBrace,
            expected_literal: "}".to_string(),
        },
        Test {
            expected_type: Token::Int("10".to_string().into_bytes()),
            expected_literal: "10".to_string(),
        },
        Test {
            expected_type: Token::Eq,
            expected_literal: "==".to_string(),
        },
        Test {
            expected_type: Token::Int("10".to_string().into_bytes()),
            expected_literal: "10".to_string(),
        },
        Test {
            expected_type: Token::Semicolon,
            expected_literal: ";".to_string(),
        },
        Test {
            expected_type: Token::Int("10".to_string().into_bytes()),
            expected_literal: "10".to_string(),
        },
        Test {
            expected_type: Token::NotEq,
            expected_literal: "!=".to_string(),
        },
        Test {
            expected_type: Token::Int("9".to_string().into_bytes()),
            expected_literal: "9".to_string(),
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
