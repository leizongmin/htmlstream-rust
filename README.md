# htmlstream-rust
Lightweight HTML parser

[![Build Status](https://travis-ci.org/leizongmin/htmlstream-rust.svg?branch=master)](https://travis-ci.org/leizongmin/htmlstream-rust)
[![](http://meritbadge.herokuapp.com/htmlstream)](https://crates.io/crates/htmlstream)
[![](https://img.shields.io/crates/d/htmlstream.svg)](https://crates.io/crates/htmlstream)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

### Documents

https://leizongmin.github.io/htmlstream-rust/


### Dependencies

Add the following to the `Cargo.toml` file:

```toml
[dependencies.htmlstream]
version = "*"
```


### Examples

```rust
extern crate htmlstream;

let html = "this is a test: <a href=\"http://rust-lang.org\">The Rust Programing Language</a>";
for (pos, tag) in htmlstream::tag_iter(html) {
    println!("{:?} {:?}", pos, tag);
    for (pos, attr) in htmlstream::attr_iter(&tag.attributes) {
        println!("    {:?} {:?}", pos, attr);
    }
}
```

Output:

```
Position { start: 0, end: 16 } HTMLTag { name: "", html: "this is a test: ", attributes: "", state: Text }
Position { start: 16, end: 47 } HTMLTag { name: "a", html: "<a href=\"http://rust-lang.org\">", attributes: "href=\"http://rust-lang.org\"", state: Opening }
    Position { start: 0, end: 27 } HTMLTagAttribute { name: "href", value: "http://rust-lang.org" }
Position { start: 47, end: 75 } HTMLTag { name: "", html: "The Rust Programing Language", attributes: "", state: Text }
Position { start: 75, end: 79 } HTMLTag { name: "a", html: "</a>", attributes: "", state: Closing }
```

## License

```
The MIT License (MIT)

Copyright (c) 2015 Zongmin Lei <leizongmin@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
