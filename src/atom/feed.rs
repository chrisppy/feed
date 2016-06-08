// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for feed by using the methods under `AtomFeedBuilder`.

use atom::{AtomFeed, AtomCategory, Entry, Generator, Link, Person, Text};
use chrono::*;
use url::Url;

impl AtomFeed {
    /// Get the id that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let id = "http://newrustacean.com/show_notes/e014/";
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id(id)
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .finalize();
    ///     assert_eq!(id.to_owned(), feed.id().into_string())
    /// }
    /// ```
    pub fn id(self) -> Url {
        self.id
    }


    /// Get the title that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .finalize();
    ///     assert_eq!(title.to_owned(), feed.title().text())
    /// }
    /// ```
    pub fn title(self) -> Text {
        self.title
    }


    /// Get the updated that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let updated = "2014-11-28T12:00:09+00:00";
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated(updated)
    ///         .finalize();
    ///     assert_eq!(updated.to_owned(), feed.updated().to_rfc3339())
    /// }
    /// ```
    pub fn updated(self) -> DateTime<FixedOffset> {
        self.updated
    }


    /// Get the authors that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, PersonBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let name = "Chris Krycho";
    ///     let authors = Some(vec![PersonBuilder::new().name(name).finalize()]);
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .authors(authors)
    ///         .finalize();
    ///     assert_eq!(name.to_owned(), feed.authors().unwrap()[0].name())
    /// }
    /// ```
    pub fn authors(self) -> Option<Vec<Person>> {
        self.authors
    }


    /// Get the links that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, LinkBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let href = "http://www.example.com/";
    ///     let links = Some(vec![LinkBuilder::new().href(href).finalize()]);
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .links(links)
    ///         .finalize();
    ///     assert_eq!(href.to_owned(), feed.links().unwrap()[0].href().into_string())
    /// }
    /// ```
    pub fn links(self) -> Option<Vec<Link>> {
        self.links
    }


    /// Get the categories that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, AtomCategoryBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let term = "Tech";
    ///     let categories = Some(vec![AtomCategoryBuilder::new().term(term).finalize()]);
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .categories(categories)
    ///         .finalize();
    ///     assert_eq!(term.to_owned(), feed.categories().unwrap()[0].term())
    /// }
    /// ```
    pub fn categories(self) -> Option<Vec<AtomCategory>> {
        self.categories
    }


    /// Get the contributors that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, PersonBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let name = "Chris Krycho";
    ///     let contributors = Some(vec![PersonBuilder::new().name(name).finalize()]);
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .contributors(contributors)
    ///         .finalize();
    ///     assert_eq!(name.to_owned(), feed.contributors().unwrap()[0].name())
    /// }
    /// ```
    pub fn contributors(self) -> Option<Vec<Person>> {
        self.contributors
    }


    /// Get the generator that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, GeneratorBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let gen = "Example Toolkit";
    ///     let generator = GeneratorBuilder::new().generator(gen).finalize();
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .generator(Some(generator))
    ///         .finalize();
    ///     assert_eq!(gen.to_owned(), feed.generator().unwrap().generator())
    /// }
    /// ```
    pub fn generator(self) -> Option<Generator> {
        self.generator
    }


    /// Get the icon that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let url = "http://example.com/image.png";
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .icon(Some(url.to_owned()))
    ///         .finalize();
    ///     assert_eq!(url.to_owned(), feed.icon().unwrap().into_string())
    /// }
    /// ```
    pub fn icon(self) -> Option<Url> {
        self.icon
    }


    /// Get the logo that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let url = "http://example.com/image.png";
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .logo(Some(url.to_owned()))
    ///         .finalize();
    ///     assert_eq!(url.to_owned(), feed.logo().unwrap().into_string())
    /// }
    /// ```
    pub fn logo(self) -> Option<Url> {
        self.logo
    }


    /// Get the rights that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let text = "© 2005 John Doe";
    ///     let rights = TextBuilder::new().text(text).finalize();
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .rights(Some(rights))
    ///         .finalize();
    ///     assert_eq!(text.to_owned(), feed.rights().unwrap().text())
    /// }
    /// ```
    pub fn rights(self) -> Option<Text> {
        self.rights
    }


    /// Get the subtitle that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let text = "Example";
    ///     let subtitle = TextBuilder::new().text(text).finalize();
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .subtitle(Some(subtitle))
    ///         .finalize();
    ///     assert_eq!(text.to_owned(), feed.subtitle().unwrap().text())
    /// }
    /// ```
    pub fn subtitle(self) -> Option<Text> {
        self.subtitle
    }


    /// Get the entries that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, Entry, EntryBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let id = "http://newrustacean.com/show_notes/e014/";
    ///
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let entry = EntryBuilder::new()
    ///         .id(id)
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .finalize();
    ///     let feed = AtomFeedBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title(text)
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .entries(Some(vec![entry]))
    ///         .finalize();
    ///     let feed_entries: Vec<Entry> = feed.entries().unwrap();
    ///     for feed_entry in feed_entries {
    ///         assert_eq!(id.to_owned(), feed_entry.id().into_string())
    ///     }
    /// }
    /// ```
    pub fn entries(self) -> Option<Vec<Entry>> {
        self.entries
    }
}
