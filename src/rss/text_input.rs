// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

/// This `TextInput` struct contains all the items that exist for the text input field under 'Channel'.
#[derive(Clone)]
pub struct TextInput {
    title:       String,
    description: String,
    name:        String,
    link:        String,
}


impl TextInput {
    /// Get the title that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let title = "Enter Comment";
    /// let text_input = TextInputBuilder::new()
    ///     .title(title)
    ///     .finalize();
    /// assert_eq!(title.to_string(), text_input.title());
    /// ```
    pub fn title(&self) -> String {
        self.title.clone()
    }


    /// Get the description that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let description = "Provided Feedback";
    /// let text_input = TextInputBuilder::new()
    ///     .description(description)
    ///     .finalize();
    /// assert_eq!(description.to_string(), text_input.description());
    /// ```
    pub fn description(&self) -> String {
        self.description.clone()
    }


    /// Get the name that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let name = "Comment";
    /// let text_input = TextInputBuilder::new()
    ///     .name(name)
    ///     .finalize();
    /// assert_eq!(name.to_string(), text_input.name());
    /// ```
    pub fn name(&self) -> String {
        self.name.clone()
    }


    /// Get the link that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let link = "www.example.com/feedback";
    /// let text_input = TextInputBuilder::new()
    ///     .link(link)
    ///     .finalize();
    /// assert_eq!(link.to_string(), text_input.link());
    /// ```
    pub fn link(&self) -> String {
        self.link.clone()
    }
}


/// This `TextInputBuilder` struct creates the `TextInput`.
pub struct TextInputBuilder {
    title:       String,
    description: String,
    name:        String,
    link:        String,
}


impl TextInputBuilder {
    /// Construct a new `TextInputBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let text_input_builder = TextInputBuilder::new();
    /// ```
    pub fn new() -> TextInputBuilder {
        TextInputBuilder {
            title:       String::new(),
            description: String::new(),
            name:        String::new(),
            link:        String::new(),
        }
    }


    /// Set the title that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.title("Title");
    /// ```
    pub fn title(&mut self, title: &str) -> &mut TextInputBuilder {
        self.title = title.to_string();
        self
    }


    /// Set the description that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.description("This is a test description.");
    /// ```
    pub fn description(&mut self, description: &str) -> &mut TextInputBuilder {
        self.description = description.to_string();
        self
    }


    /// Set the name that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.name("Comments");
    /// ```
    pub fn name(&mut self, name: &str) -> &mut TextInputBuilder {
        self.name = name.to_string();
        self
    }


    /// Set the link that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.link("http://www.example.com/feedback");
    /// ```
    pub fn link(&mut self, link: &str) -> &mut TextInputBuilder {
        self.link = link.to_string();
        self
    }


    /// Construct the `TextInput` from the `TextInputBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let text_input = TextInputBuilder::new()
    ///         .title("Title")
    ///         .description("This is a test description.")
    ///         .name("Comments")
    ///         .link("http://www.example.com/feedback")
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> TextInput {
        TextInput {
            title:       self.title.clone(),
            description: self.description.clone(),
            name:        self.name.clone(),
            link:        self.link.clone(),
        }
    }
}
