// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! All the structs for atom.
pub mod category;
pub mod category_builder;
pub mod entry;
pub mod entry_builder;
pub mod feed;
pub mod feed_builder;
pub mod generator;
pub mod generator_builder;
pub mod link;
pub mod link_builder;
pub mod person;
pub mod person_builder;
pub mod text;
pub mod text_builder;

use chrono::*;
use url::Url;

/// This `Feed` struct contains all the items that exist for the feed
/// field under RSS for Atom.
#[derive(Clone)]
pub struct AtomFeed {
    id: Url,
    title: Text,
    updated: DateTime<FixedOffset>,
    authors: Option<Vec<Person>>,
    links: Option<Vec<Link>>,
    categories: Option<Vec<AtomCategory>>,
    contributors: Option<Vec<Person>>,
    generator: Option<Generator>,
    icon: Option<Url>,
    logo: Option<Url>,
    rights: Option<Text>,
    subtitle: Option<Text>,
    entries: Option<Vec<Entry>>,
}


/// This `FeedBuilder` struct creates the `Feed`.
#[derive(Default)]
pub struct AtomFeedBuilder {
    id: String,
    title: Text,
    updated: String,
    authors: Option<Vec<Person>>,
    links: Option<Vec<Link>>,
    categories: Option<Vec<AtomCategory>>,
    contributors: Option<Vec<Person>>,
    generator: Option<Generator>,
    icon: Option<String>,
    logo: Option<String>,
    rights: Option<Text>,
    subtitle: Option<Text>,
    entries: Option<Vec<Entry>>,
}


/// This `Person` struct contains all the persons that exist for the feed
/// field under feed.
#[derive(Clone)]
pub struct Person {
    name: String,
    uri: Option<Url>,
    email: Option<String>,
}


/// This `PersonBuilder` struct creates the `Person`.
#[derive(Default)]
pub struct PersonBuilder {
    name: String,
    uri: Option<String>,
    email: Option<String>,
}


/// This `Link` struct contains all the links that exist for the feed
/// field under feed.
#[derive(Clone)]
pub struct Link {
    href: Url,
    rel: Option<String>,
    link_type: Option<String>,
    href_lang: Option<String>,
    title: Option<String>,
    length: Option<i64>,
}


/// This `LinkBuilder` struct creates the `Link`.
#[derive(Default)]
pub struct LinkBuilder {
    href: String,
    rel: Option<String>,
    link_type: Option<String>,
    href_lang: Option<String>,
    title: Option<String>,
    length: Option<i64>,
}


/// This `Category` struct contains all the categories that exist for the feed
/// field under feed.
#[derive(Clone)]
pub struct AtomCategory {
    term: String,
    scheme: Option<Url>,
    label: Option<String>,
}


/// This `CategoryBuilder` struct creates the `Category`.
#[derive(Default)]
pub struct AtomCategoryBuilder {
    term: String,
    scheme: Option<String>,
    label: Option<String>,
}


/// This `Generator` struct contains the generator that exist for the feed
/// field under feed.
#[derive(Clone)]
pub struct Generator {
    generator: String,
    uri: Option<Url>,
    version: Option<String>,
}


/// This `GeneratorBuilder` struct creates the `Generator`.
#[derive(Default)]
pub struct GeneratorBuilder {
    generator: String,
    uri: Option<String>,
    version: Option<String>,
}


/// This `Text` struct contains the text that exist for the feed
/// field under feed.
#[derive(Clone, Default)]
pub struct Text {
    text: String,
    text_type: String,
}


/// This `TextBuilder` struct creates the `Text`.
#[derive(Default)]
pub struct TextBuilder {
    text: String,
    text_type: Option<String>,
}


/// This `Entry` struct contains the entries that exist for the feed
/// field under feed.
#[derive(Clone)]
pub struct Entry {
    id: Url,
    title: Text,
    updated: DateTime<FixedOffset>,
    authors: Option<Vec<Person>>,
    content: Option<Text>,
    links: Option<Vec<Link>>,
    summary: Option<Text>,
    categories: Option<Vec<AtomCategory>>,
    contributors: Option<Vec<Person>>,
    published: Option<DateTime<FixedOffset>>,
    source: Option<String>,
    rights: Option<Text>,
}


/// This `EntryBuilder` struct creates the `Entry`.
#[derive(Default)]
pub struct EntryBuilder {
    id: String,
    title: Text,
    updated: String,
    authors: Option<Vec<Person>>,
    content: Option<Text>,
    links: Option<Vec<Link>>,
    summary: Option<Text>,
    categories: Option<Vec<AtomCategory>>,
    contributors: Option<Vec<Person>>,
    published: Option<String>,
    source: Option<String>,
    rights: Option<Text>,
}
