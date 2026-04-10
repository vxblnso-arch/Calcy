use crossterm::{
    self, ExecutableCommand, cursor,
    event::Event,
    event::KeyCode,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write, stdout};
#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Times,
    Divide,
}

fn tokenize(input: &str) -> Token {
    match input {
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Times,
        "/" => Token::Divide,
        _ => {
            if let Ok(n) = input.parse::<f64>() {
                Token::Number(n)
            } else {
                panic!("you probably typed a letter or forgot to press space... ")
            }
        }
    }
}

fn tokenize_line(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for word in line.split_whitespace() {
        tokens.push(tokenize(word))
    }
    tokens
}

enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn evaluate(expr: Expr) -> f64 {
    match expr {
        Expr::Number(n) => n,
        Expr::Add(left, right) => evaluate(*left) + evaluate(*right),
        Expr::Sub(left, right) => evaluate(*left) - evaluate(*right),
        Expr::Mul(left, right) => evaluate(*left) * evaluate(*right),
        Expr::Div(left, right) => evaluate(*left) / evaluate(*right),
    }
}

fn parse_term(tokens: &[Token], pos: &mut usize) -> Expr {
    let mut left = match &tokens[*pos] {
        Token::Number(n) => {
            *pos += 1;
            Expr::Number(*n)
        }
        _ => panic!("expected number"),
    };
    while *pos < tokens.len() && matches!(tokens[*pos], Token::Divide | Token::Times,) {
        let op = match &tokens[*pos] {
            Token::Times => {
                *pos += 1;
                Token::Times
            }
            Token::Divide => {
                *pos += 1;
                Token::Divide
            }
            _ => panic!("expected operation at token number {}", *pos),
        };

        let right = match &tokens[*pos] {
            Token::Number(n) => {
                *pos += 1;
                Expr::Number(*n)
            }
            _ => panic!("expected number"),
        };

        left = match op {
            Token::Times => Expr::Mul(Box::new(left), Box::new(right)),
            Token::Divide => Expr::Div(Box::new(left), Box::new(right)),
            _ => panic!("Tf did you do for it to panic here???"),
        }
    }
    left
}

fn parse_expr(tokens: &[Token], pos: &mut usize) -> Expr {
    let mut left = parse_term(tokens, pos);
    while *pos < tokens.len() && matches!(tokens[*pos], Token::Plus | Token::Minus,) {
        let op = match &tokens[*pos] {
            Token::Plus => {
                *pos += 1;
                Token::Plus
            }
            Token::Minus => {
                *pos += 1;
                Token::Minus
            }
            _ => panic!("expected operation at token number {}", *pos),
        };
        let right = parse_term(tokens, pos);

        left = match op {
            Token::Plus => Expr::Add(Box::new(left), Box::new(right)),
            Token::Minus => Expr::Sub(Box::new(left), Box::new(right)),
            _ => panic!("No way you got this message. How????"),
        }
    }

    left
}

fn main() {
    println!("This is a simple Calculator.\nEnter your operation: \n");
    crossterm::terminal::enable_raw_mode().unwrap();
    let mut calc = String::new();
    let mut answers: Vec<f64> = Vec::new();

    loop {
        match crossterm::event::read().unwrap() {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char(c) => {
                    calc.push(c);
                    print!("{c}");
                    io::stdout().flush().unwrap(); // I do not understand whatever tf this is but
                    // google does!
                }
                KeyCode::Backspace => {
                    calc.pop();
                    let mut stdout = stdout();
                    stdout.execute(cursor::MoveLeft(1)).unwrap();
                    execute!(stdout, Clear(ClearType::UntilNewLine)).unwrap(); // WTF is an stdout
                }
                KeyCode::Enter => {
                    if calc.is_empty() {
                        continue;
                    };
                    if calc.trim() == "q" || calc.trim() == "exit" {
                        break;
                    };
                    let tokens = tokenize_line(&calc);
                    let mut pos: usize = 0;
                    let result = evaluate(parse_expr(&tokens, &mut pos));
                    println!("\n{result}");
                    answers.push(result);
                    calc.clear();
                }
                KeyCode::Up => {
                    if let Some(last) = answers.last() {
                        let c = &last.to_string();
                        calc.push_str(c);
                        print!("{c}");
                        io::stdout().flush().unwrap(); // Seriously what?
                    }
                }
                _ => panic!("Wtf"),
            },
            _ => continue,
        }
    }
} //Praise my lord google!!!
