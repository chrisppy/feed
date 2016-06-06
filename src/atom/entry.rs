
// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for entry by using the methods under `EntryBuilder`.

use atom::{AtomCategory, Entry, Link, Person, Text};
use chrono::*;
use url::Url;

impl Entry {
    /// Get the id that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::EntryBuilder;
    ///
    /// fn main() {
    ///     let id = "http://newrustacean.com/show_notes/e014/";
    ///     let entry = EntryBuilder::new()
    ///         .id(id)
    ///         .title("e014: Stringing things along")
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .finalize();
    ///     assert_eq!(id.to_owned(), entry.id().into_string())
    /// }
    /// ```
    pub fn id(self) -> Url {
        self.id
    }


    /// Get the title that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::EntryBuilder;
    ///
    /// fn main() {
    ///     let title = "e014: Stringing things along";
    ///     let entry = EntryBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(title)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .finalize();
    ///     assert_eq!(title.to_owned(), entry.title())
    /// }
    /// ```
    pub fn title(self) -> String {
        self.title
    }


    /// Get the updated that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::EntryBuilder;
    ///
    /// fn main() {
    ///     let updated = "2014-11-28T12:00:09+00:00";
    ///     let entry = EntryBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title("e014: Stringing things along")
    ///         .updated(updated)
    ///         .finalize();
    ///     assert_eq!(updated.to_owned(), entry.updated().to_rfc3339())
    /// }
    /// ```
    pub fn updated(self) -> DateTime<FixedOffset> {
        self.updated
    }


    /// Get the authors that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, PersonBuilder};
    ///
    /// fn main() {
    ///     let name = "Chris Krycho";
    ///     let authors = Some(vec![PersonBuilder::new().name(name).finalize()]);
    ///     let entry = EntryBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title("e014: Stringing things along")
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .authors(authors)
    ///         .finalize();
    ///     assert_eq!(name.to_owned(), entry.authors().unwrap()[0].name())
    /// }
    /// ```
    pub fn authors(self) -> Option<Vec<Person>> {
        self.authors
    }


    /// Get the content that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let text = "This is an example!";
    ///     let content = TextBuilder::new().text(text).finalize();
    ///     let entry = EntryBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title("e014: Stringing things along")
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .content(Some(content))
    ///         .finalize();
    ///     assert_eq!(text.to_owned(), entry.content().unwrap().text())
    /// }
    /// ```
    pub fn content(self) -> Option<Text> {
        self.content
    }


    /// Get the links that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, LinkBuilder};
    ///
    /// fn main() {
    ///     let href = "http://www.example.com/";
    ///     let links = Some(vec![LinkBuilder::new().href(href).finalize()]);
    ///     let entry = EntryBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title("e014: Stringing things along")
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .links(links)
    ///         .finalize();
    ///     assert_eq!(href.to_owned(), entry.links().unwrap()[0].href().into_string())
    /// }
    /// ```
    pub fn links(self) -> Option<Vec<Link>> {
        self.links
    }


    /// Get the summary that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let text = "This is an example!";
    ///     let summary = TextBuilder::new().text(text).finalize();
    ///     let entry = EntryBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title("e014: Stringing things along")
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .summary(Some(summary))
    ///         .finalize();
    ///     assert_eq!(text.to_owned(), entry.summary().unwrap().text())
    /// }
    /// ```
    pub fn summary(self) -> Option<Text> {
        self.summary
    }


    /// Get the categories that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, AtomCategoryBuilder};
    ///
    /// fn main() {
    ///     let term = "Tech";
    ///     let categories = Some(vec![AtomCategoryBuilder::new().term(term).finalize()]);
    ///     let entry = EntryBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title("e014: Stringing things along")
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .categories(categories)
    ///         .finalize();
    ///     assert_eq!(term.to_owned(), entry.categories().unwrap()[0].term())
    /// }
    /// ```
    pub fn categories(self) -> Option<Vec<AtomCategory>> {
        self.categories
    }


    /// Get the contributors that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, PersonBuilder};
    ///
    /// fn main() {
    ///     let name = "Chris Krycho";
    ///     let contributors = Some(vec![PersonBuilder::new().name(name).finalize()]);
    ///     let entry = EntryBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title("e014: Stringing things along")
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .contributors(contributors)
    ///         .finalize();
    ///     assert_eq!(name.to_owned(), entry.contributors().unwrap()[0].name())
    /// }
    /// ```
    pub fn contributors(self) -> Option<Vec<Person>> {
        self.contributors
    }


    /// Get the published that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::EntryBuilder;
    ///
    /// fn main() {
    ///     let date = "2014-11-28T12:00:09+00:00";
    ///     let entry = EntryBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title("e014: Stringing things along")
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .published(Some(date.to_owned()))
    ///         .finalize();
    ///     assert_eq!(date.to_owned(), entry.published().unwrap().to_rfc3339())
    /// }
    /// ```
    pub fn published(self) -> Option<DateTime<FixedOffset>> {
        self.published
    }


    /// Get the source that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::EntryBuilder;
    ///
    /// fn main() {
    ///     let mut entry_builder = EntryBuilder::new();
    ///     entry_builder.source(Some("http://sixlegs.com/blog/atom.xml".to_owned()));
    /// }
    /// ```
    pub fn source(self) -> Option<String> {
        self.source
    }


    /// Get the rights that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let text = "© 2005 John Doe";
    ///     let rights = TextBuilder::new().text(text).finalize();
    ///     let entry = EntryBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title("e014: Stringing things along")
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .rights(Some(rights))
    ///         .finalize();
    ///     assert_eq!(text.to_owned(), entry.rights().unwrap().text())
    /// }
    /// ```
    pub fn rights(self) -> Option<Text> {
        self.rights
    }
}
