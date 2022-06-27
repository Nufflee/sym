use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

fn integer_sqrt(value: i32) -> Option<i32> {
    if value < 0 {
        todo!("integer_sqrt: negative roots");
    }

    // Use binary search to find the integer square root. Adapted from https://en.wikipedia.org/wiki/Integer_square_root#Algorithm_using_binary_search.
    let mut low = 0;
    let mut mid;
    let mut high = value + 1;

    while low != high - 1 {
        mid = (low + high) / 2;

        if mid * mid <= value {
            low = mid;
        } else {
            high = mid;
        }
    }

    if low * low == value {
        Some(low)
    } else {
        None
    }
}

fn integer_cbrt(value: i32) -> Option<i32> {
    // Cube root is an odd function meaning that cbrt(-a) = -cbrt(a). So, in order to compute cbrt(-a) we compute cbrt(a) and tack a minus on at the end.
    let value_abs = value.abs();

    // Use binary search to find the integer cube root. Adapted from https://en.wikipedia.org/wiki/Integer_square_root#Algorithm_using_binary_search.
    let mut low = 0;
    let mut mid;
    let mut high = value_abs + 1;

    while low != high - 1 {
        mid = (low + high) / 2;

        if mid.pow(3) <= value_abs {
            low = mid;
        } else {
            high = mid;
        }
    }

    if low.pow(3) == value_abs {
        Some(if value < 0 { -low } else { low })
    } else {
        None
    }
}

