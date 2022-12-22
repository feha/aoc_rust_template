#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

use proc_macro_lib;

proc_macro_lib::impl_day!(

part 1
(input: &str) -> &str {
    let solution = "";
    
    return Ok(solution);
}

part 2
(input: &str) -> &str {
    let solution = "";
    
    return Ok(solution);
}

test 1
assert("" , "")
("" , "")

test 2
("" , "")
("" , "")

);