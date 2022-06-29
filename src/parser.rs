use std::collections::HashMap;

use crate::{polynomial::Polynomial, rational::Rational};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Token {
    Number(Rational),
    Operator(char),
    Symbol(String),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars = input.chars().collect::<Vec<_>>();

    while i < input.len() {
        let c = chars[i];

        match c {
            '0'..='9' => {
                let mut number = 0;

                while i < input.len() && ('0'..='9').contains(&chars[i]) {
                    number = number * 10 + (chars[i] as i32 - '0' as i32);
                    i += 1;
                }

                tokens.push(Token::Number(Rational::from(number)));

                continue;
            }
            '+' | '-' | '*' | '/' | '^' | '=' => tokens.push(Token::Operator(c)),
            'x' => tokens.push(Token::Symbol(String::from("x"))),
            ' ' => (),
            _ => println!("Unknown: {}", c),
        }

        i += 1
    }

    tokens
}

pub fn parse_polynomial_expr(input: &str) -> Polynomial {
    let tokens = tokenize(input);
    let mut i = 0;

    let mut coeffs = HashMap::new();

    let mut sign = 1;
    let mut equals_seen = false;

    // NOTE: i has to be passed as a mut reference because otherwise it is borrowed for the duration of the closing function which makes borrowck angy
    let parse_exponent = |i: &mut usize| -> Option<i32> {
        if tokens.get(*i) == Some(&Token::Operator('^')) {
            *i += 1;

            let exponent = match tokens[*i] {
                Token::Number(value) => value,
                _ => panic!("expected number after exponentiation operator"),
            };
            *i += 1;

            let exponent = exponent
                .as_integer()
                .expect("parse_polynomial_expr: exponents currently must be integers");

            assert!(
                exponent >= 0,
                "parse_polynomial_expr: exponents currently must be non-negative"
            );

            return Some(exponent);
        }

        None
    };

    while i < tokens.len() {
        match tokens[i] {
            Token::Number(value) => {
                i += 1;

                let coefficient = Rational::from(sign * if equals_seen { -1 } else { 1 }) * value;

                if tokens.get(i) == Some(&Token::Symbol("x".to_string())) {
                    i += 1;

                    if let Some(exponent) = parse_exponent(&mut i) {
                        *coeffs
                            .entry(exponent as u32)
                            .or_insert_with(|| Rational::from(0)) += coefficient;
                    } else {
                        *coeffs.entry(1).or_insert_with(|| Rational::from(0)) += coefficient;
                    }
                } else {
                    *coeffs.entry(0).or_insert_with(|| Rational::from(0)) += coefficient;
                }

                sign = 1;

                continue;
            }
            Token::Symbol(ref name) if name == "x" => {
                i += 1;

                let coefficient = Rational::from(sign * if equals_seen { -1 } else { 1 });

                if let Some(exponent) = parse_exponent(&mut i) {
                    *coeffs
                        .entry(exponent as u32)
                        .or_insert_with(|| Rational::from(0)) += coefficient;
                } else {
                    *coeffs.entry(1).or_insert_with(|| Rational::from(0)) += coefficient;
                }

                sign = 1;

                continue;
            }
            Token::Operator('-') => {
                sign = -sign;
            }
            Token::Operator('=') => {
                equals_seen = true;
            }
            _ => (),
        }

        i += 1;
    }

    Polynomial::new(coeffs)
}
