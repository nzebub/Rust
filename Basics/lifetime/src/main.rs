//!
//! Lifetimes in Rust
//!
//! Reference is pointing to an invalid resource. This is called a dangling pointer or 'use after
//! free', when the resource is memory.
//!
//! ```rust
//! let r;              //  Introduce reference: 'r'.
//! {
//!     let i = 1;      //  Introduce scoped value: 'i'.
//!     r = &i;         //  Store reference of 'i' in 'r'.
//! }                   //  'i' goes out of scope and is dropped.
//!
//! println!("{}", r);  //  'r' still refers to 'i'. got an error in Rust.
//! ```
//!
//! When we have a function that takes argument by reference the situation becomes more complex.
//!
//! ```rust
//! fn skip_prefix(line: &str, prefix: &str) -> &str {
//!     // ...
//! }
//!
//! let line = "lang:en=Hello World";
//! let lang = "en";
//!
//! let v;
//! {
//!     let p = format!("lang:{}=", lang);  //  -+  'p' comes into scope.
//!     v = skip_prefix(line, p.as_str());  //   |
//! }                                       //  -+  'p' goes out of scope.
//! println!("{}", v);
//! ```
//!
//! Because of the above ambiguity, Rust will refuse to compile the example code. To get it to
//! compile we need to tell the compiler more about the lifetime of the references.
//!
//! This can be done by making the lifetimes explicit in the function declaration.
//!
//! ```rust
//! fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
//!     //  ...
//! }
//! ```
//!
//! **Syntax**
//!
//! We use `<>` to declare our lifetimes. This says that `bar` has one lifetime, `'a`. 
//!
//! ```rust
//! fn bar<'a>(...)
//! ```
//!
//! If we had two reference parameters with different lifetimes, it would look like this:
//!
//! ```rust
//! fn bar<'a, 'b>(...)
//! ```
//!
//! Then in our parameter list, we use the lifetime we've named:
//!
//! ```rust
//! ...(x: &'a i32)
//!
//! //  or
//!
//! ...(x: &'a mut i32)
//! ```
//!
//! **In `struct`s**
//!
//! ```rust
//! struct Foo<'a> {
//!     x: &'a i32,
//! }
//! ```
//!
//! **`imp` blocks**
//! 
//! ```rust
//! struct Foo<'a> {
//!     x: &'a i32,
//! }
//!
//! impl<'a> Foo<'a> {
//!     fn x(&self) -> &'a i32 { self.x }
//! }
//! 
//! fn main() {
//!     let y = &5; // This is the same as `let _y = 5; let y = &_y;`.
//!     let f = Foo { x: y };
//!
//!     println!("x is: {}", f.x());
//! }
//! ```
//!
//! **Thinking in scopes**
//!
//! ```rust
//! struct Foo<'a> {
//!     x: &'a i32,
//! }
//!
//! fn main() {
//!     let y = &5;             // -+ `y` comes into scope.
//!     let f = Foo { x: y };   // -+ `f` comes into scope.
//!                             //  |
//!     // Stuff...             //  |
//!                             //  |
//! }                           // -+ `f` and `y` go out of scope.
//! ```
//! 
//! **'static**
//!
//! The lifetime named 'static' is a special lifetime. It signals that something has the lifetime
//! of te entire program. Most Rust programmers first come across `'static` when dealing with
//! strings.
//!
//! ```rust
//! let x:&'static str = "Hello, world.";
//! ```
//!
//! Another example are globals:
//!
//! ```rust
//! static FOO: i32 = 5;
//! let x: &'static i32 = &FOO;
//! ```
//!
//! This adds n `i32` to the data segment of the binary, and `x` is a reference to it.
//!
//! **Lifetime Elision**
//!
//! Here are the three rules:
//! 
//!  * Each elided lifetime in a afunction's argument becomes a distinct lifetime parameter.
//!  * If there is exactly one input lifetime, elided or not, that lifetime is assigned to all
//!  elided lifetimes in the return values of that function.
//!  * If there are multiple input lifetimes, but one of them is `&self` or `&mut self`, the
//!  lifetimeof `self` is assigned to all elided output lifetimes.
//!

// `main()` function
fn main() {

}
