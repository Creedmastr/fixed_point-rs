use std::ops::Add;

use crate::check::{Checking, Utils};

pub struct Float {
    pub integer: i64,
    pub floating_part: i64,
    pub is_negative: bool,
}

impl Add for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut temp_integer = self.check_int_negatives() + rhs.check_int_negatives();
        let temp_floating_part = self.check_float_negatives() + rhs.check_float_negatives();

        let f_len = self.floating_part.len();
        let tempf_len = temp_floating_part.len();

        if tempf_len.0 != f_len.0 {
            // If the upper multiple of ten is exceeded, you add the first numbers
            // by the difference of length between the original and the new one

        }

        return Float {
            integer: temp_integer,
            floating_part: temp_floating_part,
            is_negative: self.is_negative,
        };

    }
}