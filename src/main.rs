use std::io::Write;

mod iterator;   
mod src {
    pub mod tokens;
    pub mod lexer;
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
        let token = lexer.generate_tokens();

        println!("{}", token.value);
    }
}
