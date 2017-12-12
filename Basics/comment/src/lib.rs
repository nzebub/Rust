//!
//! Comments in `Rust`
//!
//! Rust provides as two kind of comments.
//!
//!  * _line comments_
//!  * _doc comments_
//!
//! **Line Comments**
//!
//! ```rust
//! // Line comments are anything after '//' and extend to the end of the line.
//!
//! let x = 5; // Another example of line comment.
//! ```
//!
//! **Doc Comments**
//!
//! Doc comments use `///` instead of `//`, and support Markdown notation inside.
//!
//! There is another style of doc comment, `//!`, to comment containing items crates, modules or
//! function. Commeonly used inside crates root (lib.rs) or modules root (mod.rs).
//!
//! `rustdoc` tool is used to generate HTML documentation from these doc comments, and also to run
//! the code examples as tests!
//!

