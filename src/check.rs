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
    fn len(&self) -> u16;
}

impl Utils for i64 {
    fn len(&self) -> u16 {
        let mut count: u16 = 0;
        for ch in &self.abs().to_string().chars().collect::<Vec<char>>() {
            count += 1;
        }

        return count;
    }
}