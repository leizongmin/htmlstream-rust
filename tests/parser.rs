extern crate htmlstream;
use htmlstream::*;
use htmlstream::HTMLTagState::*;

#[test]
fn test_parse_rest_text() {
    let html = "this is a test: <a href=\"http://rust-lang.org\" disabled>The Rust Programing Language</a>\r\n\r\n";
    let mut list: Vec<(Position, HTMLTag)> = vec![];
    for (pos, tag) in tag_iter(&html) {
        list.push((pos, tag));
    }
    assert_eq!(list, [
        (Position { start: 0, end: 16 }, HTMLTag { name: "".to_string(), html: "this is a test: ".to_string(), attributes: "".to_string(), state: Text }),
        (Position { start: 16, end: 56 }, HTMLTag { name: "a".to_string(), html: "<a href=\"http://rust-lang.org\" disabled>".to_string(), attributes: "href=\"http://rust-lang.org\" disabled".to_string(), state: Opening }),
        (Position { start: 56, end: 84 }, HTMLTag { name: "".to_string(), html: "The Rust Programing Language".to_string(), attributes: "".to_string(), state: Text }),
        (Position { start: 84, end: 88 }, HTMLTag { name: "a".to_string(), html: "</a>".to_string(), attributes: "".to_string(), state: Closing }),
        (Position { start: 88, end: 92 }, HTMLTag { name: "".to_string(), html: "\r\n\r\n".to_string(), attributes: "".to_string(), state: Text })
    ]);
}

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

#[test]
fn test_parse_html_1() {
    let html = "<a href=\"javascript:alert(/xss/)\" title=ok disabled>hello</a> <b>ok</b>";
    let mut list: Vec<(Position, HTMLTag)> = vec![];
    for (pos, tag) in tag_iter(&html) {
        list.push((pos, tag));
    }
    assert_eq!(list, [
        (Position { start: 0, end: 52 }, HTMLTag { name: "a".to_string(), html: "<a href=\"javascript:alert(/xss/)\" title=ok disabled>".to_string(), attributes: "href=\"javascript:alert(/xss/)\" title=ok disabled".to_string(), state: Opening }),
        (Position { start: 52, end: 57 }, HTMLTag { name: "".to_string(), html: "hello".to_string(), attributes: "".to_string(), state: Text }),
        (Position { start: 57, end: 61 }, HTMLTag { name: "a".to_string(), html: "</a>".to_string(), attributes: "".to_string(), state: Closing }),
        (Position { start: 61, end: 62 }, HTMLTag { name: "".to_string(), html: " ".to_string(), attributes: "".to_string(), state: Text }),
        (Position { start: 62, end: 65 }, HTMLTag { name: "b".to_string(), html: "<b>".to_string(), attributes: "".to_string(), state: Opening }),
        (Position { start: 65, end: 67 }, HTMLTag { name: "".to_string(), html: "ok".to_string(), attributes: "".to_string(), state: Text }),
        (Position { start: 67, end: 71 }, HTMLTag { name: "b".to_string(), html: "</b>".to_string(), attributes: "".to_string(), state: Closing })
    ]);
}
