//!
//! Vectors in Rust.
//!
//! A 'vector' is a dynamic or 'growable' array, implemented as the standard library type `Vec<T>`.
//! The `T` means that we can have vectors of any type. Vectors always allocate their data on the
//! heap. You can create them with the `vec!` macro.
//!
//! ```rust
//! let v = vec![1, 2, 3, 4, 5];    //  v: Vec<i32>
//!
//! let w = vec![0; 10];    //  A vector of ten zeros.
//! ```
//!
//! Vectors store their contents as contiguous arrays of `T` on the heap.
//!
//! **Accessing elements**
//!
//! To get the value at a particular index in the vector, we use `[]`s:
//!
//! ```rust
//! let v = vec![1, 2, 3, 4, 5];
//!
//! println!("The elements of v is {:?}", &v);
//! println!("The third element of v is {}", v[2]);
//! ```
//!
//! **Out-of-bounds Access**
//!
//! If you wand to handle out-of-bounds errors without panicking, you can use methods like `get`
//! and `get_mut` that return `None` when given an invalid index:
//!
//! ```rust
//! let v = vec![1, 2, 3];
//!
//! match v.get(7) {
//!     Some(x) => println!("Item in 7 is {}", x);
//!     None    => println!("Sorry, this vector is too short.");
//! }
//! ```
//!
//! **Iterating**
//!
//! Once you have a vector, you can iterate through its elements with `for`. There are three
//! versions:
//!
//! ```rust
//! let mut v = vec![1, 2, 3, 4, 5];
//!
//! for i in &v {
//!     println!("A reference to {}", i);
//! }
//!
//! for i in &mut v {
//!     println!("A mutable reference ot {}", i);
//! }
//!
//! for i in v {
//!     println!("Take ownership of the vector and its element {}", i);
//! }
//! ```
//!

// `main()` function
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    println!("The elements of vector v is {:?}", v);
    println!("The third element of vector v is {}", v[2]);

    for i in &v {
        println!("-- {}", i);
    }

    let mut w = vec![1, 2, 3];

    for i in &mut w {
        *i += 4;
    }
    println!("w --- {:?}", w);
}
