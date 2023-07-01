use std::{ops::{Add, Sub, Mul, Div}};

pub struct fi {
    pub value: u32,
    pub fixed_point: u8,
}

impl fi {
    fn len(&self) -> u8 {
        return self.value.to_string().chars().count() as u8;      
    }

    pub fn from<T: TryInto<u32>>(rhs: T) -> Self {
        return fi {
            value: rhs.try_into().unwrap_or(0),
            fixed_point: 12
        };
    }

    pub fn fixed_from<T: TryInto<u32>>(rhs: T, fixed_point: u8) -> Self {
        return fi {
            value: rhs.try_into().unwrap_or(0),
            fixed_point: fixed_point,
        };
    }

    pub fn fmt(&self) -> String {
        match self.fixed_point > self.len() {
            true => {
                return format!("0.{}", self.value);
            }

            false => {
                let binding = self.value.to_string();
                let splitted_value = &binding.split_at((self.fixed_point) as usize);
                let stringed_fmt = format!("{}.{}", splitted_value.0, splitted_value.1);

                return stringed_fmt;
            }
        }
    }
}

impl Add for fi {
    type Output = Self;

    fn add(self, rhs: fi) -> Self::Output {
        return fi {
            value: self.value + rhs.value,
            fixed_point: self.fixed_point
        }
    }
}

impl Sub for fi {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return fi {
            value: self.value - rhs.value,
            fixed_point: self.fixed_point
        }
    }
}

impl Mul for fi {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return fi {
            value: self.value * rhs.value,
            fixed_point: self.fixed_point,
        }
    }
}

impl Div for fi {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        return fi {
            value: self.value / rhs.value,
            fixed_point: self.fixed_point
        }
    }
}