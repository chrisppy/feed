// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under text input can be retrieved by using the methods under
//! `TextInput`.


use channels::TextInputGetters;
use rss::TextInput;


impl TextInputGetters for TextInput
{
    /// Get the title that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::TextInputBuilder;
    ///
    /// let title = "Enter Comment";
    /// let text_input = TextInputBuilder::new()
    ///     .title(title)
    ///     .link("http://www.example.com/feedback")
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(title.to_owned(), text_input.title());
    /// ```
    fn title(&self) -> String
    {
        self.title.clone()
    }


    /// Get the description that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::TextInputBuilder;
    ///
    /// let description = "Provided Feedback";
    /// let text_input = TextInputBuilder::new()
    ///     .description(description)
    ///     .link("http://www.example.com/feedback")
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(description.to_owned(), text_input.description());
    /// ```
    fn description(&self) -> String
    {
        self.description.clone()
    }


    /// Get the name that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::TextInputBuilder;
    ///
    /// let name = "Comment";
    /// let text_input = TextInputBuilder::new()
    ///     .name(name)
    ///     .link("http://www.example.com/feedback")
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(name.to_owned(), text_input.name());
    /// ```
    fn name(&self) -> String
    {
        self.name.clone()
    }


    /// Get the link that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::TextInputBuilder;
    ///
    /// let link = "http://www.example.com/feedback";
    /// let text_input = TextInputBuilder::new()
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(link.to_owned(), text_input.link().into_string());
    /// ```
    fn link(&self) -> String
    {
        self.link.clone()
    }
}
