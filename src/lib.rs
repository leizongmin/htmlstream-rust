#![crate_name = "htmlstream"]
#![doc(html_root_url = "https://leizongmin.github.io/htmlstream-rust/")]


/// The HTML source position
///
/// # Examples
/// ```
/// # use self::htmlstream::*;
/// let pos = Position {
///     start: 0,
///     end: 10,
/// };
/// ```
#[derive(Debug)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

/// The tag state
///
/// + `Text`: not a HTML tag, e.g. hello
/// + `Opening`: an opening tag, e.g. <a href="#">
/// + `Closing`: a closing tag, e.g. </a>
/// + `SelfClosing`: a selfclosing tag,e.g. <br />
#[derive(Debug)]
pub enum HTMLTagState {
    Text, Opening, Closing, SelfClosing
}

/// The HTML tag
///
/// # Examples
///
/// ```
/// # use self::htmlstream::*;
/// let tag = HTMLTag {
///     name: "a".to_string(),
///     html: "<a href=\"#\">link</a>".to_string(),
///     attributes: "href=\"#\"".to_string(),
///     state: HTMLTagState::Opening,
/// };
/// ```
#[derive(Debug)]
pub struct HTMLTag {
    pub name: String,
    pub html: String,
    pub attributes: String,
    pub state: HTMLTagState
}

/// The tag attribute
///
/// # Examples
///
/// ```
/// # use self::htmlstream::*;
/// let attr = HTMLTagAttribute {
///     name: "href".to_string(),
///     value: "#".to_string(),
/// };
/// ```
#[derive(Debug)]
pub struct HTMLTagAttribute {
    pub name: String,
    pub value: String,
}

const CHAR_SINGLE_QUOTE: u8 = b'\'';
const CHAR_DOUBLE_QUOTE: u8 = b'"';
const CHAR_LT: u8 = b'<';
const CHAR_GT: u8 = b'>';
const CHAR_SLASH: u8 = b'/';
const CHAR_SPACE: u8 = b' ';
const CHAR_EQUAL: u8 = b'=';


/// Parse HTML source, find the tags and call `on_tag()` every time
///
/// # Examples
///
/// ```
/// # use self::htmlstream::*;
/// let html = "this is a test: <a href=\"http://rust-lang.org\">The Rust Programing Language</a>";
/// parse_html(html, |pos: &Position, tag: &HTMLTag| {
///     println!("{:?} {:?}", pos, tag);
/// });
/// ```
pub fn parse_html<F>(html: &str, on_tag: F) where F: Fn(&Position, &HTMLTag) {
    let mut is_tag_start: bool = false;
    let mut is_quote_start: bool = false;
    let mut is_get_tag_name: bool = false;
    let mut is_closing_tag: bool = false;
    let mut current_tag_name: &str = "";

    let mut quote_char: u8 = 0;
    let mut last_char: u8 = 0;
    let mut last_index: usize = 0;
    let mut current_index: usize = 0;
    let mut attributes_start_index: usize = 0;

    let html_bytes = html.as_bytes();
    for b in html_bytes {
        let c = *b;
        if current_index > 0 {
            last_char = html_bytes[current_index - 1];;
        }
        current_index += 1;

        if is_tag_start {

            if !is_get_tag_name {
                if CHAR_SLASH == c && last_index + 2 == current_index {
                    is_closing_tag = true;
                } else {
                    if c <= CHAR_SPACE || CHAR_SLASH == c || CHAR_GT == c || CHAR_LT == c {
                        if is_closing_tag {
                            current_tag_name = &html[(last_index + 2)..(current_index - 1)];
                        } else {
                            current_tag_name = &html[(last_index + 1)..(current_index - 1)];
                        }
                        attributes_start_index = current_index;
                        is_get_tag_name = true;
                    }
                }
            }

            // only when match the same `quote` char
            if is_quote_start {
                if c == quote_char {
                    is_quote_start = false;
                    continue;
                } else {
                    continue;
                }
            }

            // quote start
            if CHAR_SINGLE_QUOTE == c || CHAR_DOUBLE_QUOTE == c {
                // only when the last char is `equal`
                if CHAR_EQUAL == last_char {
                    is_quote_start = true;
                    quote_char = c;
                }
                continue;
            }

            // tag end
            if CHAR_GT == c {
                let tag_html = &html[last_index..current_index];
                let position = Position { start: last_index, end: current_index };

                let tag_state: HTMLTagState;
                if is_closing_tag {
                    tag_state = HTMLTagState::Closing;
                } else if CHAR_SLASH == last_char {
                    tag_state = HTMLTagState::SelfClosing;
                } else {
                    tag_state = HTMLTagState::Opening;
                }

                let attributes_html: &str;
                if is_get_tag_name && attributes_start_index < current_index - 1 {
                    match tag_state {
                        HTMLTagState::SelfClosing => {
                            attributes_html = &html[attributes_start_index..(current_index - 2)];
                        },
                        HTMLTagState::Opening | HTMLTagState::Closing => {
                            attributes_html = &html[attributes_start_index..(current_index - 1)];
                        },
                        _ => {
                            attributes_html = "";
                        }
                    }
                } else {
                    attributes_html = "";
                }

                let tag = HTMLTag {
                    name: current_tag_name.to_string(),
                    html: tag_html.to_string(),
                    attributes: attributes_html.to_string(),
                    state: tag_state,
                };
                on_tag(&position, &tag);

                last_index = current_index;
                is_tag_start = false;
            }

        } else {

            if CHAR_LT == c {
                // text
                if last_index < current_index - 1 {
                    let tag_html = &html[last_index..(current_index - 1)];
                    let position = Position { start: last_index, end: current_index };
                    let tag = HTMLTag {
                        name: "".to_string(),
                        html: tag_html.to_string(),
                        attributes: "".to_string(),
                        state: HTMLTagState::Text,
                    };
                    on_tag(&position, &tag);
                }

                // init
                is_tag_start = true;
                is_get_tag_name = false;
                is_closing_tag = false;
                is_quote_start = false;
                last_index = current_index - 1;
                continue;
            }

        }
    }

    // the rest text
    if last_index < current_index - 1 {
        let tag_html = &html[last_index..];
        let position = Position { start: last_index, end: current_index };
        let tag = HTMLTag {
            name: "".to_string(),
            html: tag_html.to_string(),
            attributes: "".to_string(),
            state: HTMLTagState::Text,
        };
        on_tag(&position, &tag);
    }
}


