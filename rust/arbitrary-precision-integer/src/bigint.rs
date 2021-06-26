use std::ops::Add;

#[derive(Debug, PartialEq)]
struct BigInt {
    raw: u64,
}

impl From<i64> for BigInt {
    fn from(value: i64) -> BigInt {
        BigInt { raw: value as u64 }
    }
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> BigInt {
        BigInt {
            raw: self.raw + other.raw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BigInt;

    #[test]
    fn add_two_bigints() {
        let a = BigInt::from(2);
        let b = BigInt::from(3);

        let res = a + b;

        assert_eq!(res, BigInt::from(5));
    }
}
