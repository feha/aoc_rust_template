# AoC Rust Template #

A Rust 'template/framework' for AoC to easily add new days.

Uses proc_macro's to remove the need for user to rename anything when starting a new day, with the exception of the filename when copying the prior day / day template-file.

The relevant (actually used) macros are kept in the `proc_macro_aoc` library. But some additional unused macro code is kept in the `proc_macro_lib` library for posterity and the ability to look over in the future.

## Results: ##
    When using this template, you can paste your results here if you want.

## Usage: ##
When starting a year/project with this template, you need to:
 - Edit the `./Cargo.toml` file's project name to something fitting.
 - Change the `YEAR` constant in `./src/main.rs` to whichever year you target.
 - Create a file `./res/.SESSION` and paste your AoC-cookie's session-header into it.
 - ***[Optionally]***, you can also edit the `README.md` to say whatever you wish. Like how this at that point is no longer a template, but rather an implementation fo the template for a specific year of AoC.

---------

When starting a day with this template, you need to:
 - Create a copy of `./src/day.rs` and rename the copy along the lines of `day1.rs`.
 - Implement your answers to the matching days AoC questions in the `part_1` & `part_2` functions.
 - Implement some unit-tests in the `mod tests` section. Usually the example inputs/outputs found in said question.
 - ***[Optionally]***, you could rename the functions however you wish - but then you need to remember to update the `proc_macro_aoc::impl_day!(part_1, part_2);` line correspondingly as well.

In other words, the only overhead is bullet 1. The other bullets can be summed up as "solve the question".

---------

You can build, run & test with cargo in the terminal:
```console
> cargo build
> 
> cargo test
> cargo test module_name
> 
> cargo run
> cargo run [--] \d+
> cargo run module_name
> cargo run -- -a
```

You can also helpfully expand macros with cargo. Take note it doesn't show unit-tests:
```console
> cargo expand
```

Benchmarks doesn't work in stable. To test benchmarks you need `rustup` AND `cargo`:
```console
> rustup run nightly cargo bench`
```


## Examples (Run/Unit-Test): ##
  - > `cargo run`
    * Executes the implementations for latest day.
    * example output:
      ```console
      > $ cargo run
      >
      > = 4 =
      > Part 1: Ok("")
      > Part 2: Ok("")
      ```

  - > `cargo run 1`
    * Executes  the implementations for the given day
    * example output:
      ```console
      > $ cargo run 1
      >
      > = 1 =
      > Part 1: Ok(6700587)
      > Part 2: Ok(6700587)
      ```
    
  - > `cargo run -- -a`
    * Executes the implementations for all days, starting with latest goind down to day 1.
    * example output:
      ```console
      > $ cargo run -- -a
      >
      > = 4 =
      > Part 1: Ok("")
      > Part 2: Ok("")
      > = 3 =
      > Part 1: Ok(54839567422891)
      > Part 2: Ok(54839567422891)
      > = 2 =
      > Part 1: Ok("")
      > Part 2: Ok("")
      > = 1 =
      > Part 1: Ok(6700587)
      > Part 2: Ok(6700587)
      ```
    
  - > `cargo test`
    * Runs all unit-tests.
    * Take note that they are mostly unordered,
       although often they atleast go from day 1 and upwards
    * example output:
      ```console
      > $ cargo test 
      >
      > running 16 tests
      > test day1::tests::test_1_0 ... ok
      > test day1::tests::test_1_1 ... ok
      > test day1::tests::test_2_1 ... ok
      > test day1::tests::test_2_0 ... ok
      > test day2::tests::test_1_1 ... ok
      > test day2::tests::test_2_1 ... ok
      > test day2::tests::test_1_0 ... ok
      > test day2::tests::test_2_0 ... ok
      > test day3::tests::test_1_0 ... ok
      > test day3::tests::test_1_1 ... ok
      > test day3::tests::test_2_0 ... ok
      > test day3::tests::test_2_1 ... ok
      > test day4::tests::test_1_0 ... ok
      > test day4::tests::test_1_1 ... ok
      > test day4::tests::test_2_0 ... ok
      > test day4::tests::test_2_1 ... ok
      ```
      
  - > `cargo test day1`
    * Runs all unit-tests for the given module.
    * Take note that they are entirely unordered.
    * example output:
      ```console
      > $ cargo test day1
      >
      > running 4 tests
      > test day1::tests::test_1_0 ... ok
      > test day1::tests::test_1_1 ... ok
      > test day1::tests::test_2_1 ... ok
      > test day1::tests::test_2_0 ... ok
      ```
      