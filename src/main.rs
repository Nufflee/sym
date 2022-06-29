mod parser;
mod polynomial;
mod rational;
mod solver;

use parser::parse_polynomial_expr;
use polynomial::Polynomial;
use rational::Rational;
use std::{cmp::Ordering, collections::HashMap};

fn solve_univariate_polynomial(poly: &Polynomial) -> Vec<Rational> {
    match poly.degree() {
        1 => vec![-poly.get(0) / poly.get(1)],
        2 => {
            let a = poly.get(2);
            let b = poly.get(1);
            let c = poly.get(0);

            let discriminant = b * b - Rational::from(4) * a * c;

            match discriminant.cmp(&Rational::from(0)) {
                Ordering::Greater => {
                    vec![
                        (-b - discriminant.sqrt()) / (Rational::from(2) * a),
                        (-b + discriminant.sqrt()) / (Rational::from(2) * a),
                    ]
                }
                Ordering::Equal => vec![-b / (Rational::from(2) * a)],
                Ordering::Less => vec![],
            }
        }
        // TODO: Analytical solutions for 3rd degree polynomials
        /* 3 => {
            let a = coeffs[&3];
            let b = coeffs[&2];
            let c = coeffs[&1];
            let d = coeffs[&0];

            // https://en.wikipedia.org/wiki/Cubic_equation#General_cubic_formula
            let d0 = b.pow(2) - Rational::from(3) * a * c;
            let d1 = Rational::from(2) * b.pow(3) - Rational::from(9) * a * b * c
                + Rational::from(27) * a.pow(2) * d;

            if d0 == Rational::from(0) && d1 == Rational::from(0) {
                // Triple root
                return vec![
                    -b / Rational::from(3) * a,
                    -b / Rational::from(3) * a,
                    -b / Rational::from(3) * a,
                ];
            }

            // dbg!(d0, d1);
            // dbg!(d1.pow(2) - Rational::from(4) * d0.pow(3));
            // dbg!((d1 - (d1.pow(2) - Rational::from(4) * d0.pow(3)).sqrt()) / Rational::from(2));

            let C1 = ((d1 + (d1.pow(2) - Rational::from(4) * d0.pow(3)).sqrt())
                / Rational::from(2))
            .cbrt();

            let C2 = ((d1 - (d1.pow(2) - Rational::from(4) * d0.pow(3)).sqrt())
                / Rational::from(2))
            .cbrt();

            let x1 = Rational::from(-1) / (Rational::from(3) * a) * (b + C1 + d0 / C1);
            let x2 = Rational::from(-1) / (Rational::from(3) * a) * (b + C2 + d0 / C2);

            vec![x1, x2]
        } */
        _ => todo!("{}th degree polynomials", degree),
    }
}
use solver::solve_univariate_polynomial;

fn print_solutions(input: &str) {
    println!("{}", input);

    let poly = parse_polynomial_expr(input);

    let solns = solve_univariate_polynomial(&poly);
    println!(
        "=> x = {{{}}}",
        solns
            .iter()
            .map(|r| format!("{}", r))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!();
}

fn main() {
    // print_solutions("5x = 0");

    // print_solutions("5x + 3 = 0");

    // print_solutions("x^2 + 5x + 6 = 0");

    // print_solutions("x^2 + 5 = 0");

    // print_solutions("x^2 - 3x - 5x = 0");

    // print_solutions("x^2 - 3x - 5x = x^2 + 2x + 3");

    print_solutions("x^3 + 5x^2 - 25x - 125 = 0"); // (x + 5)^2 * (x - 5)
    print_solutions("-27 + 27 x - 9 x^2 + x^3 = 0"); // (x - 3)^3
    print_solutions("x^4 - 16 x^3 + 96 x^2 - 256 x + 256"); // (x - 4)^4
}
