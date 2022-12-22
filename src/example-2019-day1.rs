#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs


pub struct Day01 {}

impl Day for Day01 {
  fn part1(&self, input: &str) {
    println!("part1: {:?}", self.part1_impl(input) );
  }

  fn part2(&self, input: &str) {
    println!("part2: {:?}", self.part2_impl(input) );
  }
}

impl Day01 {
  fn part1_impl(&self, input: &str) -> Result<isize, String> {
    let masses = input.lines().map(|s| s.parse::<isize>());
    let fuels = masses.map(|mass| {
      let m = mass.unwrap();
      return get_fuel(&m);
    });
    let solution = fuels.fold(0, |sum, x| sum + x );
    return Ok(solution);
  }

  fn part2_impl(&self, input: &str) -> Result<isize, String> {
    let masses = input.lines().map(|s| s.parse::<isize>());
    let fuels = masses.map(|mass| {
      let m = mass.unwrap();
      return get_corrected_fuel(&m);
    });
    let solution = fuels.fold(0, |sum, x| sum + x );
    return Ok(solution);
  }
}


fn main() -> Result<(), isize> {
  println!("Hello, day1!");
  
  // let contents = get_contents(FILENAME);
  let compiler_error = get_input(1);
  let input = compiler_error.trim();
  
  let masses = input.lines().map(|s| s.parse::<isize>());
  let fuels = masses.map(|mass| {
    let m = mass.unwrap();
    return (get_fuel(&m), get_corrected_fuel(&m));
  });
  let fuel = fuels.fold((0,0), |sum, x| (sum.0 + x.0, sum.1 + x.1) );
  
  println!("part1: {}\npart2: {}", fuel.0, fuel.1);
  
  return Ok(());
  //Err(1);
}


fn get_fuel(mass : &isize) -> isize {
  return mass / 3 - 2;
}

fn get_corrected_fuel(mass : &isize) -> isize {
  let mut fuel = 0;
  let mut more_fuel = get_fuel(mass);
  while more_fuel > 0 {
    fuel = fuel + more_fuel;
    more_fuel = get_fuel(&more_fuel);
  }
  
  return fuel;
}



#[cfg(test)]
mod tests {
  use super::*;

  fn test1(s: &str, v: isize) {
    assert_eq!(Day01 {}.part1_impl(s).unwrap(), v);
  }

  #[test]
  fn part1() {
    test1("12", 2);
    test1("14", 2);
    test1("1969", 654);
    test1("100756", 33583);
  }

  fn test2(s: &str, v: isize) {
    assert_eq!(Day01 {}.part2_impl(s).unwrap(), v);
  }

  #[test]
  fn part2() {
    test2("1969", 966);
    test2("100756", 50346);
  }
}