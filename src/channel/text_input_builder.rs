// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for text input by using the methods
//! under `TextInputBuilder`.


use TextInputBuilder;
use rss::TextInput;
use utils::string_utils;


impl TextInputBuilder
{
    /// Construct a new `TextInputBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::TextInputBuilder;
    ///
    /// let text_input_builder = TextInputBuilder::new();
    /// ```
    pub fn new() -> TextInputBuilder
    {
        TextInputBuilder::default()
    }


    /// Set the title that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.title("Title");
    /// ```
    pub fn title(&mut self, title: &str) -> &mut TextInputBuilder
    {
        self.title = title.to_owned();
        self
    }


    /// Set the description that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.description("This is a test description.");
    /// ```
    pub fn description(&mut self, description: &str) -> &mut TextInputBuilder
    {
        self.description = description.to_owned();
        self
    }


    /// Set the name that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.name("Comments");
    /// ```
    pub fn name(&mut self, name: &str) -> &mut TextInputBuilder
    {
        self.name = name.to_owned();
        self
    }


    /// Set the link that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.link("http://www.example.com/feedback");
    /// ```
    pub fn link(&mut self, link: &str) -> &mut TextInputBuilder
    {
        self.link = link.to_owned();
        self
    }


    /// Validate the contents of `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::TextInputBuilder;
    ///
    /// let text_input = TextInputBuilder::new()
    ///         .title("Title")
    ///         .description("This is a test description.")
    ///         .name("Comments")
    ///         .link("http://www.example.com/feedback")
    ///         .validate().unwrap()
    ///         .finalize().unwrap();
    /// ```
    pub fn validate(&mut self) -> Result<&mut TextInputBuilder, String>
    {
        string_utils::str_to_url(self.link.clone().as_str())?;

        Ok(self)
    }


    /// Construct the `TextInput` from the `TextInputBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::TextInputBuilder;
    ///
    /// let text_input = TextInputBuilder::new()
    ///         .title("Title")
    ///         .description("This is a test description.")
    ///         .name("Comments")
    ///         .link("http://www.example.com/feedback")
    ///         .finalize()
    ///         .unwrap();
    /// ```
    pub fn finalize(&self) -> Result<TextInput, String>
    {
        Ok(TextInput {
               title: self.title.clone(),
               description: self.description.clone(),
               name: self.name.clone(),
               link: self.link.clone(),
           })
    }
}
