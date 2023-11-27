use rust_monkey::ast;
use rust_monkey::lexer;
use rust_monkey::parser;

#[test]

fn test_let_statements() {
    struct Test {
        pub expected_identifier: String,
    }
    impl Test {
        pub fn new(ident: &str) -> Self {
            Self {
                expected_identifier: ident.to_string(),
            }
        }
    }

    let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;
    let l = Box::new(lexer::Lexer::new(input.to_string()));
    let mut p = parser::Parser::new(l);
    let program = match p.parse_program() {
        Some(program) => program,
        None => panic!("parse_program returned a None"),
    };

    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements. got {}",
        program.statements.len()
    );

    let tests = vec![Test::new("x"), Test::new("y"), Test::new("foobar")];

    for (i, tt) in tests.iter().enumerate() {
        let stmt = &program.statements[i];
        test_let(stmt, tt.expected_identifier.clone());
    }
}

fn test_let(s: &ast::Statement, name: String) {
    let let_statement = match s.let_statement() {
        Some(let_statement) => let_statement,
        None => panic!("Expected LetStatement, got none"),
    };

    assert_eq!(
        let_statement.token.to_string(),
        "let".to_string(),
        "let_statement.token.to_string() not 'let' got {}",
        let_statement.token.to_string()
    );

    assert_eq!(
        let_statement.name.value, name,
        "let_statement.name.value not {} got {}",
        name, let_statement.name.value
    );

    assert_eq!(
        let_statement.name.token.to_string(),
        name,
        "let_statement.name.token.to_string not {}, got {}",
        name,
        let_statement.name.token.to_string()
    )
}
