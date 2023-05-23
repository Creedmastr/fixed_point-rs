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

        if temp_floating_part.len() != self.floating_part.len() {
            todo!();

            // TODO: if the last character change, do nothing
            // If the first character changes, then you start comparing all the characters, 
            // and add to the temp_integer variable consequentely
        }

        return Float {
            integer: temp_integer,
            floating_part: temp_floating_part,
            is_negative: self.is_negative,
        };

    }
}