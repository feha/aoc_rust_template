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



// Expands to:

// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }
// 
// 
// #[derive(Debug)]
// pub struct Day01 {}
// 
// impl Day for Day01 {
// 
//     fn part_1(&self, input: &str) -> Result<String, ()> {
//         return Ok(format!("Part {:?}: {:?}", 1, self.part_impl_1(input)));
//     }
// 
//     fn part_2(&self, input: &str) -> Result<String, ()> {
//         return Ok(format!("Part {:?}: {:?}", 2, self.part_impl_2(input)));
//     }
// 
// }
// 
// impl Day01 {
// 
//     fn part_impl_1(&self, input: &str) -> Result<isize, String> {
//         let solution = input
//             .lines()
//             .map(|s| s.parse::<isize>().unwrap())
//             .fold(0, |sum, x| sum + x);
// 
//         return Ok(solution);
//     }
// 
//     fn part_impl_2(&self, input: &str) -> Result<isize, String> {
//         let solution = input
//             .lines()
//             .map(|s| s.parse::<isize>().unwrap())
//             .fold(0, |sum, x| sum + x);
// 
//         return Ok(solution);
//     }
// 
// }
// 
// #[cfg(test)]
// mod tests
// {
//     use super :: * ;
// 
//     fn test_helper_1(s : & str, v : isize) {
//         assert_eq! (Day01 {}.part_impl_1(s).unwrap(), v) ;
//     }
// 
//     #[test]
//     fn test_1() {
//         assert_eq!("", "");
//         test_helper_1("", 0);
//     }
// 
//     fn test_helper_2(s : & str, v : isize)
//     {
//         assert_eq! (Day01 {}.part_impl_2(s).unwrap(), v) ;
//     }
// 
//     #[test]
//     fn test_2() {
//         test_helper_2("", 0);
//         test_helper_2("", 0);
//     }
// }




// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }

// #[derive(Debug)]
// pub struct Day01 {}

// impl Day for Day01
// {
//     fn part_1(& self, input : & str) -> Result<String, ()>
//     {
//         return Ok(format!("Part {:?}: {:?}", 1usize, self.part_impl_1(input)));
//     }
//     fn part_2(& self, input : & str) -> Result<String, ()>
//     {
//         return Ok(format!("Part {:?}: {:?}", 2usize, self.part_impl_2(input)));
//     }
// }

// impl Day01
// {
//     fn part_impl_1(& self, input : & str) -> Result<isize, String>
//     {
//         {
//             let solution =
//             input.lines().map(| s | s.parse :: < isize >
//             ().unwrap()).fold(0, | sum, x | sum + x) ; return Ok(solution) ;
//         }
//     } fn part_impl_2(& self, input : & str) -> Result < isize, String >
//     {
//         {
//             let solution =
//             input.lines().map(| s | s.parse :: < isize >
//             ().unwrap()).fold(0, | sum, x | sum + x) ; return Ok(solution) ;
//         }
//     }
// }
// #[cfg(test)]
// mod tests
// {
//     use super :: * ;
//     fn test_helper_1(s : & str, v : isize)
//     {
//         assert_eq! (Day01 {}.part_impl_1(s).unwrap(), v) ;
//     }

//     #[test]
//     fn test_1()
//     {
//         assert_eq! ("", "") ; test_helper_1("", 0) ;
//     }

//     fn test_helper_2(s : & str, v : isize)
//     {
//         assert_eq! (Day01 {}.part_impl_2(s).unwrap(), v) ;
//     }

//     #[test]
//     fn test_2()
//     {
//         test_helper_2("", 0) ; test_helper_2("", 0) ;
//     }
// }



// Using macro_rules!

// crate::day!{
//     Day01
//     
//     part1
//     |input: &str| -> isize {
//         let solution = input.lines()
//             .map(|s| s.parse::<isize>().unwrap())
//             .fold(0, |sum, x| sum + x );
//         
//         return Ok(solution);
//     }
//     
//     part2
//     |input: &str| -> isize {
//         let solution = input.lines()
//             .map(|s| s.parse::<isize>().unwrap())
//             .fold(0, |sum, x| sum + x );
//         
//         return Ok(solution);
//     }
//     
//     test1
//     ("" , "", false)
//     ("" , 0)
//     
//     test2
//     ("" , 0)
//     ("" , 0)
//     
// }
