// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields under text input can be retrieved by using the methods under
//! `TextInput`.


use channel::TextInput;


impl TextInput {
    /// Get the title that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channel::TextInputBuilder;
    ///
    /// let title = "Enter Comment";
    /// let text_input = TextInputBuilder::new()
    ///     .title(title)
    ///     .finalize();
    /// assert_eq!(title.to_owned(), text_input.title());
    /// ```
    pub fn title(&self) -> String {
        self.title.clone()
    }


    /// Get the description that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channel::TextInputBuilder;
    ///
    /// let description = "Provided Feedback";
    /// let text_input = TextInputBuilder::new()
    ///     .description(description)
    ///     .finalize();
    /// assert_eq!(description.to_owned(), text_input.description());
    /// ```
    pub fn description(&self) -> String {
        self.description.clone()
    }


    /// Get the name that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channel::TextInputBuilder;
    ///
    /// let name = "Comment";
    /// let text_input = TextInputBuilder::new()
    ///     .name(name)
    ///     .finalize();
    /// assert_eq!(name.to_owned(), text_input.name());
    /// ```
    pub fn name(&self) -> String {
        self.name.clone()
    }


    /// Get the link that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channel::TextInputBuilder;
    ///
    /// let link = "www.example.com/feedback";
    /// let text_input = TextInputBuilder::new()
    ///     .link(link)
    ///     .finalize();
    /// assert_eq!(link.to_owned(), text_input.link());
    /// ```
    pub fn link(&self) -> String {
        self.link.clone()
    }
}
