use std::os::raw::c_char;

type NumericDigit = c_char;

#[derive(Debug, Default)]
struct Numeric {
    digits: Vec<NumericDigit>,
    weight: i32,
    negative: bool
}

impl Numeric {
    pub fn from_str(val: &str) -> Self {
        if val.len() == 0 {
            return Numeric::default()
        }
        let mut digit : NumericDigit = 0;
        let mut digits: Vec<NumericDigit> = vec![];
        let bytes = val.as_bytes();

        'a: for byte in bytes {

            if *byte == 45 { continue; }
            if !byte.is_ascii_digit() { break 'a; }

            digit = (byte - 48) as i8;
            digits.push(digit)
        }

        let weight = digits.len() as i32;

        Numeric { digits, weight, negative: bytes[0] == 45 }
    }

    pub fn from_u32(val : u32) -> Self {
        Self::from_str(val.to_string().as_str())
    }

    pub fn from_u64(val : u64) -> Self {
        Self::from_str(val.to_string().as_str())
    }

    pub fn from_u128(val : u128) -> Self {
        Self::from_str(val.to_string().as_str())
    }
}

impl From<String> for Numeric {
    fn from(s: String) -> Self {
        Self::from_str(s.as_str())
    }
}

impl From<u32> for Numeric {
    fn from(s: u32) -> Self {
        Self::from_u32(s)
    }
}

impl From<u64> for Numeric {
    fn from(s: u64) -> Self {
        Self::from_u64(s)
    }
}

impl From<u128> for Numeric {
    fn from(s: u128) -> Self {
        Self::from_u128(s)
    }
}

#[test]
fn parse_str_test() {
    let numeric = Numeric::from_str("-1");
    println!("{:?}", numeric)
}

#[test]
fn parse_string_test() {
    let numeric = Numeric::from(String::from("3434z6"));
    println!("{:?}", numeric)
}
