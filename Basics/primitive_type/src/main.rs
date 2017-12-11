//!
//! Primitive Types in `Rust`
//!
//! # Booleans
//!
//! ```rust
//! let x = true;
//!
//! let y: bool = false;
//! ```
//! The `Bool` type has two values, `true` and `false`.
//! 
//! # Char
//!
//! ```rust
//! let x = 'x';
//!
//! let two_hearts: char = '💕';
//! ```
//! The `char` type represents a single Unicode scalar value. Rust`s `char` is not a single byte,
//! but four.
//!
//! # Numeric types
//!
//! The different numeric types are,
//!
//!  * i8
//!  * i16
//!  * i32
//!  * i64
//!  * u8
//!  * u16
//!  * u32
//!  * u64
//!  * isize
//!  * usize
//!  * f32
//!  * f64
//!
//! Signed and Unsigned
//!
//! Usigned types use a `u`for their category, and signed types use `i`.
//!
//! Fixed-size types
//!
//! The before signed and unsigned category having one of suffix 8, 16, 32, 64 are fixed-size type.
//!
//! Variable-size types
//!
//! The before signed and unsigned category having suffix `size` are variable-sized type.
//!
//! Floatinf-point types
//!
//! Rust also has two floating point types,
//!  * `f32` 32-bit floating point
//!  * `f64` 64-bit floating point
//!
//! # Arrays
//!
//! Rust has list types to represent a sequence of things. The most basic is the __array__, a
//! fixed-size list of elements of the same type. By default arrays are immutable
//! 
//! ```rust
//! let a = [0; 10];
//!
//! let b = [1, 2, 3];
//!
//! let c = ["hi", "bye"];
//!
//! println!("First element of a is {}", a[0]);
//! println!("b has {} elements.", b.len());
//! println!("Second element of c is {}", c[1]);
//! ```
//!
//! # Slices
//!
//! A `slice` is a reference to another data structure. Internally, slices are represented as a
//! pointer to the begining of the data and a length. Slices have type `&[T]`.
//! 
//! ```rust
//! let a = [0, 1, 2, 3, 4, 5];
//!
//! let complete = &a[..];
//!
//! let middle = &a[1..5];
//! ```
//!
//! `str`
//!
//! Rust's `str` type is the most primitive string type. As an unsized type.
//!
//! # Tuples
//!
//! A tuple is an ordered list of fixed size. The parenthesese and commas for the tuple.
//!
//! ```rust
//! let mut x = (1, 2);
//!
//! let y = (2,3);
//! ```
//!
//! ```rust
//! let (x, y, z) = (1, 2, 3);
//!
//! println!("x is {}", x);
//! ```
//!
//! The Tuple Indexing.
//!
//! ```rust
//! let tuple = (1, 2, 3);
//!
//! let x = tuple.0;
//! let y = tuple.1;
//! let z = tuple.2;
//! ```
//!
//! # Functions
//!
//! Functions also have a type!
//!
//! ```rust
//!
//! fn foo(x: i32) -> i32 { x }
//!
//! let x: fn(i32) -> i32 = foo;
//! ```
//!

/// `main()` function
fn main() {
    let x = true;
    let y: bool = false;

    println!("x - {}, y - {}", x, y);

    let x = 'x';
    let two_hearts = '💕';

    println!("x - {}, {}", x, two_hearts);

    let x = 42; // `x` has type `i32`.
    let y = 1.0; // `y` has type `f64`.

    println!("x - {}, y - {}", x, y);

    let a = [1, 2, 3];
    let b = ["hi", "bye"];

    println!("a is {:?} and its length {}", a, a.len());
    println!("the second element of b is {}", b[1]);

    let a = [0, 1, 2, 3, 4];
    let complete = &a[..]; // A slice containing all of the elements in `a`.
    let middle = &a[1..4]; // A slice of `a`: only the elements `1`, `2`, and `3`.

    println!("Complete - {:?}, middle - {:?}", complete, middle);

    let x = (1, "hello");

    println!("x - {:?}", x);

    let (x, y, z) = (1, 2, 3);

    println!("x is {}, y is {}, z is {}", x, y, z);

    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("x is {}, y is {}, z is {}", x, y, z);
}
