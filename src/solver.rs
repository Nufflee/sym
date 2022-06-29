use crate::polynomial::Polynomial;
use crate::rational::Rational;
use std::cmp::Ordering;

pub fn solve_univariate_polynomial(poly: &Polynomial) -> Vec<Rational> {
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
        _ => {
            /* Algorithm:
            let P be the polynomial of degree deg(P)

            if deg(P) >= 3:
                1. normalize P to only have integer coefficients
                2. use rational root theorem to find all possible rational real roots x_i of P
                3. for each x_i that is an actual root, determine its multiplicity using derivatives and store it
                4. if number of rational roots i < deg(P):
                    4.1. use numerical methods to find the remaining (real) roots and store them
                5. end
            */

            let mut roots = Vec::new();

            // Find all the rational roots using the rational root theorem (https://en.wikipedia.org/wiki/Rational_root_theorem)
            // TODO: normalization of non-integer coefficients
            let ps = integer_factors(
                poly.get(0)
                    .as_integer()
                    .expect("todo: normalization of non-integer coefficients"),
            );
            let qs = integer_factors(
                poly.get(poly.degree())
                    .as_integer()
                    .expect("todo: normalization of non-integer coefficients"),
            );

            for &p in &ps {
                for &q in &qs {
                    let potential_root = Rational::new(p, q);

                    // Check if it's an actual root
                    if poly.eval(potential_root) == Rational::from(0) {
                        // If so, determine the multiplicity by counting the number of derivatives that vanish (are 0) at the root
                        let mut test_derivative = poly.diff();
                        let mut multiplicity = 1;

                        while test_derivative.eval(potential_root) == Rational::from(0) {
                            multiplicity += 1;
                            test_derivative = test_derivative.diff();
                        }

                        roots.append(&mut [potential_root].repeat(multiplicity));
                    }
                }
            }

            roots
        }
    }
}

fn integer_factors(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();

    for i in 1..=n.abs() {
        if n % i == 0 {
            if n < 0 {
                factors.push(-i);
            }
            factors.push(i);
        }
    }

    factors
}
