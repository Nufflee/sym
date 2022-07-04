use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
    ops::{Add, Div, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct BigInt {
    sign: Sign,
    digits: Vec<u64>,
}

impl BigInt {}

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Positive) => {
                let mut result_digits = Vec::new();
                let mut carry = false;

                for i in 0..self.digits.len() {
                    let (result, carry1) = self.digits[i].overflowing_add(other.digits[i]);
                    let (result, carry2) = result.overflowing_add(if carry { 1 } else { 0 });

                    carry = carry1 || carry2;

                    result_digits.push(result);
                }

                if carry {
                    result_digits.push(1);
                }

                BigInt {
                    sign: self.sign,
                    digits: result_digits,
                }
            }
            (Sign::Positive, Sign::Negative) => self - -other,
            (Sign::Negative, Sign::Positive) => other - -self,
            (Sign::Negative, Sign::Negative) => -(-self + -other),
        }
    }
}

impl Sub for BigInt {
    type Output = BigInt;

    fn sub(self, other: Self) -> Self::Output {
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Positive) => {
                let mut result_digits = Vec::new();
                let mut borrow = false;

                let left = if self > other { &self } else { &other };
                let right = if self > other { &other } else { &self };

                for i in 0..left.digits.len() {
                    let (result, borrow1) =
                        left.digits[i].overflowing_sub(*right.digits.get(i).unwrap_or(&0));
                    let (result, borrow2) = result.overflowing_sub(if borrow { 1 } else { 0 });

                    borrow = borrow1 || borrow2;

                    result_digits.push(result);
                }

                let sign = if self < other {
                    Sign::Negative
                } else {
                    Sign::Positive
                };

                BigInt {
                    sign,
                    digits: result_digits,
                }
            }
            (Sign::Positive, Sign::Negative) => self + -other, // `other` is negated in order to make it positive
            (Sign::Negative, Sign::Positive) => -(-self + other), // `self` is negated in order to make it positive
            (Sign::Negative, Sign::Negative) => -(-self - -other),
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

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.sign == other.sign {
            if self.digits.len() == other.digits.len() {
                if self.sign == Sign::Positive {
                    self.digits
                        .last()
                        .unwrap()
                        .cmp(other.digits.last().unwrap())
                } else {
                    match self
                        .digits
                        .last()
                        .unwrap()
                        .cmp(other.digits.last().unwrap())
                    {
                        Ordering::Less => Ordering::Greater,
                        Ordering::Equal => Ordering::Equal,
                        Ordering::Greater => Ordering::Less,
                    }
                }
            } else {
                self.digits.len().cmp(&other.digits.len())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_simple() {
        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![1]
            } + BigInt {
                sign: Sign::Positive,
                digits: vec![2]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![3]
            }
        );
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
        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![5]
            } + BigInt {
                sign: Sign::Negative,
                digits: vec![9]
            },
            BigInt {
                sign: Sign::Negative,
                digits: vec![4]
            }
        );

        // (+) + (-) = (+)
        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![69]
            } + BigInt {
                sign: Sign::Negative,
                digits: vec![9]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![60]
            }
        );

        // (-) + (+) = (-)
        assert_eq!(
            BigInt {
                sign: Sign::Negative,
                digits: vec![10]
            } + BigInt {
                sign: Sign::Positive,
                digits: vec![9]
            },
            BigInt {
                sign: Sign::Negative,
                digits: vec![1]
            }
        );

        // (-) + (+) = (-)
        assert_eq!(
            BigInt {
                sign: Sign::Negative,
                digits: vec![9]
            } + BigInt {
                sign: Sign::Positive,
                digits: vec![10]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![1]
            }
        );

        // (-) + (-) = (-)
        assert_eq!(
            BigInt {
                sign: Sign::Negative,
                digits: vec![10]
            } + BigInt {
                sign: Sign::Negative,
                digits: vec![9]
            },
            BigInt {
                sign: Sign::Negative,
                digits: vec![19]
            }
        );
    }

    #[test]
    fn subtraction_simple() {
        // (+) - (+) = (+)
        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![5]
            } - BigInt {
                sign: Sign::Positive,
                digits: vec![2]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![3]
            }
        );

        // (+) - (+) = (-)
        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![1]
            } - BigInt {
                sign: Sign::Positive,
                digits: vec![69]
            },
            BigInt {
                sign: Sign::Negative,
                digits: vec![68]
            }
        );
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
                digits: vec![u64::MAX]
            },
            BigInt {
                sign: Sign::Negative,
                digits: vec![u64::MAX]
            }
        );

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
        assert_eq!(
            BigInt {
                sign: Sign::Positive,
                digits: vec![5]
            } - BigInt {
                sign: Sign::Negative,
                digits: vec![69]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![74]
            }
        );

        // (-) - (+) = (-)
        assert_eq!(
            BigInt {
                sign: Sign::Negative,
                digits: vec![69]
            } - BigInt {
                sign: Sign::Positive,
                digits: vec![5]
            },
            BigInt {
                sign: Sign::Negative,
                digits: vec![74]
            }
        );

        // (-) - (-) = (-)
        assert_eq!(
            BigInt {
                sign: Sign::Negative,
                digits: vec![69]
            } - BigInt {
                sign: Sign::Negative,
                digits: vec![5]
            },
            BigInt {
                sign: Sign::Negative,
                digits: vec![64]
            }
        );

        // (-) - (-) = (+)
        assert_eq!(
            BigInt {
                sign: Sign::Negative,
                digits: vec![5]
            } - BigInt {
                sign: Sign::Negative,
                digits: vec![69]
            },
            BigInt {
                sign: Sign::Positive,
                digits: vec![64]
            }
        );
    }
}
