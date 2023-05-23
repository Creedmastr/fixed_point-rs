use crate::fpoint::Float;

pub trait Checking {
    fn check_int_negatives(&self) -> i64;

    fn check_float_negatives(&self) -> i64;
}

impl Checking for Float {
    fn check_int_negatives(&self) -> i64 {
        if self.is_negative {
            return -self.integer;
        } else {
            return self.integer;
        }
    }

    fn check_float_negatives(&self) -> i64 {
        if self.is_negative {
            return -self.floating_part;
        } else {
            return self.floating_part;
        }
    }
}

pub trait Utils {
    fn len(&self) -> (usize, char, char);
}

impl Utils for i64 {
    fn len(&self) -> (usize, char, char) {
        let chs = &self.abs().to_string().chars().collect::<Vec<char>>();
        let length = chs.len();
        let last_character = chs[length - 1];
        let first_character = chs[0];

        return (length, last_character, first_character);
    }
}

pub fn upper_multiple_of_10(n: i32) -> u32 {
    return ((n / 10 + 1) * 10) as u32
}
