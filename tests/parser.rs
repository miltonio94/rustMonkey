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

#[test]
fn test_integer_literal() {
    let input = "5;".to_string();

    let l = Box::new(lexer::Lexer::new(input));
    let mut p = parser::Parser::new(l);
    let program = match p.parse_program() {
        Some(program) => program,
        None => panic!("p.parse_program returned None, was expecting a Program"),
    };
    check_parser_errors(&p);

    assert_eq!(
        program.statements.len(),
        1,
        "was expecting program to have 1 statement, got {} instead",
        program.statements.len()
    );

    let stmt = match program.statements[0].expression_statement() {
        Some(stmt) => stmt,
        None => panic!(
            "was expecting program.statements[0] to be an ExpressionStatement, got None instead"
        ),
    };

    let literal = match stmt.expression.integer_literal() {
        Some(literal) => literal,
        None => panic!("Was expecting stmt.expression.integer_literal to be an IntegerLiteral, got None instead"),
    };

    assert_eq!(
        literal.value, 5,
        "literal.value not 5 got {} instead",
        literal.value
    );

    assert_eq!(
        literal.token_literal(),
        "5".to_string(),
        r#"was expecting literal.token_literal to be "5" got "{}" instead"#,
        literal.token_literal()
    );
}

#[test]
fn test_parsing_prefix_expression() {
    struct Test {
        input: String,
        operator: String,
        integer_value: i64,
    }
    impl Test {
        fn new(input: String, operator: String, integer_value: i64) -> Test {
            Test {
                input,
                operator,
                integer_value,
            }
        }
    }

    let prefix_test = vec![
        Test::new("!15;".to_string(), "!".to_string(), 15),
        Test::new("-15;".to_string(), "-".to_string(), 15),
    ];

    for tt in prefix_test.iter() {
        let l = Box::new(lexer::Lexer::new(tt.input.clone()));
        let mut p = parser::Parser::new(l);
        let program = match p.parse_program() {
            Some(program) => program,
            None => panic!("p.parse_program returned None, was expecting Some(program)"),
        };
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "Was expecting program.statements to have the length of 1 got {} instead",
            program.statements.len()
        );

        let stmt = match program.statements[0].expression_statement() {
            Some(stmt) => stmt,
            None => panic!("program.Statements[0].expression_statement() returned None"),
        };

        let mut exp = match stmt.expression.prefix_expression() {
            Some(exp) => exp,
            None => panic!("stmt.expression is not a PrefixExpression, got None"),
        };

        assert_eq!(
            exp.operator, tt.operator,
            "Was expecting exp.operator to be {} got {} instead",
            tt.operator, exp.operator
        );

        integer_literal_test(exp.right.as_ref(), tt.integer_value);
    }
}

fn integer_literal_test(il: &ast::Expression, value: i64) {
    let integ = match il.integer_literal() {
        Some(integ) => integ,
        None => panic!("il not IntegerLiteral, got {:?}", il),
    };

    assert_eq!(
        integ.value, value,
        "integ.value not {}, got {}",
        value, integ.value
    );

    let value_string = format!("{}", value);
    assert_eq!(
        integ.token_literal(),
        value_string,
        "integ.TokenLiteral not {} got {} instead",
        value_string,
        integ.token_literal()
    );
}

#[test]
fn test_parsing_infix_expression() {
    struct Test {
        input: String,
        left_value: i64,
        operator: String,
        right_value: i64,
    }
    fn new(input: String, left_value: i64, operator: String, right_value: i64) -> Test {
        Test {
            input,
            left_value,
            operator,
            right_value,
        }
    }

    let infix_tests = vec![
        new("5 + 5".to_string(), 5, "+".to_string(), 5),
        new("5 - 5".to_string(), 5, "-".to_string(), 5),
        new("5 * 5".to_string(), 5, "*".to_string(), 5),
        new("5 / 5".to_string(), 5, "/".to_string(), 5),
        new("5 > 5".to_string(), 5, ">".to_string(), 5),
        new("5 < 5".to_string(), 5, "<".to_string(), 5),
        new("5 == 5".to_string(), 5, "==".to_string(), 5),
        new("5 != 5".to_string(), 5, "!=".to_string(), 5),
    ];

    for tt in infix_tests.iter() {
        let l = Box::new(lexer::Lexer::new(tt.input.clone()));
        let mut p = parser::Parser::new(l);
        let program = match p.parse_program() {
            Some(program) => program,
            None => panic!("p.parse_program() returned None"),
        };
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "Was expecting program.statements to have 1 statement got {} instead",
            program.statements.len()
        );

        let stmt = match program.statements[0].expression_statement() {
            Some(stmt) => stmt,
            None => panic!(
                "program.statements[0] is not an expression statement, got {:?} instead",
                program.statements[0]
            ),
        };

        let exp = match stmt.expression.infix_expression() {
            Some(exp) => exp,
            None => panic!(
                "exp.Expression is not an InfixExpression got {:?} instead",
                stmt.expression
            ),
        };

        integer_literal_test(&exp.left, tt.left_value);
        assert_eq!(
            exp.operator, tt.operator,
            "exp.operator is not {} got {} instead",
            tt.operator, exp.operator
        );
        integer_literal_test(&exp.right, tt.right_value);
    }
}

#[test]
fn test_operator_precedence_parsing() {
    struct Test {
        input: String,
        expected: String,
    }
    fn new(input: &str, expected: &str) -> Test {
        Test {
            input: input.to_string(),
            expected: expected.to_string(),
        }
    }

    let tests = vec![
        new("-a * b", "((-a) * b)"),
        new("!-a", "(!(-a))"),
        new("a + b + c", "((a + b) + c)"),
        new("a + b - c", "((a + b) - c)"),
        new("a * b * c", "((a * b) * c)"),
        new("a * b / c", "((a * b) / c)"),
        new("a + b / c", "(a + (b / c))"),
        new("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        new("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        new("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        new("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        new(
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        new(
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        new("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        new("(5 + 5) * 2", "((5 + 5) * 2)"),
        new("2 / (5 + 5)", "(2 / (5 + 5))"),
        new("-(5 + 5)", "(-(5 + 5))"),
        new("!(true == true)", "(!(true == true))"),
    ];

    for tt in tests.iter() {
        let l = Box::new(lexer::Lexer::new(tt.input.clone()));
        let mut p = parser::Parser::new(l);
        let program = match p.parse_program() {
            Some(program) => program,
            None => panic!("p.parse_program() returned None"),
        };
        check_parser_errors(&p);

        let actual = program.to_string();

        assert_eq!(
            actual, tt.expected,
            " expected = {}, got {} instead",
            tt.expected, actual
        );
    }
}

#[test]
fn test_bool() {
    struct Test {
        input: String,
        expected: String,
    }
    fn new(input: &str, expected: &str) -> Test {
        Test {
            input: input.to_string(),
            expected: expected.to_string(),
        }
    }

    let tests = vec![new("true", "true"), new("false", "false")];

    for tt in tests.iter() {
        let l = Box::new(lexer::Lexer::new(tt.input.clone()));
        let mut p = parser::Parser::new(l);
        let program = match p.parse_program() {
            Some(program) => program,
            None => panic!("p.parse_program() returned None"),
        };
        check_parser_errors(&p);

        let actual = program.to_string();

        assert_eq!(
            actual, tt.expected,
            " expected = {}, got {} instead",
            tt.expected, actual
        );
    }
}
