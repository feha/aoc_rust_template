A Rust 'framework' for AoC to easily add new days.
Using proc_macro's to remove the need for user to rename anything when starting a new day,
with the exception of the filename when copying the day template-file.


RESULTS:
  TODO: Make it populate this section with start-times when you start a new day.
        Probably best achieved through a simply CLI-tool,
        that edits here + copies the template-file when starting each day.
        Could also edit the .toml do add binaries so the main-functions work.


You can build, run & test with cargo
  > cargo build
  > 
  > cargo test
  > cargo test module_name
  > 
  > cargo run
  > cargo run [--] \d+
  > cargo run module_name
  > cargo run -- -a

You can also helpfully expand macros with cargo. Take note it doesn't show unit-tests.
  > cargo expand

Benchmarks doesn't work in stable
To test benchmarks you need rustup AND cargo
  rustup run nightly cargo bench


Examples:
  - $ cargo run
    * Executes the implementations for latest day.
    * example output:
      >
      > $ cargo run
      > = 4 =
      > Part 1: Ok("")
      > Part 2: Ok("")

  - $ cargo run 1
    * Executes  the implementations for the given day
    * example output:
      > $ cargo run 1
      >
      > = 1 =
      > Part 1: Ok(6700587)
      > Part 2: Ok(6700587)
    
  - $ cargo run -- -a
    * Executes the implementations for all days, starting with latest goind down to day 1.
    * example output:
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
    
  - $ cargo test
    * Runs all unit-tests.
    * Take not that they are mostly unordered,
       although often they atleast go from day 1 and upwards
    * example output:
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
      
  - $ cargo test day1
    * Runs all unit-tests for the given module.
    * Take not that they are entirely unordered.
    * example output:
      > $ cargo test day1
      >
      > running 4 tests
      > test day1::tests::test_1_0 ... ok
      > test day1::tests::test_1_1 ... ok
      > test day1::tests::test_2_1 ... ok
      > test day1::tests::test_2_0 ... ok
      