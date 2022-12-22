#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

use proc_macro_lib;

proc_macro_lib::impl_day!(

fn main() -> Result<(), ()> {
    println!("hello day!");
    return Ok(());
}

part 1
(input: &str) -> isize {
    // println!("{}", input);
    let solution = input.lines()
        .map(|s| s.parse::<isize>().unwrap())
        .fold(0, |sum, x| sum + x );
    
    return Ok(solution);
}

part 2
(input: &str) -> isize {
    let solution = input.lines()
        .map(|s| s.parse::<isize>().unwrap())
        .fold(0, |sum, x| sum + x );
    
    return Ok(solution);
}

test 1
assert("" , "")
("" , 0)

test 2
("" , 0)
("" , 0)

);