/// Parse attributes strings, find the attribute and call `on_tag_attribute()` every time
///
/// # Examples
///
/// ```
/// # use self::htmlstream::*;
/// let attributes = "href=\"http://rust-lang.org\" title=Rust disabled";
/// parse_attributes(attributes, |pos: &Position, attr: &HTMLTagAttribute| {
///     println!("{:?} {:?}", pos, attr);
/// });
/// ```
pub fn parse_attributes<F>(html: &str, on_tag_attribute: F) where F: Fn(&Position, &HTMLTagAttribute) {
    let mut is_quote_start: bool = false;
    let mut is_attribute_start: bool = false;
    let mut is_get_attribute_name: bool = false;

    let mut quote_char: u8 = 0;
    let mut last_char: u8 = 0;
    let mut last_index: usize = 0;
    let mut current_index: usize = 0;
    let mut value_start_index: usize = 0;

    let html_bytes = html.as_bytes();
    for b in html_bytes {
        let c = *b;
        if current_index > 0 {
            last_char = html_bytes[current_index - 1];;
        }
        current_index += 1;

        if is_attribute_start {

            if is_get_attribute_name {

                if is_quote_start {
                    if c == quote_char {
                        // only when match the same `quote` char
                        if c == quote_char {
                            let name = &html[last_index..(value_start_index - 1)];
                            let value = &html[(value_start_index + 1)..(current_index - 1)];
                            let position = Position { start: last_index, end: current_index };
                            let attribute = HTMLTagAttribute {
                                name: name.to_string(),
                                value: value.to_string(),
                            };
                            on_tag_attribute(&position, &attribute);

                            is_attribute_start = false;
                            is_quote_start = false;
                            is_get_attribute_name = false;
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                // quote start
                if CHAR_SINGLE_QUOTE == c || CHAR_DOUBLE_QUOTE == c {
                    // only when the last char is `equal`
                    if CHAR_EQUAL == last_char {
                        is_quote_start = true;
                        quote_char = c;
                    }
                    continue;
                }

                // only when match a `blank` char
                if c <= CHAR_SPACE {
                    let name = &html[last_index..(value_start_index - 1)];
                    let value = &html[(value_start_index)..(current_index - 1)];
                    let position = Position { start: last_index, end: current_index };
                    let attribute = HTMLTagAttribute {
                        name: name.to_string(),
                        value: value.to_string(),
                    };
                    on_tag_attribute(&position, &attribute);

                    is_attribute_start = false;
                    is_quote_start = false;
                    is_get_attribute_name = false;
                    continue;
                }

            } else {

                // only when match an `equal` char, start the attribute value
                if CHAR_EQUAL == c {
                    value_start_index = current_index;
                    is_get_attribute_name = true;
                    continue;
                }

                // only when match an `blank` char, stop current attribute
                if c <= CHAR_SPACE {
                    let name = &html[last_index..(current_index - 1)];
                    let position = Position { start: last_index, end: current_index };
                    let attribute = HTMLTagAttribute {
                        name: name.to_string(),
                        value: "".to_string(),
                    };
                    on_tag_attribute(&position, &attribute);

                    is_attribute_start = false;
                    is_quote_start = false;
                    is_get_attribute_name = false;
                    continue;
                }

            }

        } else {

            // ignore `blank` char
            if c <= CHAR_SPACE {
                continue;
            }

            is_attribute_start = true;
            is_get_attribute_name = false;
            is_quote_start = false;
            last_index = current_index - 1;

        }
    }

    // the rest text
    let name = &html[last_index..];
    let position = Position { start: last_index, end: current_index };
    let attribute = HTMLTagAttribute {
        name: name.to_string(),
        value: "".to_string()
    };
    on_tag_attribute(&position, &attribute);
}
