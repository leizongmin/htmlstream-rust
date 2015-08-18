extern crate htmlstream;
use htmlstream::*;

fn main() {
    let html = "this is a test: <a href=\"http://rust-lang.org\">The Rust Programing Language</a>";
    for (pos, tag) in tag_iter(html) {
        println!("{:?} {:?}", pos, tag);
        for (pos, attr) in attr_iter(&tag.attributes) {
            println!("    {:?} {:?}", pos, attr);
        }
    }
}
