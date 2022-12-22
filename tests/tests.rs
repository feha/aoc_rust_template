// Only compile this file if testing ('cargo test', but not 'cargo bench')
#![cfg(test)]


// #[test] // This function is a unit-test.
// #[ignore] // ignore this test; doesn't run it, but still list (as ignored)
// fn hello_test_unimplemented() {
//     unimplemented!();
// }

// #[test]
// #[should_panic] // succeeds of it panics
// fn hello_test_panic() {
//     panic!();
// }

// #[test]
// fn hello_test_assert_true() {
//     assert!(true);
// }

// #[test]
// #[ignore]
// fn hello_test_assert_false() {
//     assert!(false);
// }

// #[test]
// fn hello_test_assert_eq_true() {
//     assert_eq!(false, false);
// }

// #[test]
// #[ignore]
// fn hello_test_assert_eq_false() {
//     assert_eq!(true, false);
// }

// #[test]
// fn hello_test_return_true() -> Result<(), i32> {
//     return Ok(());
// }

// #[test]
// #[ignore]
// fn hello_test_return_false() -> Result<(), i32> {
//     return Err(2);
// }