//!
//! References and Borrowing in Rust.
//!
//! Borrowing is one of Rust's most distinct and compelling features.
//!
//! ```rust
//! fn main() {
//!     fn sum_vec(v: &Vec<i32>) -> i32 {
//!         v.iter().fold(0, |a, &b| a + b)
//!     }
//!     // Borrow two vectors and sum them.
//!
//!     fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
//!         // Do stuff with `v1` and `v2`.
//!         let s1 = sum_vec(v1);
//!         let s2 = sum_vec(v2);
//!         // Return the answer.
//!         s1 + s2
//!     }
//!
//!     let v1 = vec![1, 2, 3];
//!     let v2 = vec![4, 5, 6];
//!
//!     let answer = foo(&v1, &v2);
//!     println!("{}", answer);
//! }
//! ```
//!
//! **&mut references**
//!
//! There’s a second kind of reference: &mut T. A ‘mutable reference’ allows you to mutate the resource you’re borrowing. For example:
//!
//! ```rust
//! let mut x = 5;
//! {
//!     let y = &mut x;     //  -+  &mut borrow 'x' starts here.
//!     *y += 1;            //   |
//! }                       //  -+  ... and ends here.
//!
//! println!("{}", x);      //  <-  Try to borrow 'x' here.
//! ```
//! 
//! # The Rules
//!
//! Here are the rules for borrowing in Rust.
//!
//! First, any borrow must last for a scope no greater than that of the owner. Second, you may have
//! one or the other of these two kinds of borrows, but not both at the same time:
//!
//!  * one or more references (&T) to a resource,
//!  * exactly one mutable reference (&mut T).
//!
//! **Use after free**
//!
//! References must not live longer than the resource they refer to. Rust will check scopes of your
//! references to ensure thathis is true.
//!
//! ```rust
//! let y: &i32;
//! {
//!     let x = 5;
//!     y = &x;
//! }               // At the end of this scope 'x' is dropped here.
//!
//! println!("{}", y);  //  So cannot access y. Got an error.
//! ```
//!
//! The same problem occurs when the reference is declared __before__ the variable it refers to.
//! This is because resources within the same scope are freed in the opposite order they were
//! declared:
//!
//! ```rust
//! let y: i32;
//! let x = 5;
//! y = &x;
//!
//! println!("{}", y);  //  got an error similar to above.
//! ```
//!

// `main()` function
fn main() {
    fn sum_vec(v: &Vec<i32>) -> i32 {
        v.iter().fold(0, |a, &b| a + b)
    }
    // Borrow two vectors and sum them.
    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // Do stuff with `v1` and `v2`.
        let s1 = sum_vec(v1);
        let s2 = sum_vec(v2);
        // Return the answer.
        s1 + s2
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let answer = foo(&v1, &v2);
    println!("{}", answer);

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);
}

