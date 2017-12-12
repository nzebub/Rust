//!
//! Loops in Rust
//!
//! Rust provides three approaches to performing some kind of iterative activity. They are: `loop`,
//! `while` and `for`.
//!
//! **loop**
//!
//! The infinite `loop` is the simplest of loop available in rust. Using the keyword `loop`, rust
//! provides a way to loop indefinitely until some terminating statement is reached.
//!
//! ```rust
//! loop {
//!     println!("Loop forever!");
//! }
//! ```
//!
//! **while**
//!
//! Rust also has a `while` loop.
//!
//! ```rust
//! let mut x = 5;
//! let mut done = false;
//!
//! while !done {
//!     x += x - 3;
//!
//!     println!("x - {}", x);
//!
//!     if x % 5 == 0 {
//!         done = ture;
//!     }
//! }
//! ```
//!
//! **for**
//!
//! The `for` loop is used to loop a particular number of times.
//!
//! ```rust
//! for x in 0..10 {
//!     println!("x - {}", x):
//! }
//! ```
//!
//! **Enumerate**
//!
//! When you need to keep track of how many times you have alreay looped, you can use the
//! `.enumerate()` function.
//!
//! ```rust
//! for (index, value) in (5..10).enumerate() {
//!     println!("index - {}, and value - {}", index, value);
//! }
//! ```
//!
//! Outputs:
//!
//! ```text
//! index - 0, and value -5
//! index - 1, and value -6
//! index - 2, and value -7
//! index - 3, and value -8
//! index - 4, and value -9
//! ```
//! **On iterators:**
//!
//! ```rust
//! let lines = "hello\nworld".lines();
//!
//! for (linenumber, line) in lines.enumerate() {
//!     println!("{}: {}", linenumber, line);
//! }
//! ```
//! Outputs:
//! 
//! ```text
//! 0: hello
//! 1: world
//! ```
//!
//! **Ending iteration early**
//!
//! Rust has two keywords to help us with modifying iteration: `break` and `continue`.
//!
//! In this case, we can write the loop in a better way with `break`:
//!
//! ```rust
//! let mut x = 5;
//!
//! loop {
//!     x += x - 3;
//!
//!     println!("{}", x);
//!
//!     if x % 5 == 0 { break; }
//! }
//! ```
//!
//! `continue` is similar, but instead of ending the loop, it goes to the next iteration.
//!
//! ```rust
//! for x in 0..10 {
//!     if x % 2 == 0 { continue; }
//!
//!     println!("{}", x);
//! }
//! ```
//!
//! **Loop labels**
//!
//! ```rust
//! 'outer: for x in 0..10 {
//!     'inner: for y in 0..10 {
//!         if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
//!         if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
//!         println!("x: {}, y: {}", x, y);
//!     }
//! }
//! ```
//!

// `main()` function
fn main() {
    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {
        x += x - 3;
        
        println!("{}", x);
        
        if x % 5 == 0 {
            done = true;
        }
    }

    for (index, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }

    let lines = "hello\nworld".lines();
    
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    let mut x = 5;
    
    loop {
        x += x - 3;
        
        println!("{}", x);
        
        if x % 5 == 0 { break; }
    }

    for x in 0..10 {
        if x % 2 == 0 { continue; }
        
        println!("{}", x);
    }

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
            if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }
}

