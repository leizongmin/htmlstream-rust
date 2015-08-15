#[derive(Debug)]
pub struct Position {
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub enum HTMLTagState {
    Text, Opening, Closing, SelfClosing
}

#[derive(Debug)]
pub struct HTMLTag {
    name: String,
    html: String,
    attributes: String,
    state: HTMLTagState
}

#[derive(Debug)]
pub struct HTMLTagAttribute {
    name: String,
    value: String,
}

const CHAR_SINGLE_QUOTE: u8 = b'\'';
const CHAR_DOUBLE_QUOTE: u8 = b'"';
const CHAR_LT: u8 = b'<';
const CHAR_GT: u8 = b'>';
const CHAR_SLASH: u8 = b'/';
const CHAR_SPACE: u8 = b' ';


pub fn parse_html<F>(html: &str, on_tag: F) where F: Fn(&Position, &HTMLTag) {
    let mut is_tag_start: bool = false;
    let mut is_quote_start: bool = false;
    let mut is_get_tag_name: bool = false;
    let mut is_closing_tag: bool = false;
    let mut current_tag_name: &str = "";

    let mut quote_char: u8 = 0;
    let mut last_index: usize = 0;
    let mut current_index: usize = 0;
    let mut attributes_start_index: usize = 0;

    let html_bytes = html.as_bytes();
    for b in html_bytes {
        let c = *b;
        current_index += 1;

        if is_tag_start {

            // 获取标签名称
            if !is_get_tag_name {
                if CHAR_SLASH == c && last_index + 2 == current_index {
                    is_closing_tag = true;
                } else {
                    if CHAR_SPACE == c || CHAR_SLASH == c || CHAR_GT == c || CHAR_LT == c {
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

            // 如果当前字符出现在引号内，只有遇到相同的引号才能结束
            if is_quote_start {
                if c == quote_char {
                    is_quote_start = false;
                    continue;
                } else {
                    continue;
                }
            }

            // 引号开始
            if CHAR_SINGLE_QUOTE == c || CHAR_DOUBLE_QUOTE == c {
                is_quote_start = true;
                quote_char = c;
                continue;
            }

            // 标签结束
            if CHAR_GT == c {
                // 触发新标签
                let tag_html = &html[last_index..current_index];
                let tag_bytes = tag_html.as_bytes();
                let position = Position { start: last_index, end: current_index };

                let tag_state: HTMLTagState;
                if CHAR_SLASH == tag_bytes[1] {
                    tag_state = HTMLTagState::Closing;
                } else if CHAR_SLASH == tag_bytes[tag_bytes.len() - 2] {
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
                // 中间的文本
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

                // 重新初始化
                is_tag_start = true;
                is_get_tag_name = false;
                is_closing_tag = false;
                last_index = current_index - 1;
                continue;
            }

        }
    }

    // 剩余部分的文本
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
}

pub fn parse_attribute<F>(html: &String, on_tag_attribute: F) where F: Fn(&Position, &HTMLTagAttribute) {

}


#[test]
fn test_parse_html() {
    parse_html("<b>这里是html: <a href='#' title=ok>link</a> yes <b /> goosa", |pos: &Position, tag: &HTMLTag| {
        println!("{:?} {:?}", pos, tag);
        //println!("{:?}", tag.html);
    });
}
