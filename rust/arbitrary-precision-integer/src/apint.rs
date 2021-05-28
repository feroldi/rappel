struct APInt {
    value: APIntValue,
}

#[derive(Debug, PartialEq)]
enum APIntValue {
    Scalar(i128),
}

impl From<i32> for APInt {
    fn from(value: i32) -> Self {
        APInt {
            value: APIntValue::Scalar(value.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{APInt, APIntValue};

    #[test]
    fn create_apint_from_i32() {
        let apint = APInt::from(42i32);
        assert_eq!(apint.value, APIntValue::Scalar(42i128));
    }
}
