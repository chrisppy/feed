// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for text by using the methods under `TextBuilder`.

use atom::{Text, TextBuilder};
use errors;

impl TextBuilder {
    /// Construct a new `TextBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::TextBuilder;
    ///
    /// let text_builder = TextBuilder::new();
    /// ```
    pub fn new() -> TextBuilder {
        TextBuilder::default()
    }


    /// Set the text that exists under `text`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::TextBuilder;
    ///
    /// fn main() {
    ///     let mut text_builder = TextBuilder::new();
    ///     text_builder.text("This is a test");
    /// }
    /// ```
    pub fn text(&mut self, text: &str) -> &mut TextBuilder {
        self.text = text.to_owned();
        self
    }


    /// Set the text_type that exists under `text`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::TextBuilder;
    ///
    /// fn main() {
    ///     let text_type = "text";
    ///     let mut text_builder = TextBuilder::new();
    ///     text_builder.text_type(Some(text_type.to_owned()));
    /// }
    /// ```
    pub fn text_type(&mut self, text_type: Option<String>) -> &mut TextBuilder {
        let text_type_str = text_type.clone().unwrap();
        match text_type_str.as_str() {
            "text" => (),
            "html" => (),
            "xhtml" => (),
            _ => panic!(errors::invalid_str_error("text_type", text_type_str.as_str())),
        }
        self.text_type = text_type.clone();
        self
    }


    /// Construct the `Text` from the `TextBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::TextBuilder;
    ///
    /// fn main() {
    ///     let text = TextBuilder::new()
    ///         .text("This is a test")
    ///         .text_type(Some("text".to_owned()))
    ///         .finalize();
    /// }
    /// ```
    pub fn finalize(&self) -> Text {
        if self.text.is_empty() {
            panic!(errors::empty_string_error("Atom text"));
        }

        let mut text_type = self.text_type.clone();
        if text_type.is_none() {
            text_type = Some("text".to_owned());
        }
        Text {
            text: self.text.clone(),
            text_type: text_type.unwrap(),
        }
    }
}
