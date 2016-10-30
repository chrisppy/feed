// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields can be set for text input by using the methods
//! under `TextInputBuilder`.


use channels::{TextInput, TextInputBuilder};
use utils::string_utils;


impl TextInputBuilder {
    /// Construct a new `TextInputBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::TextInputBuilder;
    ///
    /// let text_input_builder = TextInputBuilder::new();
    /// ```
    pub fn new() -> TextInputBuilder {
        TextInputBuilder::default()
    }


    /// Set the title that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.title("Title");
    /// ```
    pub fn title(&mut self, title: &str) -> &mut TextInputBuilder {
        self.title = title.to_owned();
        self
    }


    /// Set the description that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.description("This is a test description.");
    /// ```
    pub fn description(&mut self, description: &str) -> &mut TextInputBuilder {
        self.description = description.to_owned();
        self
    }


    /// Set the name that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.name("Comments");
    /// ```
    pub fn name(&mut self, name: &str) -> &mut TextInputBuilder {
        self.name = name.to_owned();
        self
    }


    /// Set the link that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.link("http://www.example.com/feedback");
    /// ```
    pub fn link(&mut self, link: &str) -> &mut TextInputBuilder {
        self.link = link.to_owned();
        self
    }


    /// Construct the `TextInput` from the `TextInputBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::TextInputBuilder;
    ///
    /// let text_input = TextInputBuilder::new()
    ///         .title("Title")
    ///         .description("This is a test description.")
    ///         .name("Comments")
    ///         .link("http://www.example.com/feedback")
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> TextInput {
        let link_string = self.link.clone();
        let link = string_utils::str_to_url(link_string.as_str(), "TextInput Link");

        TextInput {
            title: self.title.clone(),
            description: self.description.clone(),
            name: self.name.clone(),
            link: link,
        }
    }
}
