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

    const MAX: u64 = u64::MAX;

    #[test]
    fn keep_result_in_low_part_when_addition_fits_64_bit() {
        let a = BigInt {
            high: 0,
            low: MAX - 1,
        };
        let b = BigInt { high: 0, low: 1 };

        let res = a + b;

        assert_eq!(res, BigInt { high: 0, low: MAX });
    }

    #[test]
    fn carry_out_to_high_when_low_overflows_in_addition() {
        let a = BigInt { high: 3, low: MAX };
        let b = BigInt { high: 0, low: 1 };

        let res = a + b;

        assert_eq!(res, BigInt { high: 4, low: 0 });
    }

    #[test]
    fn wrap_around_low_addition_when_overflowed() {
        let a = BigInt {
            high: 0,
            low: MAX - 3,
        };
        let b = BigInt { high: 0, low: 42 };

        let res = a + b;

        assert_eq!(res, BigInt { high: 1, low: 38 });
    }

    #[test]
    fn carrying_out_is_commutative_in_addition() {
        let a = BigInt { high: 3, low: MAX };
        let b = BigInt { high: 5, low: 1 };

        let a_plus_b = a + b;
        let b_plus_a = b + a;

        assert_eq!(a_plus_b, b_plus_a);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn panic_when_adding_high_parts_overflows() {
        let a = BigInt { high: MAX, low: 0 };
        let b = BigInt { high: 1, low: 0 };

        let _ = a + b;
    }
}
