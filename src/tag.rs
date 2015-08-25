use super::*;

#[derive(Debug)]
pub struct HTMLTagIterator<'a> {
    pub html: &'a str,

    is_tag_start: bool,
    is_quote_start: bool,
    is_get_tag_name: bool,
    is_closing_tag: bool,
    current_tag_name: &'a str,

    quote_char: u8,
    last_char: u8,
    last_index: usize,
    current_index: usize,
    attributes_start_index: usize,

    html_bytes: &'a [u8],
    html_len: usize,
}

impl<'a> HTMLTagIterator<'a> {
    fn new(html: &'a str) -> HTMLTagIterator<'a> {
        HTMLTagIterator {
            html: html,
            is_tag_start: false,
            is_quote_start: false,
            is_get_tag_name: false,
            is_closing_tag: false,
            current_tag_name: "",
            quote_char: 0,
            last_char: 0,
            last_index: 0,
            current_index: 0,
            attributes_start_index: 0,
            html_bytes: html.as_bytes(),
            html_len: html.len()
        }
    }
}

impl<'a> Iterator for HTMLTagIterator<'a> {
    type Item = (Position, HTMLTag);

    fn next(&mut self) -> Option<(Position, HTMLTag)> {
        while self.current_index < self.html_len {
            let c = self.html_bytes[self.current_index];
            if self.current_index > 0 {
                self.last_char = self.html_bytes[self.current_index - 1];;
            }
            self.current_index += 1;

            if self.is_tag_start {

                if !self.is_get_tag_name {
                    if b'/' == c && self.last_index + 2 == self.current_index {
                        self.is_closing_tag = true;
                    } else {
                        if c <= b' ' || b'/' == c || b'>' == c || b'<' == c {
                            if self.is_closing_tag {
                                self.current_tag_name = &self.html[(self.last_index + 2)..(self.current_index - 1)];
                            } else {
                                self.current_tag_name = &self.html[(self.last_index + 1)..(self.current_index - 1)];
                            }
                            self.attributes_start_index = self.current_index;
                            self.is_get_tag_name = true;
                        }
                    }
                }

                // only when match the same `quote` char
                if self.is_quote_start {
                    if c == self.quote_char {
                        self.is_quote_start = false;
                        continue;
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

                // tag end
                if b'>' == c {
                    let tag_html = &self.html[self.last_index..self.current_index];
                    let position = Position { start: self.last_index, end: self.current_index };

                    let tag_state: HTMLTagState;
                    if self.is_closing_tag {
                        tag_state = HTMLTagState::Closing;
                    } else if b'/' == self.last_char {
                        tag_state = HTMLTagState::SelfClosing;
                    } else {
                        tag_state = HTMLTagState::Opening;
                    }

                    let attributes_html: &str;
                    if self.is_get_tag_name && self.attributes_start_index < self.current_index - 1 {
                        match tag_state {
                            HTMLTagState::SelfClosing => {
                                attributes_html = &self.html[self.attributes_start_index..(self.current_index - 2)];
                            },
                            HTMLTagState::Opening | HTMLTagState::Closing => {
                                attributes_html = &self.html[self.attributes_start_index..(self.current_index - 1)];
                            },
                            _ => {
                                attributes_html = "";
                            }
                        }
                    } else {
                        attributes_html = "";
                    }

                    let tag = HTMLTag {
                        name: self.current_tag_name.to_string(),
                        html: tag_html.to_string(),
                        attributes: attributes_html.to_string(),
                        state: tag_state,
                    };
                    self.last_index = self.current_index;
                    self.is_tag_start = false;
                    return Some((position, tag));
                }

            } else {

                if b'<' == c {
                    let last_index = self.last_index;
                    self.is_tag_start = true;
                    self.is_get_tag_name = false;
                    self.is_closing_tag = false;
                    self.is_quote_start = false;
                    self.last_index = self.current_index - 1;

                    // text
                    if last_index < self.current_index - 1 {
                        let tag_html = &self.html[last_index..(self.current_index - 1)];
                        let position = Position { start: last_index, end: self.current_index - 1 };
                        let tag = HTMLTag {
                            name: "".to_string(),
                            html: tag_html.to_string(),
                            attributes: "".to_string(),
                            state: HTMLTagState::Text,
                        };
                        return Some((position, tag));
                    }
                }

            }
        }

        // the rest text
        if self.current_index > 1 && self.last_index < self.current_index - 1 {
            let tag_html = &self.html[self.last_index..];
            let position = Position { start: self.last_index, end: self.current_index };
            let tag = HTMLTag {
                name: "".to_string(),
                html: tag_html.to_string(),
                attributes: "".to_string(),
                state: HTMLTagState::Text,
            };
            return Some((position, tag));
        }

        return None;
    }
}

/// Return a HTMLTag Iterator
pub fn tag_iter(html: &str) -> HTMLTagIterator {
    HTMLTagIterator::new(html)
}
