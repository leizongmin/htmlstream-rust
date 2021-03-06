#![crate_name = "htmlstream"]
#![crate_type = "rlib"]

//! #Lightweight HTML parser
//!
//! Examples:
//!
//! ```rust
//! extern crate htmlstream;
//!
//! fn main() {
//!     let html = "this is a test: <a href=\"http://rust-lang.org\">The Rust Programing Language</a>";
//!     for (pos, tag) in htmlstream::tag_iter(html) {
//!         println!("{:?} {:?}", pos, tag);
//!         for (pos, attr) in htmlstream::attr_iter(&tag.attributes) {
//!             println!("    {:?} {:?}", pos, attr);
//!         }
//!     }
//! }
//! ```
//!
//! Output:
//!
//! ```{rust,ignore}
//! Position { start: 0, end: 16 } HTMLTag { name: "", html: "this is a test: ", attributes: "", state: Text }
//! Position { start: 16, end: 47 } HTMLTag { name: "a", html: "<a href=\"http://rust-lang.org\">", attributes: "href=\"http://rust-lang.org\"", state: Opening }
//!     Position { start: 0, end: 27 } HTMLTagAttribute { name: "href", value: "http://rust-lang.org" }
//! Position { start: 47, end: 75 } HTMLTag { name: "", html: "The Rust Programing Language", attributes: "", state: Text }
//! Position { start: 75, end: 79 } HTMLTag { name: "a", html: "</a>", attributes: "", state: Closing }
//! ```

pub use base::{Position, HTMLTagState, HTMLTag, HTMLTagAttribute};
pub use tag::{HTMLTagIterator, tag_iter};
pub use attribute::{HTMLTagAttributeIterator, attr_iter};

mod base;
mod tag;
mod attribute;
