use rust_monkey::ast::{self, NodeInterface};
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
    check_parser_errors(&p);

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
        let_statement.token_literal(),
        "let".to_string(),
        "let_statement.token.to_string() not 'let' got {}",
        let_statement.token_literal()
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

#[test]
fn test_return_statement() {
    let input = "return 5; return 10; return 993322;".to_string();

    let l = Box::new(lexer::Lexer::new(input));
    let mut p = parser::Parser::new(l);

    let program = match p.parse_program() {
        Some(p) => p,
        None => panic!("parse_program returned a None"),
    };

    check_parser_errors(&p);

    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements, got {} instead",
        program.statements.len()
    );

    for stmt in program.statements.iter() {
        let return_statement = match stmt.return_statement() {
            Some(stmt) => stmt,
            None => panic!("expected ReturnStatement got None"),
        };

        assert_eq!(
            return_statement.token.to_string(),
            "return".to_string(),
            "return_statement.token_literal() not 'return', got {} instead",
            return_statement.token.to_string()
        );
    }
}

fn check_parser_errors(p: &parser::Parser) {
    let errors = p.errors();

    let errors: Vec<String> = errors
        .iter()
        .map(|e| format!("parser errors: {}", e))
        .collect();

    assert_eq!(
        errors.len(),
        0,
        "parser has {} errors\n{:#?}",
        errors.len(),
        errors
    );
}

#[test]
fn test_identifier() {
    let input = "foobar;".to_string();
    let l = Box::new(lexer::Lexer::new(input));
    let mut p = parser::Parser::new(l);
    let program = match p.parse_program() {
        Some(p) => p,
        None => panic!("was expecting p.parse_program to return Some, got None"),
    };
    check_parser_errors(&p);

    assert_eq!(
        program.statements.len(),
        1,
        "was expecting program to have 1 statement, got {} instead",
        program.statements.len()
    );

    let stmt = match program.statements[0].expression_statement() {
        Some(exp) => exp,
        None => panic!("Was expecting expression got None"),
    };

    let ident = match stmt.expression.identifier() {
        Some(idnt) => idnt,
        None => panic!("Was expecting an Identifier got None"),
    };

    assert_eq!(
        ident.token_literal(),
        "foobar",
        "ident.token_literal is not foobar got {} instead",
        ident.token_literal()
    );
}