fn greatest_common_divisor(mut a: i32, mut b: i32) -> i32 {
    // Use Euclidean algorithm to find the GCD (https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm)
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

#[derive(Clone, Copy, Debug)]
pub struct Rational {
    numer: i32,
    denom: i32,
}

impl Rational {
    pub fn new(mut numer: i32, mut denom: i32) -> Rational {
        let gcd = greatest_common_divisor(numer, denom).abs();

        if denom == 0 {
            panic!("denominator cannot be zero.");
        }

        // Make sure the sign is always kept in the numerator.
        if denom < 0 {
            numer = -numer;
            denom = -denom;
        }

        Rational {
            numer: numer / gcd,
            denom: denom / gcd,
        }
    }

    pub fn reciprocal(&self) -> Self {
        Rational {
            numer: self.denom,
            denom: self.numer,
        }
    }

    pub fn sqrt(&self) -> Rational {
        Rational {
            numer: integer_sqrt(self.numer)
                .expect("todo: irrational square roots not supported yet"),
            denom: integer_sqrt(self.denom)
                .expect("todo: irrational square roots not supported yet"),
        }
    }

    pub fn cbrt(&self) -> Rational {
        Rational {
            numer: integer_cbrt(self.numer).expect("todo: irrational cube roots not supported yet"),
            denom: integer_cbrt(self.denom).expect("todo: irrational cube roots not supported yet"),
        }
    }

    pub fn pow(&self, exponent: u32) -> Self {
        Rational {
            numer: self.numer.pow(exponent as u32),
            denom: self.denom.pow(exponent as u32),
        }
    }

    pub fn reduce(&self) -> Rational {
        let gcd = greatest_common_divisor(self.numer, self.denom);

        Rational {
            numer: self.numer / gcd,
            denom: self.denom / gcd,
        }
    }

    pub fn as_integer(&self) -> Option<i32> {
        if self.denom == 1 {
            Some(self.numer)
        } else {
            None
        }
    }
}

impl From<i32> for Rational {
    fn from(x: i32) -> Self {
        Rational { numer: x, denom: 1 }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Rational::new(
            self.numer * other.denom + self.denom * other.numer,
            self.denom * other.denom,
        )
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Self {
        Rational::new(-self.numer, self.denom)
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self {
        Rational::new(self.numer * rhs.numer, self.denom * rhs.denom)
    }
}

impl Div for Rational {
    type Output = Rational;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Rational) -> Self {
        self * rhs.reciprocal()
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        let self_reduced = self.reduce();
        let other_reduced = other.reduce();

        self_reduced.numer == other_reduced.numer && self_reduced.denom == other_reduced.denom
    }
}

impl Eq for Rational {}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        let ad = self.numer * other.denom;
        let bc = self.denom * other.numer;

        ad.cmp(&bc)
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.numer)
        } else {
            write!(f, "{}/{}", self.numer, self.denom)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rationals_are_canonicalized() {
        assert_eq!(Rational::new(16, 4), Rational::from(4));
        assert_eq!(Rational::new(8, -2), Rational::from(-4));
        assert_eq!(Rational::new(-32, 8), Rational::from(-4));
        assert_eq!(Rational::new(-8, -3), Rational::new(8, 3));
    }

    #[test]
    fn sqrt() {
        assert_eq!(Rational::new(16, 1).sqrt(), Rational::new(4, 1));
        assert_eq!(Rational::new(1, 4).sqrt(), Rational::new(1, 2));
        assert_eq!(Rational::new(16, 4).sqrt(), Rational::from(2));
    }

    #[test]
    fn cbrt() {
        assert_eq!(Rational::new(8, 1).cbrt(), Rational::new(2, 1));
        assert_eq!(Rational::new(27, 8).cbrt(), Rational::new(3, 2));
        assert_eq!(Rational::new(-27, 8).cbrt(), Rational::new(-3, 2));
        assert_eq!(Rational::new(1, 1).cbrt(), Rational::new(1, 1));
        assert_eq!(Rational::new(0, 1).cbrt(), Rational::new(0, 1));
    }

    #[test]
    fn addition() {
        assert_eq!(Rational::new(1, 2) + Rational::new(1, 2), Rational::from(1));
        assert_eq!(
            Rational::new(1, 2) + Rational::new(1, 3),
            Rational::new(5, 6)
        );
        assert_eq!(
            Rational::new(1, 2) + Rational::new(-1, 3),
            Rational::new(1, 6)
        );
        assert_eq!(
            Rational::new(1, 2) + Rational::from(5),
            Rational::new(11, 2)
        )
    }

    #[test]
    fn subtraction() {
        assert_eq!(Rational::new(1, 2) - Rational::new(1, 2), Rational::from(0));
        assert_eq!(
            Rational::new(1, 2) - Rational::new(1, 3),
            Rational::new(1, 6)
        );
        assert_eq!(
            Rational::new(1, 2) - Rational::new(-1, 3),
            Rational::new(5, 6)
        );
    }

    #[test]
    fn multiplication() {
        assert_eq!(
            Rational::new(1, 2) * Rational::new(1, 2),
            Rational::new(1, 4)
        );
        assert_eq!(
            Rational::new(1, 2) * Rational::new(1, 3),
            Rational::new(1, 6)
        );
        assert_eq!(
            Rational::new(1, 2) * Rational::new(-1, 3),
            Rational::new(-1, 6)
        );
        assert_eq!(Rational::new(1, 2) * Rational::from(5), Rational::new(5, 2))
    }

    #[test]
    fn division() {
        assert_eq!(Rational::new(1, 2) / Rational::new(1, 2), Rational::from(1));
        assert_eq!(
            Rational::new(1, 2) / Rational::new(1, 3),
            Rational::new(3, 2)
        );
        assert_eq!(
            Rational::new(1, 2) / Rational::new(-1, 3),
            Rational::new(-3, 2)
        );
    }

    #[test]
    fn equality() {
        assert_eq!(Rational::new(1, 2), Rational::new(1, 2));
        assert_eq!(Rational::new(1, 2), Rational::new(2, 4));
        assert_eq!(Rational::new(1, 2), Rational::new(-2, -4));
        assert_ne!(Rational::new(1, 2), Rational::new(-2, 4));
        assert_ne!(Rational::new(1, 2), Rational::new(2, -4));
        assert_ne!(
            Rational { numer: 6, denom: 2 },
            Rational { numer: 7, denom: 2 }
        );
    }

    #[test]
    fn ordering() {
        assert!(Rational::new(1, 4) < Rational::new(1, 2));
        assert!(Rational::new(2, 3) > Rational::new(1, 2));
        assert!(Rational::new(-2, 3) < Rational::new(1, 2));
    }
}
