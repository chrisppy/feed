// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for entry by using the methods under `EntryBuilder`.

use atom::{AtomCategory, Entry, EntryBuilder, Link, Person, Text};
use chrono::*;
use errors;
use FeedBuilder;
use url::Url;

impl EntryBuilder {
    /// Construct a new `EntryBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::EntryBuilder;
    ///
    /// let entry_builder = EntryBuilder::new();
    /// ```
    pub fn new() -> EntryBuilder {
        EntryBuilder::default()
    }


    /// Set the id that exists under `Entry`.
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
    ///     entry_builder.id("http://newrustacean.com/show_notes/e014/");
    /// }
    /// ```
    pub fn id(&mut self, id: &str) -> &mut EntryBuilder {
        self.id = id.to_owned();
        self
    }


    /// Set the title that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::EntryBuilder;
    ///
    /// fn main() {
    ///     let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    ///     let mut entry_builder = EntryBuilder::new();
    ///     entry_builder.title(text);
    /// }
    /// ```
    pub fn title(&mut self, title: Text) -> &mut EntryBuilder {
        self.title = title;
        self
    }


    /// Set the updated that exists under `Entry`.
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
    ///     entry_builder.updated("2014-11-28T12:00:09+00:00");
    /// }
    /// ```
    pub fn updated(&mut self, updated: &str) -> &mut EntryBuilder {
        self.updated = updated.to_owned();
        self
    }


    /// Set the authors that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, PersonBuilder};
    ///
    /// fn main() {
    ///     let author = PersonBuilder::new().name("Chris Krycho").finalize();
    ///     let mut entry_builder = EntryBuilder::new();
    ///     entry_builder.authors(Some(vec![author]));
    /// }
    /// ```
    pub fn authors(&mut self, authors: Option<Vec<Person>>) -> &mut EntryBuilder {
        self.authors = authors;
        self
    }


    /// Set the content that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let content = TextBuilder::new().text("This is an example!").finalize();
    ///     let mut entry_builder = EntryBuilder::new();
    ///     entry_builder.content(Some(content));
    /// }
    /// ```
    pub fn content(&mut self, content: Option<Text>) -> &mut EntryBuilder {
        self.content = content;
        self
    }


    /// Set the links that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, LinkBuilder};
    ///
    /// fn main() {
    ///     let link = LinkBuilder::new().href("http://www.example.com").finalize();
    ///     let mut entry_builder = EntryBuilder::new();
    ///     entry_builder.links(Some(vec![link]));
    /// }
    /// ```
    pub fn links(&mut self, links: Option<Vec<Link>>) -> &mut EntryBuilder {
        self.links = links;
        self
    }


    /// Set the summary that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let summary = TextBuilder::new().text("This is an example!").finalize();
    ///     let mut entry_builder = EntryBuilder::new();
    ///     entry_builder.summary(Some(summary));
    /// }
    /// ```
    pub fn summary(&mut self, summary: Option<Text>) -> &mut EntryBuilder {
        self.summary = summary;
        self
    }


    /// Set the categories that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, AtomCategoryBuilder};
    ///
    /// fn main() {
    ///     let category = AtomCategoryBuilder::new().term("Tech").finalize();
    ///     let mut entry_builder = EntryBuilder::new();
    ///     entry_builder.categories(Some(vec![category]));
    /// }
    /// ```
    pub fn categories(&mut self, categories: Option<Vec<AtomCategory>>) -> &mut EntryBuilder {
        self.categories = categories;
        self
    }


    /// Set the contributors that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, PersonBuilder};
    ///
    /// fn main() {
    ///     let contributor = PersonBuilder::new().name("Chris Krycho").finalize();
    ///     let mut entry_builder = EntryBuilder::new();
    ///     entry_builder.contributors(Some(vec![contributor]));
    /// }
    /// ```
    pub fn contributors(&mut self, contributors: Option<Vec<Person>>) -> &mut EntryBuilder {
        self.contributors = contributors;
        self
    }


    /// Set the published that exists under `Entry`.
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
    ///     entry_builder.published(Some("2014-11-28T12:00:09+00:00".to_owned()));
    /// }
    /// ```
    pub fn published(&mut self, published: Option<String>) -> &mut EntryBuilder {
        self.published = published;
        self
    }


    /// Set the source that exists under `Entry`.
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
    pub fn source(&mut self, source: Option<String>) -> &mut EntryBuilder {
        self.source = source;
        self
    }


    /// Set the rights that exists under `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{EntryBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let rights = TextBuilder::new().text("© 2005 John Doe").finalize();
    ///     let mut entry_builder = EntryBuilder::new();
    ///     entry_builder.rights(Some(rights));
    /// }
    /// ```
    pub fn rights(&mut self, rights: Option<Text>) -> &mut EntryBuilder {
        self.rights = rights;
        self
    }


    /// Construct the `Entry` from the `EntryBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::EntryBuilder;
    ///
    /// fn main() {
    ///     let entry = EntryBuilder::new()
    ///     .id("http://newrustacean.com/show_notes/e014/")
    ///     .title("e014: Stringing things along")
    ///     .updated("2014-11-28T12:00:09+00:00")
    ///     .finalize();
    /// }
    /// ```
    pub fn finalize(&self) -> Entry {
        let mut published_option: Option<DateTime<FixedOffset>> = None;
        if self.published.is_some() {
            let date = DateTime::parse_from_rfc3339(self.published.clone().unwrap().as_str())
                .expect(errors::date_parse_error(self.published
                        .clone()
                        .unwrap()
                        .as_str())
                    .as_str());
            published_option = Some(date);
        }

        let mut source_option: Option<String> = None;
        if self.source.is_some() {
            let feed = FeedBuilder::new()
                .read_from_url(self.source.clone().unwrap().as_str())
                .finalize();
            source_option = Some(String::from_utf8(feed.xml()).unwrap());
        }

        Entry {
            id: Url::parse(self.id.clone().as_str())
                .expect(errors::url_parse_error(self.id.clone().as_str()).as_str()),
            title: self.title.clone(),
            updated: DateTime::parse_from_rfc3339(self.updated.clone().as_str())
                .expect(errors::date_parse_error(self.updated.clone().as_str()).as_str()),
            authors: self.authors.clone(),
            content: self.content.clone(),
            links: self.links.clone(),
            summary: self.summary.clone(),
            categories: self.categories.clone(),
            contributors: self.contributors.clone(),
            published: published_option,
            source: source_option,
            rights: self.rights.clone(),
        }
    }
}
