//! # Cargo And Crates
//!
//! `cargo_and_crates` is a collection of utilities to make performing certain
//! calculations more convenient.

pub mod kinds;
pub mod utils;

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, cargo_and_crates::add_one(five));
/// ```
pub fn add_one(x: i32) -> i32 {
    add(x, 1)
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}


