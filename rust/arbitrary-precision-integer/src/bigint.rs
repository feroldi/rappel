use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
struct BigInt {
    high: u64,
    low: u64,
}

impl From<u64> for BigInt {
    fn from(value: u64) -> BigInt {
        BigInt {
            high: 0,
            low: value,
        }
    }
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> BigInt {
        let (low_add, has_overflowed) = self.low.overflowing_add(other.low);
        let high_add = self.high + other.high + has_overflowed as u64;
        BigInt {
            high: high_add,
            low: low_add,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BigInt;

    #[test]
    fn keep_result_in_low_part_when_addition_fits_64_bit() {
        let a = BigInt {
            high: 0,
            low: u64::MAX - 1,
        };
        let b = BigInt { high: 0, low: 1u64 };

        let res = a + b;

        assert_eq!(
            res,
            BigInt {
                high: 0u64,
                low: u64::MAX,
            }
        );
    }

    #[test]
    fn carry_out_to_high_when_low_overflows_in_addition() {
        let a = BigInt {
            high: 0,
            low: u64::MAX,
        };
        let b = BigInt { high: 0, low: 1u64 };

        let res = a + b;

        assert_eq!(
            res,
            BigInt {
                high: 1u64,
                low: 0u64
            }
        );
    }

    #[test]
    fn carrying_out_is_commutative_in_addition() {
        let a = BigInt {
            high: 0,
            low: u64::MAX,
        };
        let b = BigInt { high: 0, low: 1u64 };

        let a_plus_b = a + b;
        let b_plus_a = b + a;

        assert_eq!(a_plus_b, b_plus_a);
    }
}
