use std::io::Write;

mod src {
    pub mod tokens;
    pub mod lexer;
    pub mod nodes;
    pub mod parser;
}

fn main() {
    loop {
        print!("kelvir > ");
        std::io::stdout().flush().unwrap();

        let mut user_input = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut user_input).expect("Failed to retrieve user input!");

        if user_input.trim() == "exit".trim() {
            std::process::exit(0);
        }

        let mut lexer = src::lexer::Lexer::new(user_input.trim().to_string());
        let tokens = lexer.generate_tokens();

        let mut parser = src::parser::Parser::new(tokens);
        let tree = parser.parse();

        match tree {
            src::nodes::ALL_VARIANT::Number(number_rc) => {
                let number = &*number_rc; 
                println!("{}", number.value)
            }
            src::nodes::ALL_VARIANT::Add(_) => {
                println!("Add")
            }
            src::nodes::ALL_VARIANT::Sub(_) => {
                println!("Sub")
            },
            src::nodes::ALL_VARIANT::Mul(_) => {
                println!("Mul")
            },
            src::nodes::ALL_VARIANT::Div(_) => {
                println!("Div")
            }
            src::nodes::ALL_VARIANT::Plus(_) => {
                println!("Plus")
            },
            src::nodes::ALL_VARIANT::Minus(_) => {
                println!("Minus")
            },
            src::nodes::ALL_VARIANT::EmptyNode() => {
                println!("NULL")
            },
        }

        println!("");
    }
}
