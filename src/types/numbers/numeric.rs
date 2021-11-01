use std::iter::FromIterator;
use std::ops::{Add, Deref};
use std::os::raw::c_char;
use std::string::FromUtf8Error;

pub type NumericDigit = c_char;

pub const NUMERIC_MAX_PRECISION: i16 = 1000;
pub const NUMERIC_MAX_SCALE: i16 = 1000;
pub const NUMERIC_MIN_SCALE: i16 = -1000;

///'-' char
const MINUS_CHAR: u8 = 45;
///'0' char
const ZERO_CHAR: u8 = 48;
/// '.' char
const DOT_CHAR: u8 = 46;

#[derive(Debug, Default)]
pub struct Numeric {
    digits: Vec<NumericDigit>,
    prec: i16,
    scale: i16,
    negative: bool,
}

impl Numeric {
    pub fn from_str(val: &str) -> Self {
        if val.len() == 0 {
            return Numeric::default();
        }
        let mut digit: NumericDigit = 0;
        let mut digits: Vec<NumericDigit> = vec![];
        let bytes = val.as_bytes();

        let mut was_factor: bool = false;
        let mut prec: i16 = 0;

        'byte_cycle:
        for byte in bytes {
            if *byte == MINUS_CHAR {
                continue;
            }
            if *byte == DOT_CHAR {
                was_factor = true;
                continue;
            }

            if !byte.is_ascii_digit() {
                break 'byte_cycle;
            }

            digit = (byte - 48) as NumericDigit;

            if !was_factor {
                prec += 1;
            }

            digits.push(digit)
        }

        let scale = (digits.len() - prec as usize) as i16;

        Numeric {
            digits,
            prec,
            scale,
            negative: bytes[0] == MINUS_CHAR,
        }
    }

    pub fn from_i32(val: i32) -> Self {
        Self::from_str(val.to_string().as_str())
    }

    pub fn from_i64(val: i64) -> Self {
        Self::from_str(val.to_string().as_str())
    }

    pub fn from_i128(val: i128) -> Self {
        Self::from_str(val.to_string().as_str())
    }
}

impl From<String> for Numeric {
    fn from(s: String) -> Self {
        Self::from_str(s.as_str())
    }
}

impl From<&str> for Numeric {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<i32> for Numeric {
    fn from(s: i32) -> Self {
        Self::from_i32(s)
    }
}

impl From<i64> for Numeric {
    fn from(s: i64) -> Self {
        Self::from_i64(s)
    }
}

impl From<i128> for Numeric {
    fn from(s: i128) -> Self {
        Self::from_i128(s)
    }
}

impl IntoIterator for Numeric {
    type Item = NumericDigit;
    type IntoIter = std::vec::IntoIter<NumericDigit>;

    fn into_iter(self) -> Self::IntoIter {
        return self.digits.into_iter();
    }
}

impl Add for Numeric {
    type Output = Numeric;

    fn add(self, rhs: Self) -> Self::Output {
        // get less and greater vecs
        let (g_digits, l_digits) = {
            if self.digits.len() >= rhs.digits.len() {
                (&self.digits, &rhs.digits)
            } else {
                (&rhs.digits, &self.digits)
            }
        };

        let mut i = 0;
        let mut rem : bool = false;
        let mut sum : u8;
        let mut res : String= String::new();

        'sum_cycle:
        for g_digit in g_digits.into_iter().rev() {
            let ii: i32
                = l_digits.len() as i32 - 1 - i;
            let maybe_l_digit
                = l_digits.get(ii as usize);

            if maybe_l_digit.is_some() {

                let l_digit = maybe_l_digit.unwrap();
                let rem_u8 : u8 = if rem {1} else {0};
                sum = (*g_digit + *l_digit) as u8 + rem_u8;
                rem = sum > 9;
                sum = {if rem {sum - 10} else {sum}};
            } else {
                let rem_u8 :u8 = if rem {1} else {0};
                sum = (*g_digit as u8 + rem_u8);
                rem = false;
            }

            res = res.to_owned().add(sum.to_string().as_str());

            i+=1;
        }

        res = res.chars().rev().collect();
        return Numeric::from_str(res.as_str());
    }
}

#[test]
fn parse_str_test() {
    let numeric = Numeric::from("-1.00004");
    println!("{:?}", numeric)
}

#[test]
fn parse_string_test() {
    let numeric = Numeric::from("3434z6");
    println!("{:?}", numeric);
    let numeric = Numeric::from("3434.6");
    println!("{:?}", numeric);
    let numeric = Numeric::from("-3434.6");
    println!("{:?}", numeric)
}

#[test]
fn numeric_precision_scale_test() {
    let numeric = Numeric::from("54.00006");
    assert_eq!(numeric.prec, 2);
    assert_eq!(numeric.scale, 5);
}

#[test]
fn numeric_iterator_test() {
    let numeric = Numeric::from("3434.6");

    'a: for i in numeric {
        println!("{:}", i);
    }
}

#[test]
fn numeric_parse_from_string_test() {
    let s = String::from("123123");
    let numeric = Numeric::from(s);
    println!("{:?}", numeric);
}



#[test]
fn numeric_adding_test() {
    // 123 + 9 = 132
    // 123 + 29 = 152
    // 91 + 5001 = 5092
    let numeric = Numeric::from(5001);
    let numeric2 = Numeric::from(91);
    let res = numeric + numeric2;
    println!("{:?}", res);
}
