use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
    ops::{Add, AddAssign, Div, Mul, Neg, Shl, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Clone)]
struct BigInt {
    sign: Sign,
    digits: Vec<u64>,
}

impl BigInt {
    fn new() -> Self {
        Self {
            sign: Sign::Positive,
            digits: vec![0],
        }
    }

    fn add_positive(&self, other: &BigInt) -> BigInt {
        assert_eq!(self.sign, Sign::Positive);
        assert_eq!(other.sign, Sign::Positive);

        let mut result_digits = Vec::new();
        let mut carry = false;

        let (left, right) = if self > other {
            (self, other)
        } else {
            (other, self)
        };

        for i in 0..left.digits.len() {
            let (result, carry1) =
                left.digits[i].overflowing_add(*right.digits.get(i).unwrap_or(&0));
            let (result, carry2) = result.overflowing_add(if carry { 1 } else { 0 });

            carry = carry1 || carry2;

            result_digits.push(result);
        }

        if carry {
            result_digits.push(1);
        }

        BigInt {
            sign: Sign::Positive,
            digits: result_digits,
        }
    }

    fn sub_positive(&self, other: &BigInt) -> BigInt {
        assert_eq!(self.sign, Sign::Positive);
        assert_eq!(other.sign, Sign::Positive);

        let mut result_digits = Vec::new();
        let mut borrow = false;

        // Make sure we are subtracting the smaller number from the bigger one.
        let sign = if self >= other {
            Sign::Positive
        } else {
            Sign::Negative
        };

        let (left, right) = if sign == Sign::Positive {
            (self, other)
        } else {
            (other, self)
        };

        for i in 0..left.digits.len() {
            let (result, borrow1) =
                left.digits[i].overflowing_sub(*right.digits.get(i).unwrap_or(&0));
            let (result, borrow2) = result.overflowing_sub(if borrow { 1 } else { 0 });

            borrow = borrow1 || borrow2;

            result_digits.push(result);
        }

        BigInt {
            sign,
            digits: result_digits,
        }
    }

    fn mul_positive(&self, other: &BigInt) -> BigInt {
        let mut result = BigInt::new();
        let mut last_carry = 0;

        for (i, &left_digit) in self.digits.iter().enumerate() {
            for (j, &right_digit) in other.digits.iter().enumerate() {
                let result128 = left_digit as u128 * right_digit as u128;
                let result_lower = result128 as u64;
                let carry = (result128 >> 64) as u64;

                result += (BigInt::from(result_lower) + BigInt::from(last_carry)) << (i + j);

                last_carry = carry;
            }
        }

        if last_carry > 0 {
            result.digits.push(last_carry);
        }

        result
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // The idea is to take make all operands positive using the negation operator (not an `abs` function to avoid branching) and then either add or subtract their absolute values.
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Positive) => self.add_positive(&other),
            (Sign::Positive, Sign::Negative) => self.sub_positive(&(-other)),
            (Sign::Negative, Sign::Positive) => other.sub_positive(&(-self)),
            (Sign::Negative, Sign::Negative) => -((-self).add_positive(&(-other))),
        }
    }
}

impl AddAssign for BigInt {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for BigInt {
    type Output = BigInt;

    fn sub(self, other: Self) -> Self::Output {
        // The idea is to take make all operands positive using the negation operator (not an `abs` function to avoid branching) and then either subtract or add their absolute values.
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Positive) => self.sub_positive(&other),
            (Sign::Positive, Sign::Negative) => self.add_positive(&(-other)),
            (Sign::Negative, Sign::Positive) => -((-self).add_positive(&other)),
            (Sign::Negative, Sign::Negative) => -((-self).sub_positive(&(-other))),
        }
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Positive) => self.mul_positive(&other),
            (Sign::Positive, Sign::Negative) => -(self.mul_positive(&(-other))),
            (Sign::Negative, Sign::Positive) => -((-self).mul_positive(&other)),
            (Sign::Negative, Sign::Negative) => (-self).mul_positive(&(-other)),
        }
    }
}

impl Neg for BigInt {
    type Output = Self;

    fn neg(self) -> Self {
        match self.sign {
            Sign::Positive => BigInt {
                sign: Sign::Negative,
                digits: self.digits,
            },
            Sign::Negative => BigInt {
                sign: Sign::Positive,
                digits: self.digits,
            },
        }
    }
}

/// Since `BigInt` is just a collection of base 2^64 digits, we can extend the left shift operation
/// to it. Just like a regular binary left shift, it shifts in 0's from the left (the LSD side).
impl Shl<usize> for BigInt {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        let mut new_digits = [0].repeat(rhs);
        new_digits.extend(self.digits);

        BigInt {
            sign: self.sign,
            digits: new_digits,
        }
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        // Make sure +0 and -0 are equal
        if self.digits.len() == 1
            && other.digits.len() == 1
            && self.digits[0] == 0
            && other.digits[0] == 0
        {
            return true;
        }

        if self.sign != other.sign {
            return false;
        }

        if self.digits.len() == other.digits.len() {
            for (&a, &b) in self.digits.iter().zip(other.digits.iter()) {
                if a != b {
                    return false;
                }
            }

            return true;
        }

        false
    }
}

