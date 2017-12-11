//!
//! Functions in `Rust`
//!
//! Every `Rust` program has atleast one fuction, the `main` function:
//!
//! ```rust
//! fn main() {
//! }
//! ```
//! Function should be created by `fn` keyword followed by the name, some parentheses with or
//! without arguments, then return type and some curly braces to indicate the body.
//!
//! # Example
//!
//! ```rust
//! fn example_function(x: i32, y: f64) -> i32 {
//!     //Some statements
//! }
//! ```
//! The above function named `example_function` has one integer x and one float y as argument and has a retuen type interger.
//!
//! Function pointer, we can create variable bindings wihch point to functions.
//! ```rust
//! let f: fn(i32, f64) -> i32;
//! ```
//!

/// `main()` function
fn main() {
    let f: fn(i32, f64) -> i32 = example_function;
    
    let x = f(3, 4.5);
    println!("x = {}", x);
}

fn example_function(x: i32, y: f64) -> i32 {
    println!("x = {}, y = {}.", x, y);
    x
}

