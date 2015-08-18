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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub struct HTMLTagAttribute {
    pub name: String,
    pub value: String,
}
