//!
//! `if` in Rust
//!
//! # Examples
//!
//! ```rust
//! let x = 5;
//!
//! if x == 5 {
//!     println!("x is five!");
//! }
//! ```
//!
//! ```rust
//! let x = 5;
//! 
//! if x == 5 {
//!     println!("x is five!");
//! } else {
//!     println!("x is not a five :(");
//! }
//! ```
//!
//! ```rust
//! let x = 5;
//!
//! if x == 5 {
//!     println!("x is five!");
//! } else if x == 6 {
//!     println!("x is six!");
//! } else {
//!     println!("x is not five or six :(");
//! }
//! ```
//!
//! Rust allows this also.
//!
//! ```rust
//! let x = 5;
//!
//! let y = if x == 5 {
//!     10
//! } else {
//!     15
//! }
//! ```
//!
//! This works because `if` is an expression. the value of the expression is the value of the last
//! expression in wichever branch was chosen. An `if` without `else` always results in `()` as the
//! value.
//!

// `main()` function
fn main() {
    let x = 5;

    let y = if x == 5 { 10 } else { 15 };

    if y == 10 {
        println!("y is ten!");
    } else if y == 15 {
        println!("y is fifteen!");
    } else {
        println!("y is not ten or fifteen :(");
    }
}
