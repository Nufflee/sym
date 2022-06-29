use std::{collections::HashMap, fmt::Display};

use crate::rational::Rational;

#[derive(Debug)]
pub struct Polynomial {
    coeffs: HashMap<u32, Rational>,
    degree: u32,
}

impl Polynomial {
    pub fn new(coeffs: HashMap<u32, Rational>) -> Self {
        if coeffs.is_empty() {
            panic!("polynomial must have at least 1 coefficient")
        }

        Polynomial {
            degree: *coeffs.keys().max().unwrap(),
            coeffs,
        }
    }

    /// Get the coefficient associated with the `degree`-th term.
    pub fn get(&self, degree: u32) -> Rational {
        *self.coeffs.get(&degree).unwrap_or(&Rational::from(0))
    }

    /// Evaluate the polynomial at a given value `x`.
    pub fn eval(&self, x: Rational) -> Rational {
        let mut result = Rational::from(0);

        for degree in 0..=self.degree() {
            let coeff = self.get(degree as u32);

            result += coeff * x.pow(degree as u32);
        }

        result
    }

    /// Get the first derivative (wrt. `x`) of the polynomial.
    pub fn diff(&self) -> Polynomial {
        let mut diff_coeffs = HashMap::new();

        for (&degree, &coeff) in &self.coeffs {
            // Ignore the 0-th order term as it will be 0
            if degree > 0 {
                diff_coeffs.insert(degree - 1, coeff * Rational::from(degree));
            }
        }

        Polynomial::new(diff_coeffs)
    }

    /// Get the degree of the polynomial.
    pub fn degree(&self) -> u32 {
        self.degree
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut exponents = self.coeffs.keys().collect::<Vec<_>>();

        // Sort the exponents in descending order
        exponents.sort_unstable();
        exponents.reverse();

        for (i, &exponent) in exponents.into_iter().enumerate() {
            let coeff = self.coeffs[&exponent];

            if coeff == Rational::from(0) {
                continue;
            }

            if i != 0 {
                if coeff > Rational::from(0) {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                }
            }

            if coeff != Rational::from(1) {
                write!(f, "{}", coeff.abs())?;
            }

            if exponent != 0 {
                write!(f, "x")?;
            }

            if exponent > 1 {
                write!(f, "^{}", exponent)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degree() {
        assert_eq!(
            Polynomial::new(HashMap::from([
                (0, Rational::from(1)),
                (1, Rational::from(2)),
                (2, Rational::from(3)),
            ]))
            .degree(),
            2
        );

        assert_eq!(
            Polynomial::new(HashMap::from([(4, Rational::from(5)),])).degree(),
            4
        );
    }

    #[test]
    fn eval() {
        assert_eq!(
            Polynomial::new(HashMap::from([
                (0, Rational::from(1)),
                (1, Rational::from(2)),
                (2, Rational::from(3)),
            ]))
            .eval(Rational::from(2)),
            Rational::from(17)
        );

        assert_eq!(
            Polynomial::new(HashMap::from([
                (0, Rational::from(1)),
                (1, Rational::from(2)),
                (4, Rational::from(5)),
            ]))
            .eval(Rational::from(0)),
            Rational::from(1)
        );
    }

    #[test]
    fn diff() {
        assert_eq!(
            Polynomial::new(HashMap::from([
                (0, Rational::from(1)),
                (1, Rational::from(2)),
                (2, Rational::from(3)),
            ]))
            .diff(),
            Polynomial::new(HashMap::from([
                (0, Rational::from(2)),
                (1, Rational::from(6)),
            ]))
        );

        assert_eq!(
            Polynomial::new(HashMap::from([
                (0, Rational::from(1)),
                (2, Rational::from(-5)),
                (3, Rational::from(69)),
            ]))
            .diff(),
            Polynomial::new(HashMap::from([
                (1, Rational::from(-5 * 2)),
                (2, Rational::from(69 * 3)),
            ]))
        );
    }
}
