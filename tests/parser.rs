extern crate htmlstream;
use htmlstream::*;
use htmlstream::HTMLTagState::*;

#[test]
fn test_parse_tag() {
    let html = "this is a test: <a href=\"http://rust-lang.org\" disabled>The Rust Programing Language</a>";
    let mut list: Vec<(Position, HTMLTag)> = vec![];
    for (pos, tag) in tag_iter(&html) {
        list.push((pos, tag));
    }
    assert_eq!(list, [
        (Position { start: 0, end: 16 }, HTMLTag { name: "".to_string(), html: "this is a test: ".to_string(), attributes: "".to_string(), state: Text }),
        (Position { start: 16, end: 56 }, HTMLTag { name: "a".to_string(), html: "<a href=\"http://rust-lang.org\" disabled>".to_string(), attributes: "href=\"http://rust-lang.org\" disabled".to_string(), state: Opening }),
        (Position { start: 56, end: 84 }, HTMLTag { name: "".to_string(), html: "The Rust Programing Language".to_string(), attributes: "".to_string(), state: Text }),
        (Position { start: 84, end: 88 }, HTMLTag { name: "a".to_string(), html: "</a>".to_string(), attributes: "".to_string(), state: Closing })
    ]);
}

#[test]
fn test_parse_attributes() {
    let html = "this is a test: <a href=\"http://rust-lang.org\" disabled>The Rust Programing Language</a>";
    let mut list: Vec<(Position, HTMLTagAttribute)> = vec![];
    for (_, tag) in tag_iter(&html) {
        for (pos, attr) in attr_iter(&tag.attributes) {
            list.push((pos, attr));
        }
    }
    assert_eq!(list, [
        (Position { start: 0, end: 27 }, HTMLTagAttribute { name: "href".to_string(), value: "http://rust-lang.org".to_string() }),
        (Position { start: 28, end: 36 }, HTMLTagAttribute { name: "disabled".to_string(), value: "".to_string() })
    ]);
}

#[test]
fn test_parse_attributes_2() {
    let html = "a=123\"bbb bbb=\"456\"ccc='789' ddd
fff=ggg hhhh";
    let mut list: Vec<(Position, HTMLTagAttribute)> = vec![];
    for (pos, attr) in attr_iter(html) {
        list.push((pos, attr));
    }
    assert_eq!(list, [
        (Position { start: 0, end: 9 }, HTMLTagAttribute { name: "a".to_string(), value: "123\"bbb".to_string() }),
        (Position { start: 10, end: 19 }, HTMLTagAttribute { name: "bbb".to_string(), value: "456".to_string() }),
        (Position { start: 19, end: 28 }, HTMLTagAttribute { name: "ccc".to_string(), value: "789".to_string() }),
        (Position { start: 29, end: 32 }, HTMLTagAttribute { name: "ddd".to_string(), value: "".to_string() }),
        (Position { start: 33, end: 40 }, HTMLTagAttribute { name: "fff".to_string(), value: "ggg".to_string() }),
        (Position { start: 41, end: 45 }, HTMLTagAttribute { name: "hhhh".to_string(), value: "".to_string() })
    ]);
}
