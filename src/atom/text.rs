


// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields under text can be retrieved by using the methods under `Text`.

use atom::Text;

impl Text {
    /// Get the text that exists under `Text`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::TextBuilder;
    ///
    /// fn main() {
    ///     let text_str = "This is a test";
    ///     let text = TextBuilder::new()
    ///         .text(text_str)
    ///         .finalize();
    ///     assert_eq!(text_str.to_owned(), text.text());
    /// }
    /// ```
    pub fn text(&self) -> String {
        self.text.clone()
    }


    /// Get the text_type that exists under `Text`.
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
    ///     let text = TextBuilder::new()
    ///         .text_type(Some(text_type.to_owned()))
    ///         .finalize();
    ///     assert_eq!(text_type.to_owned(), text.text_type());
    /// }
    /// ```
    pub fn text_type(&self) -> String {
        self.text_type.clone()
    }
}
