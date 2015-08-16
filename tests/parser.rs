extern crate htmlstream;
use htmlstream::*;

#[test]
fn test_parse_html() {
    let html = "this is a test: <a href=\"http://rust-lang.org\">The Rust Programing Language</a>";
    parse_html(html, |pos: &Position, tag: &HTMLTag| {
        println!("{:?} {:?}", pos, tag);
    });
}

