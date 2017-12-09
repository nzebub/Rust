//!
//! Variables in `Rust`
//!
//! Binding values to a variable name through `Rust` keyword `let`
//!
//! # Example
//!
//! ```rust
//! let x = 5;
//!
//! let (x, y) = (1, 2);
//!
//! let x: i32 = 5;
//! ```
//!
//! Basically `Rust` variables are immutable bindings of value by default.
//! If you want to change or update values to variable through `mut` keyword.
//! # Example
//!
//! ```rust
//! let mut x = 5;
//!
//! x = 10;
//! ```
//! Variable initialization is must in `Rust`. Otherwise it give an error.
//! The following statements are not allowed in `Rust`
//!
//! # Example
//!
//! ```rust
//! let x;
//!
//! let mut y;
//! ```
//!
//! Scope of variable in `Rust`
//!
//! # Example
//! 
//! ```rust
//! let x = 5;
//! {
//!     let y = 6;
//! }   // y dropped here
//! println!("x = {}, y = {}",x, y);    // Here y cannot be accessed by println!().
//! ```
//!
//! Shadowing of variable in `Rust`.
//!
//! Immutable variables are allowed to shadowed.
//! But mutable variables are not shadowed as immutble one.
//!
//! # Example
//!
//! ```rust
//! let x = 5;
//! {
//!     let x = 6;
//!     // Using x as 6 here.
//! }
//! // Using x as 5 here.
//! ```
//!
//! ```rust
//! let mut x = 5;
//!
//! x = 7;
//!
//! let x = x; // Not allowed this statement by `Rust`
//! ```
//!

/// main() function
fn main() {
    let x = 5;
    let y = 6;
    {
        let y: i32 = -6;
        println!("x = {}, y = {}", x, y);   // x = 5
        let mut x = x + 1;  // x = 5 + 1 = 6
        x = x + 2;  // x = 6 + 2 = 8
        println!("x = {}, y = {}", x, y);
    }
    println!("x = {}, y = {}", x, y);   // Here x = 5, y = 6
}


