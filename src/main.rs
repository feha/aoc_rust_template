#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.
// #[macro_use] // Allows usage of macros.

use std::env;

use proc_macro_lib;

mod utils; // imports utils.rs (needed as its not in .toml)
use utils::*;

proc_macro_lib::import_days!();


fn main() -> Result<(), ()> {

    let days = proc_macro_lib::instantiate_days!();
    
    let mut args = env::args();
    let _binary_path = args.next();
    let arg0 = args.next();
    let all = arg0.is_some() && arg0.clone().unwrap() == "-a";
    
    
    for (i,day) in days.iter().enumerate().rev() {
        let i = i+1;
        let target = arg0.clone();
        if !all && target.is_some() && target.unwrap() != (i).to_string() {
            continue; // Skip days not asked for
        }
        
        let input = get_input(i);
        let input = input.trim();
        println!("= {} =", i);
        println!("{}", day.part_1(input)?);
        println!("{}", day.part_2(input)?);
        
        if !all {
            break; // only run specified day
        } 
    }
    
    return Ok(());
    //Err(1);
}


// fn add(a : i32, b : i32) -> i32 {
//   return a + b;
// }

// #[cfg(test)] // Only compiled with 'cargo'test' ('cargo bench' can't find it)
// #[test] // This function is a unit-test.
// fn hello_test() {
    //   assert_eq!(main(), Ok(()));
// }

// #[test]
// fn hello_test_add() {
    //   assert_eq!(add(1,2),3);
// }

// #[test]
// #[ignore] // ignore this test; doesn't run it, but still list (as ignored)
// fn hello_test_add_false() {
    //   assert_ne!(add(1,2),3);
// }