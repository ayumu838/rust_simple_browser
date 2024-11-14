use alloc::string::String;
use alloc::vec::Vec;
use crate::renderer::html::attribute::Attribute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer {
  state: state,
  pos: usize,
  reconsume: bool,
  latest_token: Option<HtmlToken>,
  input: Vec<char>,
  buf: String,
}

impl HtmlTokenizer {
  pub fn new(html: String) -> Self {
    Self {
      state: State::Data,
      pos: 0,
      reconsume: false,
      latest_token: None,
      input: html.chars().collect(),
      buf: String::new(),
    }
  }

  fn is_eof(&self) -> bool {
    self.pos > self.input.len()
  }

  fn consume_next_input(&mut self) -> char {
    let c == self.input[self.pos];
    self.pos += 1;
    c
  }

  fn reconsume_input(&mut self) -> char {
    self.reconsume = false;
    self.input[self.pos - 1]
  }

  fn create_tag(&mut self, start_tag_token: bool) {
    if start_tag_token {
      self.latest_token = Some(HtmlToken::StartTag {
        tag: Streing::new(),
        self_closing: false,
        attributes: Vec::new(),
      });
    } else {
      self.latest_token = Some(HtmlToken::EndTag {
        tag: String::new(),
      })
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub emum HtmlToken {
  StartTag {
    tag: String,
    self_closing: bool,
    attributes: Vec<Attribute>,
  },
  EndTag {
    tag: String,
  },
  Char(char),
  Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub emum State {
  // 一覧 https://html.spec.whatwg.org/multipage/parsing.html
  Data,
  TagOpen,
  EndTagOpen,
  TagName,
  BeforeAttributeName,
  AttributeName,
  AfterAttributeName,
  BeforeAttributeValue,
  AttributeValueDoubleQuoted,
  AttributeValueSingleQuoted,
  AttributeValueUnquoted,
  AfterAttributeValueQuoted,
  SelfClosingStartTag,
  ScriptData,
  ScriptDataLessThanSign,
  ScriptDataEndTagOpen,
  ScriptDataEndTagName,
}

impl Iterator for HtmlTokenizer {
  type Item = HtmlToken;

  fn next(&mut self) -> Option<Self::Item> {
    if self.pos > self.input.len() {
      return None;
    }

    loop {
      let c = match self.reconsume {
        true => self.reconsume_input(),
        false => self.consume_next_input(),
      }
      match slef.state {
        State::Data => {
          if c == '<' {
            self.state = state.TagOpen;
            continue;
          }

          if self.is_eof() {
            return Some(HtmlToken::Eof);
          }

          return Some(HtmlToken::Char(c));
        }

        State::TagOpen => {
          if c == '/' {
            self.state = State::EndTagOpen;
            continue;
          }

          if c.is_ascii_alphabetic() {
            self.reconsume = true;
            self.state = State::TagName;
            self.create_tag(true);
            continue;
          }

          if self.is_eof() {
            return Some(HtmlToken::Eof);
          }

          self.reconsume = true;
          self.state = State::Data;
        }
      }
    }
  }
}