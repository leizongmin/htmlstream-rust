use base::*;


#[derive(Debug)]
pub struct HTMLTagAttributeIterator<'a> {
    pub html: &'a str,

    is_quote_start: bool,
    is_attribute_start: bool,
    is_get_attribute_name: bool,

    quote_char: u8,
    last_char: u8,
    last_index: usize,
    current_index: usize,
    value_start_index: usize,

    html_bytes: &'a [u8],
    html_len: usize,
}

impl<'a> HTMLTagAttributeIterator<'a> {
    fn new(html: &'a str) -> HTMLTagAttributeIterator<'a> {
        HTMLTagAttributeIterator {
            html: html,
            is_quote_start: false,
            is_attribute_start: false,
            is_get_attribute_name: false,
            quote_char: 0,
            last_char: 0,
            last_index: 0,
            current_index: 0,
            value_start_index: 0,
            html_bytes: html.as_bytes(),
            html_len: html.len()
        }
    }

    fn finished_item(&mut self) {
        self.is_attribute_start = false;
        self.is_quote_start = false;
        self.is_get_attribute_name = false;
        self.last_index = self.current_index - 1;
    }
}

impl<'a> Iterator for HTMLTagAttributeIterator<'a> {
    type Item = (Position, HTMLTagAttribute);

    fn next(&mut self) -> Option<(Position, HTMLTagAttribute)> {
        while self.current_index < self.html_len {
            let c = self.html_bytes[self.current_index];
            if self.current_index > 0 {
                self.last_char = self.html_bytes[self.current_index - 1];;
            }
            self.current_index += 1;

            if self.is_attribute_start {

                if self.is_get_attribute_name {

                    if self.is_quote_start {
                        if c == self.quote_char {
                            // only when match the same `quote` char
                            if c == self.quote_char {
                                let name = &self.html[self.last_index..(self.value_start_index - 1)];
                                let value = &self.html[(self.value_start_index + 1)..(self.current_index - 1)];
                                let position = Position { start: self.last_index, end: self.current_index };
                                let attribute = HTMLTagAttribute {
                                    name: name.to_string(),
                                    value: value.to_string(),
                                };
                                self.finished_item();
                                return Some((position, attribute));
                            }
                        } else {
                            continue;
                        }
                    }

                    // quote start
                    if b'\'' == c || b'"' == c {
                        // only when the last char is `equal`
                        if b'=' == self.last_char {
                            self.is_quote_start = true;
                            self.quote_char = c;
                        }
                        continue;
                    }

                    // only when match a `blank` char
                    if c <= b' ' {
                        let name = &self.html[self.last_index..(self.value_start_index - 1)];
                        let value = &self.html[(self.value_start_index)..(self.current_index - 1)];
                        let position = Position { start: self.last_index, end: self.current_index - 1 };
                        let attribute = HTMLTagAttribute {
                            name: name.to_string(),
                            value: value.to_string(),
                        };
                        self.finished_item();
                        return Some((position, attribute));
                    }

                } else {

                    // only when match an `equal` char, start the attribute value
                    if b'=' == c {
                        self.value_start_index = self.current_index;
                        self.is_get_attribute_name = true;
                        continue;
                    }

                    // only when match an `blank` char, stop current attribute
                    if c <= b' ' {
                        let name = &self.html[self.last_index..(self.current_index - 1)];
                        let position = Position { start: self.last_index, end: self.current_index - 1 };
                        let attribute = HTMLTagAttribute {
                            name: name.to_string(),
                            value: "".to_string(),
                        };
                        self.finished_item();
                        return Some((position, attribute));
                    }

                }

            } else {

                // ignore `blank` char
                if c <= b' ' {
                    continue;
                }

                self.is_attribute_start = true;
                self.is_get_attribute_name = false;
                self.is_quote_start = false;
                self.last_index = self.current_index - 1;

            }
        }

        // the rest text
        if self.current_index > 1 && self.last_index < self.current_index - 1 {
            let name = &self.html[self.last_index..];
            let position = Position { start: self.last_index, end: self.current_index };
            let attribute = HTMLTagAttribute {
                name: name.to_string(),
                value: "".to_string()
            };
            self.finished_item();
            return Some((position, attribute));
        }

        return None;
    }
}

/// Return a HTMLTagAttribute Iterator
pub fn attr_iter(html: &str) -> HTMLTagAttributeIterator {
    HTMLTagAttributeIterator::new(html)
}
