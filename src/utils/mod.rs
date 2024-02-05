pub mod aegean_numbers {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum AegeanError {
        #[error("Value `{0}` is too high (max: 99999)")]
        ValueTooHigh(u32),
        #[error("Zero not allowed in Aegean")]
        Zero,
        #[error("String `{0}` is not convertable to Aegean")]
        UnparsableStr(String),
    }

    #[derive(Debug)]
    pub struct UAegean(u32);

    const ONES: [char; 9] = ['𐄇', '𐄈', '𐄉', '𐄊', '𐄋', '𐄌', '𐄍', '𐄎', '𐄏'];
    const TENS: [char; 9] = ['𐄐', '𐄑', '𐄒', '𐄓', '𐄔', '𐄕', '𐄖', '𐄗', '𐄘'];
    const HUNDREDS: [char; 9] = ['𐄙', '𐄚', '𐄛', '𐄜', '𐄝', '𐄞', '𐄟', '𐄠', '𐄡'];
    const THOUSENDS: [char; 9] = ['𐄢', '𐄣', '𐄤', '𐄥', '𐄦', '𐄧', '𐄨', '𐄩', '𐄪'];
    const TENTHOUSENDS: [char; 9] = ['𐄫', '𐄬', '𐄭', '𐄮', '𐄯', '𐄰', '𐄱', '𐄲', '𐄳'];

    impl UAegean {
        pub fn new(value: u32) -> Result<Self, AegeanError> {
            if value >= 100_000 {
                Err(AegeanError::ValueTooHigh(value))
            } else if value == 0 {
                Err(AegeanError::Zero)
            } else {
                Ok(Self(value))
            }
        }

        pub fn into_aegean(self) -> String {
            let val = self.0;
            fn digits(mut num: u32) -> impl Iterator<Item = u32> {
                let mut divisor = 1;
                while num >= divisor * 10 {
                    divisor *= 10;
                }

                std::iter::from_fn(move || {
                    if divisor == 0 {
                        None
                    } else {
                        let v = num / divisor;
                        num %= divisor;
                        divisor /= 10;
                        Some(v)
                    }
                })
            }

            let mut v: Vec<u32> = digits(val).collect();
            let mut c = 1;
            let mut output = String::new();
            while let Some(digit) = v.pop() {
                let series = match c {
                    1 => ONES,
                    2 => TENS,
                    3 => HUNDREDS,
                    4 => THOUSENDS,
                    5 => TENTHOUSENDS,
                    _ => unreachable!(),
                };

                output.push(series[digit as usize - 1]);
                c += 1;
            }

            output.chars().rev().collect()
        }
    }

    impl TryFrom<u32> for UAegean {
        type Error = AegeanError;

        fn try_from(value: u32) -> Result<Self, Self::Error> {
            UAegean::new(value)
        }
    }

    impl TryFrom<&str> for UAegean {
        type Error = AegeanError;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let value = match value.parse::<u32>() {
                Ok(v) => v,
                Err(_) => return Err(AegeanError::UnparsableStr(value.into())),
            };
            Self::new(value)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn validation() {
            let input: u32 = 0;
            let au = UAegean::new(input);
            assert!(au.is_err());
            let input: u32 = 100_000;
            let au = UAegean::new(input);
            assert!(au.is_err());
            let input: u32 = 99_999;
            let au = UAegean::new(input);
            assert!(au.is_ok());
        }
        #[test]
        fn conversion() {
            let input: u32 = 98_765;
            let expected = "𐄳𐄩𐄟𐄕𐄋";
            let au = UAegean::new(input).unwrap();
            let output = au.into_aegean();
            assert_eq!(&output, expected);

            let input: u32 = 1;
            let expected = "𐄇";
            let au = UAegean::new(input).unwrap();
            let output = au.into_aegean();
            assert_eq!(&output, expected);
        }
    }
}
