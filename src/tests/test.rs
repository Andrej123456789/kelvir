use crate::src::*;

#[test]
fn add() {
    let mut lexer = lexer::Lexer::new("2+3".to_string());
    let tokens = lexer.generate_tokens();

    let mut parser = parser::Parser::new(tokens);
    let tree = parser.parse();

    let interpreter = interpreter::Interpreter{};
    let value = interpreter.visit(tree);

    assert_eq!(value.__repr__(), "5".trim())
}

#[test]
fn sub() {
    let mut lexer = lexer::Lexer::new("3-2".to_string());
    let tokens = lexer.generate_tokens();

    let mut parser = parser::Parser::new(tokens);
    let tree = parser.parse();

    let interpreter = interpreter::Interpreter{};
    let value = interpreter.visit(tree);

    assert_eq!(value.__repr__(), "1".trim())
}

#[test]
fn mul() {
    let mut lexer = lexer::Lexer::new("2*3".to_string());
    let tokens = lexer.generate_tokens();

    let mut parser = parser::Parser::new(tokens);
    let tree = parser.parse();

    let interpreter = interpreter::Interpreter{};
    let value = interpreter.visit(tree);

    assert_eq!(value.__repr__(), "6".trim())
}

#[test]
fn div() {
    let mut lexer = lexer::Lexer::new("18/6".to_string());
    let tokens = lexer.generate_tokens();

    let mut parser = parser::Parser::new(tokens);
    let tree = parser.parse();

    let interpreter = interpreter::Interpreter{};
    let value = interpreter.visit(tree);

    assert_eq!(value.__repr__(), "3".trim())
}
