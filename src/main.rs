use std::io::Write;

mod src {
    pub mod tokens;
    pub mod lexer;
    pub mod nodes;
    pub mod parser;
}

fn match_node(node: src::nodes::ALL_VARIANT) {
    match node {
        src::nodes::ALL_VARIANT::Number(number_rc) => {
            let number: &src::nodes::NumberNode = &*number_rc; 
            println!("{}", number.value)
        }
        src::nodes::ALL_VARIANT::Add(rc) => {
            let node: &src::nodes::AddNode = &*rc;

            match_node(node.node_a.clone());
            match_node(node.node_b.clone());
        }
        src::nodes::ALL_VARIANT::Sub(rc) => {
            let node: &src::nodes::SubtractNode = &*rc;

            match_node(node.node_a.clone());
            match_node(node.node_b.clone());
        },
        src::nodes::ALL_VARIANT::Mul(rc) => {
            let node: &src::nodes::MultiplyNode = &*rc;

            match_node(node.node_a.clone());
            match_node(node.node_b.clone());
        },
        src::nodes::ALL_VARIANT::Div(rc) => {
            let node: &src::nodes::DivideNode = &*rc;

            match_node(node.node_a.clone());
            match_node(node.node_b.clone());
        }
        src::nodes::ALL_VARIANT::Plus(rc) => {
            let node: &src::nodes::PlusNode = &*rc;
            match_node(node.node.clone());
        },
        src::nodes::ALL_VARIANT::Minus(rc) => {
            let node: &src::nodes::MinusNode = &*rc;
            match_node(node.node.clone());
        },
        src::nodes::ALL_VARIANT::EmptyNode() => {
            println!("NULL")
        },
    }
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

        match_node(tree)
    }
}
