#![crate_name = "htmlstream"]
#![doc(html_root_url = "https://leizongmin.github.io/htmlstream-rust/")]

pub use base::*;
pub use tag::*;
pub use attribute::*;

mod base;
mod tag;
mod attribute;
