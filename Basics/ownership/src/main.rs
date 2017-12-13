//!
//! Ownership in Rust.
//!
//! `Ownership` is one of Rust's most distinct and compelling fetures. Ownership is how Rust
//! achives its lrgest goal, memory safety. There are a few distinct concecpts,
//!
//!  * Ownership
//!  * Borrowing, and their associated feature 'references'
//!  * Lifetimes, an advanced concept of borrowing
//!
//! Rust ensures that there is __exactly one__ binding to any given resource. For example, if we
//! have a vector, we can assign it to another binding:
//!
//! ```rust
//! let v = vec![1, 2, 3];
//!
//! let v2 = v; //  Vector `v` ownership is moved to v2 variable binding.
//! ```
//!
//! But, if we try to use `v` afterwords, we get an error
//!
//! A similar thing happens if we define a function which takes ownership, and try to use something
//! after we've passed it as an argument:
//!
//! ```rust
//! fn take(v: Vec<i32>) {
//!     //  something ...
//! }
//!
//! let v = vec![1, 2, 3];
//!
//! take(v);    //  Vector `v` ownership is moved to function variable `v`binding.
//! ```
//!
//! **copy types**
//!
//! All primitive types implement the copy trait and their ownership is therfore not moved like
//! one would assume, following the 'ownership rules'
//!
//! ```rust
//! let v = 1;
//!
//! let v2 = v;
//!
//! println!("v is {}", v); //  This works, because v: i32 type as a primitive that implement the
//! copy trait.
//! ```
//!
//!
//!

//`main()` function
fn main() {
    let v = vec![1, 2, 3];
    let v2 = v; //  v ownership is moved to v2.

    println!("v2 -- {:?}", v2);

    //  But
    let v = 1;
    let v2 = v; //  v ownership is not moved to v2, because v is a primitive type that implement the __copy__ trait.

    println!("v -- {:?}, v2 -- {:?}", v, v2);
}