impl Eq for BigInt {}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            return Ordering::Equal;
        }

        if self.sign == other.sign {
            let ord = if self.digits.len() == other.digits.len() {
                // Compare last digits to determine which one is greater, if any.
                self.digits
                    .last()
                    .unwrap()
                    .cmp(other.digits.last().unwrap())
            } else {
                self.digits.len().cmp(&other.digits.len())
            };

            if self.sign == Sign::Positive {
                ord
            } else {
                ord.reverse()
            }
        } else {
            match (self.sign, other.sign) {
                (Sign::Positive, Sign::Negative) => Ordering::Greater,
                (Sign::Negative, Sign::Positive) => Ordering::Less,
                _ => unreachable!(),
            }
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/*
impl Div for BigInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {}
}
*/

/*
impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.sign == Sign::Negative {
            write!(f, "-")?;
        }


    }
}
*/

impl From<i32> for BigInt {
    fn from(n: i32) -> Self {
        BigInt::from(n as i64)
    }
}

impl From<i64> for BigInt {
    fn from(n: i64) -> Self {
        BigInt {
            sign: if n >= 0 {
                Sign::Positive
            } else {
                Sign::Negative
            },
            digits: vec![n.unsigned_abs()],
        }
    }
}

impl From<u64> for BigInt {
    fn from(n: u64) -> Self {
        BigInt {
            sign: Sign::Positive,
            digits: vec![n],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_simple() {
        assert_eq!(BigInt::from(1) + BigInt::from(2), BigInt::from(3));
    }

    #[test]
    fn addition_overflowing() {
        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![u64::MAX]
            } + BigInt {
                sign: Sign::Positive,
                digits: vec![1]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![0, 1]
            }
        );

        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![u64::MAX]
            } + BigInt {
                sign: Sign::Positive,
                digits: vec![100]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![99, 1]
            }
        );

        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![u64::MAX]
            } + BigInt {
                sign: Sign::Positive,
                digits: vec![u64::MAX]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![u64::MAX - 1, 1]
            }
        );

        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![u64::MAX, 5]
            } + BigInt {
                sign: Sign::Positive,
                digits: vec![u64::MAX, 10]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![u64::MAX - 1, 16]
            }
        );
    }

    #[test]
    fn addition_not_simple() {
        // (+) + (-) = (-)
        assert_eq!(BigInt::from(5) + BigInt::from(-9), BigInt::from(-4));

        // (+) + (-) = (+)
        assert_eq!(BigInt::from(69) + BigInt::from(-9), BigInt::from(60));

        // (-) + (+) = (-)
        assert_eq!(BigInt::from(-10) + BigInt::from(9), BigInt::from(-1));

        // (-) + (+) = (-)
        assert_eq!(BigInt::from(-9) + BigInt::from(10), BigInt::from(1));

        // (-) + (-) = (-)
        assert_eq!(BigInt::from(-10) + BigInt::from(-9), BigInt::from(-19));
    }

    #[test]
    fn addition_different_digits_len() {
        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![0]
            } + BigInt {
                sign: Sign::Positive,
                digits: vec![1, 2]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![1, 2]
            }
        );

        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![1]
            } + BigInt {
                sign: Sign::Positive,
                digits: vec![1, 2]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![2, 2]
            }
        );
    }

    #[test]
    fn subtraction_simple() {
        // (+) - (+) = (+)
        assert_eq!(BigInt::from(5) - BigInt::from(2), BigInt::from(3));

        // (+) - (+) = (-)
        assert_eq!(BigInt::from(1) - BigInt::from(69), BigInt::from(-68));
    }

    #[test]
    fn subtraction_overflowing() {
        // (+) - (+) = (-)
        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![0]
            } - BigInt {
                sign: Sign::Positive,
                digits: vec![1, u64::MAX]
            },
            BigInt {
                sign: Sign::Negative,
                digits: vec![1, u64::MAX]
            }
        );
    }

    #[test]
    fn subtraction_not_simple() {
        // (+) - (-) = (+)
        assert_eq!(BigInt::from(5) - BigInt::from(-69), BigInt::from(74));

        // (-) - (+) = (-)
        assert_eq!(BigInt::from(-69) - BigInt::from(5), BigInt::from(-74));

        // (-) - (-) = (-)
        assert_eq!(BigInt::from(-69) - BigInt::from(-5), BigInt::from(-64));

        // (-) - (-) = (+)
        assert_eq!(BigInt::from(-5) - BigInt::from(-69), BigInt::from(64));
    }

    #[test]
    fn multiplication() {
        /* assert_eq!(BigInt::from(5) * BigInt::from(2), BigInt::from(10));

        assert_eq!(
            BigInt::from(u64::MAX) * BigInt::from(u64::MAX),
            BigInt {
                sign: Sign::Positive,
                digits: vec![1, u64::MAX - 1]
            }
        ); */

        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![0, 5]
            } * BigInt {
                sign: Sign::Positive,
                digits: vec![0, 3]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![0, 0, 15]
            }
        );

        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![1, 5]
            } * BigInt {
                sign: Sign::Positive,
                digits: vec![1, 3]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![1, 8, 15]
            }
        );

        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![69, 78, 420]
            } * BigInt {
                sign: Sign::Positive,
                digits: vec![420, 69, 78]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![28980, 37521, 187164, 35064, 32760]
            }
        );

        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![69, 78, 420]
            } * BigInt {
                sign: Sign::Positive,
                digits: vec![2]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![138, 156, 840]
            }
        );
    }

    #[test]
    fn shl() {
        assert_eq!(
            BigInt::from(1) << 1,
            BigInt {
                sign: Sign::Positive,
                digits: vec![0, 1]
            }
        );

        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![1, 2]
            } << 5,
            BigInt {
                sign: Sign::Positive,
                digits: vec![0, 0, 0, 0, 0, 1, 2]
            }
        );
    }

    #[test]
    fn ordering() {
        assert!(BigInt::from(1) < BigInt::from(2));
        assert!(BigInt::from(1) > BigInt::from(-2));
        assert!(BigInt::from(-10) < BigInt::from(-2));

        assert!(
            BigInt {
                sign: Sign::Negative,
                digits: vec![0]
            } == BigInt {
                sign: Sign::Positive,
                digits: vec![0]
            }
        );
    }
}
