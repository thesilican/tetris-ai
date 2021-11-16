#![feature(error_iter)]
use common::misc::*;
use std::{error::Error, num::ParseIntError};

fn main() {
    let res = GenericErr::from("".parse::<i32>().unwrap_err());
    let mut chain = <dyn Error>::chain(&res);
    for x in chain {
        println!("{}", x);
    }
    // println!("{:?}", res.source());
}
