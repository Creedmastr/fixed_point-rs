#![allow(non_camel_case_types)]

use crate::fpoint::fi;

mod fpoint;

fn main() {
    let nb = fi::from(2092934747);

    println!("{}", nb.fmt());
}