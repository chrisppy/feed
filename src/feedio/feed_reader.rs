// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! Implementation of `FeedReader`.

use atom::{AtomFeed, AtomFeedBuilder, AtomCategory, AtomCategoryBuilder, Entry, EntryBuilder,
           GeneratorBuilder, Link, LinkBuilder, Person, PersonBuilder, TextBuilder};
use feedio::FeedReader;
use quick_xml::{Event, XmlReader};
use util;

impl FeedReader {
    /// Construct a new `FeedReader` and return the `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::feedio::FeedReader;
    ///
    /// let feed_reader = FeedReader::new("String", false);
    /// ```
    pub fn new(feed: &str, channel_exists: bool) -> FeedReader {
        let feed_string = feed.to_owned();

        let mut element = "feed";
        let mut name = "";

        let mut authors: Vec<Person> = Vec::new();
        let mut contributors: Vec<Person> = Vec::new();
        let mut links: Vec<Link> = Vec::new();
        let mut categories: Vec<AtomCategory> = Vec::new();
        let mut entries: Vec<Entry> = Vec::new();
        let mut entry_authors: Vec<Person> = Vec::new();
        let mut entry_categories: Vec<AtomCategory> = Vec::new();
        let mut entry_links: Vec<Link> = Vec::new();
        let mut entry_contributors: Vec<Person> = Vec::new();


        let mut feed_builder = AtomFeedBuilder::new();
        let mut person_builder = PersonBuilder::new();
        let mut text_builder = TextBuilder::new();
        let mut entry_builder = EntryBuilder::new();
        let mut generator_builder = GeneratorBuilder::new();

        let reader = XmlReader::from(&*feed_string).trim_text(true);

        for r in reader {
            match r {
                Ok(Event::Start(ref e)) => {
                    if !channel_exists {
                        match e.name() {
                            b"entry" => {
                                element = "entry";
                                entry_builder = EntryBuilder::new();
                                entry_categories = Vec::new();
                                entry_authors = Vec::new();
                                entry_links = Vec::new();
                                entry_contributors = Vec::new();
                            }
                            b"id" => {
                                name = "id";
                            }
                            b"title" => {
                                name = "title";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"updated" => {
                                name = "updated";
                            }
                            b"author" => {
                                name = "author";
                                person_builder = PersonBuilder::new();
                            }
                            b"contributor" => {
                                name = "contributor";
                                person_builder = PersonBuilder::new();
                            }
                            b"name" => {
                                name = "name";
                            }
                            b"email" => {
                                name = "email";
                            }
                            b"uri" => {
                                name = "email";
                            }
                            b"icon" => {
                                name = "icon";
                            }
                            b"logo" => {
                                name = "logo";
                            }
                            b"rights" => {
                                name = "rights";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"subtitle" => {
                                name = "subtitle";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"content" => {
                                name = "content";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"summary" => {
                                name = "summary";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"link" => {
                                let href = util::attribute_to_string(e.attributes(), "href");
                                let link_type = util::attribute_to_string(e.attributes(), "type");
                                let hreflang = util::attribute_to_string(e.attributes(),
                                                                         "hreflang");
                                let rel = util::attribute_to_string(e.attributes(), "rel");
                                let title = util::attribute_to_string(e.attributes(), "title");
                                let length = util::attribute_to_i64(e.attributes(), "length");

                                let link = LinkBuilder::new()
                                    .href(href.expect("Link href cannot be None").as_str())
                                    .href_lang(hreflang)
                                    .rel(rel)
                                    .link_type(link_type)
                                    .title(title)
                                    .length(length)
                                    .finalize();

                                match element {
                                    "feed" => {
                                        links.push(link);
                                    }
                                    "entry" => {
                                        entry_links.push(link);
                                    }
                                    _ => (),
                                }
                            }
                            b"category" => {
                                let term = util::attribute_to_string(e.attributes(), "term");
                                let scheme = util::attribute_to_string(e.attributes(), "scheme");
                                let label = util::attribute_to_string(e.attributes(), "label");

                                let category = AtomCategoryBuilder::new()
                                    .term(term.expect("Category term cannot be None").as_str())
                                    .scheme(scheme)
                                    .label(label)
                                    .finalize();

                                match element {
                                    "feed" => {
                                        categories.push(category);
                                    }
                                    "entry" => {
                                        entry_categories.push(category);
                                    }
                                    _ => (),
                                }
                            }
                            b"generator" => {
                                name = "generator";

                                let uri = util::attribute_to_string(e.attributes(), "uri");
                                let version = util::attribute_to_string(e.attributes(), "version");

                                generator_builder.uri(uri).version(version);
                            }
                            b"published" => {
                                name = "published";
                            }
                            _ => (),
                        }
                    } else {
                        match e.name() {
                            b"atom:entry" => {
                                element = "entry";
                                entry_builder = EntryBuilder::new();
                                entry_categories = Vec::new();
                                entry_authors = Vec::new();
                                entry_links = Vec::new();
                                entry_contributors = Vec::new();
                            }
                            b"atom:id" => {
                                name = "id";
                            }
                            b"atom:title" => {
                                name = "title";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"atom:updated" => {
                                name = "updated";
                            }
                            b"atom:author" => {
                                name = "author";
                                person_builder = PersonBuilder::new();
                            }
                            b"atom:contributor" => {
                                name = "contributor";
                                person_builder = PersonBuilder::new();
                            }
                            b"atom:name" => {
                                name = "name";
                            }
                            b"atom:email" => {
                                name = "email";
                            }
                            b"atom:uri" => {
                                name = "email";
                            }
                            b"atom:icon" => {
                                name = "icon";
                            }
                            b"atom:logo" => {
                                name = "logo";
                            }
                            b"atom:rights" => {
                                name = "rights";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"atom:subtitle" => {
                                name = "subtitle";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"atom:content" => {
                                name = "content";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"atom:summary" => {
                                name = "summary";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"atom:link" => {
                                let href = util::attribute_to_string(e.attributes(), "href");
                                let link_type = util::attribute_to_string(e.attributes(), "type");
                                let hreflang = util::attribute_to_string(e.attributes(),
                                                                         "hreflang");
                                let rel = util::attribute_to_string(e.attributes(), "rel");
                                let title = util::attribute_to_string(e.attributes(), "title");
                                let length = util::attribute_to_i64(e.attributes(), "length");

                                let link = LinkBuilder::new()
                                    .href(href.expect("Link href cannot be None").as_str())
                                    .href_lang(hreflang)
                                    .rel(rel)
                                    .link_type(link_type)
                                    .title(title)
                                    .length(length)
                                    .finalize();

                                match element {
                                    "feed" => {
                                        links.push(link);
                                    }
                                    "entry" => {
                                        entry_links.push(link);
                                    }
                                    _ => (),
                                }
                            }
                            b"atom:category" => {
                                let term = util::attribute_to_string(e.attributes(), "term");
                                let scheme = util::attribute_to_string(e.attributes(), "scheme");
                                let label = util::attribute_to_string(e.attributes(), "label");

                                let category = AtomCategoryBuilder::new()
                                    .term(term.expect("Category term cannot be None").as_str())
                                    .scheme(scheme)
                                    .label(label)
                                    .finalize();

                                match element {
                                    "feed" => {
                                        categories.push(category);
                                    }
                                    "entry" => {
                                        entry_categories.push(category);
                                    }
                                    _ => (),
                                }
                            }
                            b"atom:generator" => {
                                name = "generator";

                                let uri = util::attribute_to_string(e.attributes(), "uri");
                                let version = util::attribute_to_string(e.attributes(), "version");

                                generator_builder.uri(uri).version(version);
                            }
                            b"atom:published" => {
                                name = "published";
                            }
                            b"atom10:entry" => {
                                element = "entry";
                                entry_builder = EntryBuilder::new();
                                entry_categories = Vec::new();
                                entry_authors = Vec::new();
                                entry_links = Vec::new();
                                entry_contributors = Vec::new();
                            }
                            b"atom10:id" => {
                                name = "id";
                            }
                            b"atom10:title" => {
                                name = "title";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"atom10:updated" => {
                                name = "updated";
                            }
                            b"atom10:author" => {
                                name = "author";
                                person_builder = PersonBuilder::new();
                            }
                            b"atom10:contributor" => {
                                name = "contributor";
                                person_builder = PersonBuilder::new();
                            }
                            b"atom10:name" => {
                                name = "name";
                            }
                            b"atom10:email" => {
                                name = "email";
                            }
                            b"atom10:uri" => {
                                name = "email";
                            }
                            b"atom10:icon" => {
                                name = "icon";
                            }
                            b"atom10:logo" => {
                                name = "logo";
                            }
                            b"atom10:rights" => {
                                name = "rights";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"atom10:subtitle" => {
                                name = "subtitle";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"atom10:content" => {
                                name = "content";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"atom10:summary" => {
                                name = "summary";

                                let text_type = util::attribute_to_string(e.attributes(), "type");

                                text_builder = TextBuilder::new();
                                text_builder.text_type(text_type);
                            }
                            b"atom10:link" => {
                                let href = util::attribute_to_string(e.attributes(), "href");
                                let link_type = util::attribute_to_string(e.attributes(), "type");
                                let hreflang = util::attribute_to_string(e.attributes(),
                                                                         "hreflang");
                                let rel = util::attribute_to_string(e.attributes(), "rel");
                                let title = util::attribute_to_string(e.attributes(), "title");
                                let length = util::attribute_to_i64(e.attributes(), "length");

                                let link = LinkBuilder::new()
                                    .href(href.expect("Link href cannot be None").as_str())
                                    .href_lang(hreflang)
                                    .rel(rel)
                                    .link_type(link_type)
                                    .title(title)
                                    .length(length)
                                    .finalize();

                                match element {
                                    "feed" => {
                                        links.push(link);
                                    }
                                    "entry" => {
                                        entry_links.push(link);
                                    }
                                    _ => (),
                                }
                            }
                            b"atom10:category" => {
                                let term = util::attribute_to_string(e.attributes(), "term");
                                let scheme = util::attribute_to_string(e.attributes(), "scheme");
                                let label = util::attribute_to_string(e.attributes(), "label");

                                let category = AtomCategoryBuilder::new()
                                    .term(term.expect("Category term cannot be None").as_str())
                                    .scheme(scheme)
                                    .label(label)
                                    .finalize();

                                match element {
                                    "feed" => {
                                        categories.push(category);
                                    }
                                    "entry" => {
                                        entry_categories.push(category);
                                    }
                                    _ => (),
                                }
                            }
                            b"atom10:generator" => {
                                name = "generator";

                                let uri = util::attribute_to_string(e.attributes(), "uri");
                                let version = util::attribute_to_string(e.attributes(), "version");

                                generator_builder.uri(uri).version(version);
                            }
                            b"atom10:published" => {
                                name = "published";
                            }
                            _ => (),
                        }
                    }
                }
                Ok(Event::Text(e)) => {
                    match name {
                        "id" => {
                            match element {
                                "feed" => {
                                    feed_builder.id(e.into_string().unwrap().as_str());
                                }
                                "entry" => {
                                    entry_builder.id(e.into_string().unwrap().as_str());
                                }
                                _ => (),
                            }
                        }
                        "title" => {
                            text_builder.text(e.into_string().unwrap().as_str());
                            match element {
                                "feed" => {
                                    feed_builder.title(text_builder.finalize());
                                }
                                "entry" => {
                                    entry_builder.title(text_builder.finalize());
                                }
                                _ => (),
                            }

                        }
                        "updated" => {
                            match element {
                                "feed" => {
                                    feed_builder.updated(e.into_string().unwrap().as_str());
                                }
                                "entry" => {
                                    entry_builder.updated(e.into_string().unwrap().as_str());
                                }
                                _ => (),
                            }
                        }
                        "name" => {
                            person_builder.name(e.into_string().unwrap().as_str());
                        }
                        "email" => {
                            person_builder.email(Some(e.into_string().unwrap()));
                        }
                        "uri" => {
                            person_builder.email(Some(e.into_string().unwrap()));
                        }
                        "generator" => {
                            generator_builder.generator(e.into_string().unwrap().as_str());
                        }
                        "icon" => {
                            feed_builder.icon(Some(e.into_string().unwrap()));
                        }
                        "logo" => {
                            feed_builder.logo(Some(e.into_string().unwrap()));
                        }
                        "rights" => {
                            text_builder.text(e.into_string().unwrap().as_str());
                            match element {
                                "feed" => {
                                    feed_builder.rights(Some(text_builder.finalize()));
                                }
                                "entry" => {
                                    entry_builder.rights(Some(text_builder.finalize()));
                                }
                                _ => (),
                            }

                        }
                        "subtitle" => {
                            text_builder.text(e.into_string().unwrap().as_str());
                            feed_builder.subtitle(Some(text_builder.finalize()));
                        }
                        "content" => {
                            text_builder.text(e.into_string().unwrap().as_str());
                            entry_builder.content(Some(text_builder.finalize()));
                        }
                        "summary" => {
                            text_builder.text(e.into_string().unwrap().as_str());
                            entry_builder.summary(Some(text_builder.finalize()));
                        }
                        "published" => {
                            entry_builder.published(Some(e.into_string().unwrap()));
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if !channel_exists {
                        match e.name() {
                            b"author" => {
                                match element {
                                    "feed" => {
                                        authors.push(person_builder.finalize());
                                    }
                                    "entry" => {
                                        entry_authors.push(person_builder.finalize());
                                    }
                                    _ => (),
                                }
                            }
                            b"contributor" => {
                                match element {
                                    "feed" => {
                                        contributors.push(person_builder.finalize());
                                    }
                                    "entry" => {
                                        entry_contributors.push(person_builder.finalize());
                                    }
                                    _ => (),
                                }
                            }
                            b"generator" => {
                                feed_builder.generator(Some(generator_builder.finalize()));
                            }
                            b"entry" => {
                                entry_builder.authors(Some(entry_authors.clone()));
                                entry_builder.categories(Some(entry_categories.clone()));
                                entry_builder.links(Some(entry_links.clone()));
                                entry_builder.contributors(Some(entry_contributors.clone()));
                                entries.push(entry_builder.finalize());
                            }
                            b"feed" => {
                                feed_builder.authors(Some(authors.clone()));
                                feed_builder.contributors(Some(contributors.clone()));
                                feed_builder.links(Some(links.clone()));
                                feed_builder.categories(Some(categories.clone()));
                                feed_builder.entries(Some(entries.clone()));
                            }
                            _ => (),
                        }
                    } else {
                        match e.name() {
                            b"atom:author" => {
                                match element {
                                    "feed" => {
                                        authors.push(person_builder.finalize());
                                    }
                                    "entry" => {
                                        entry_authors.push(person_builder.finalize());
                                    }
                                    _ => (),
                                }
                            }
                            b"atom:contributor" => {
                                match element {
                                    "feed" => {
                                        contributors.push(person_builder.finalize());
                                    }
                                    "entry" => {
                                        entry_contributors.push(person_builder.finalize());
                                    }
                                    _ => (),
                                }
                            }
                            b"atom:generator" => {
                                feed_builder.generator(Some(generator_builder.finalize()));
                            }
                            b"atom:entry" => {
                                entry_builder.authors(Some(entry_authors.clone()));
                                entry_builder.categories(Some(entry_categories.clone()));
                                entry_builder.links(Some(entry_links.clone()));
                                entry_builder.contributors(Some(entry_contributors.clone()));
                                entries.push(entry_builder.finalize());
                            }
                            b"atom:feed" => {
                                feed_builder.authors(Some(authors.clone()));
                                feed_builder.contributors(Some(contributors.clone()));
                                feed_builder.links(Some(links.clone()));
                                feed_builder.categories(Some(categories.clone()));
                                feed_builder.entries(Some(entries.clone()));
                            }
                            b"atom10:author" => {
                                match element {
                                    "feed" => {
                                        authors.push(person_builder.finalize());
                                    }
                                    "entry" => {
                                        entry_authors.push(person_builder.finalize());
                                    }
                                    _ => (),
                                }
                            }
                            b"atom10:contributor" => {
                                match element {
                                    "feed" => {
                                        contributors.push(person_builder.finalize());
                                    }
                                    "entry" => {
                                        entry_contributors.push(person_builder.finalize());
                                    }
                                    _ => (),
                                }
                            }
                            b"atom10:generator" => {
                                feed_builder.generator(Some(generator_builder.finalize()));
                            }
                            b"atom10:entry" => {
                                entry_builder.authors(Some(entry_authors.clone()));
                                entry_builder.categories(Some(entry_categories.clone()));
                                entry_builder.links(Some(entry_links.clone()));
                                entry_builder.contributors(Some(entry_contributors.clone()));
                                entries.push(entry_builder.finalize());
                            }
                            b"atom10:feed" => {
                                feed_builder.authors(Some(authors.clone()));
                                feed_builder.contributors(Some(contributors.clone()));
                                feed_builder.links(Some(links.clone()));
                                feed_builder.categories(Some(categories.clone()));
                                feed_builder.entries(Some(entries.clone()));
                            }
                            _ => (),
                        }
                    }
                }
                Err((e, pos)) => panic!("{:?} at position {}", e, pos),
                _ => (),
            }
        }

        FeedReader { feed: feed_builder.finalize() }
    }


    /// Get the `AtomFeed` after parsing.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::feedio::FeedReader;
    ///
    /// let feed_reader = FeedReader::new("String", false);
    /// let channel = feed_reader.feed();
    /// ```
    pub fn feed(self) -> AtomFeed {
        self.feed.clone()
    }
}
