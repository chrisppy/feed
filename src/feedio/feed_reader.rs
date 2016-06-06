// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! Implementation of `FeedReader`.

use atom::{AtomFeed, AtomFeedBuilder};
use feedio::FeedReader;
use quick_xml::{Event, XmlReader};

impl FeedReader {
    /// Construct a new `FeedReader` and return the `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::feedio::FeedReader;
    ///
    /// let feed_reader = FeedReader::new("String");
    /// ```
    pub fn new(feed: &str, channel_exists: bool) -> FeedReader {
        let feed_string = feed.to_owned();

        let mut element = "channel";
        let mut name = "";

        let mut feed_builder = AtomFeedBuilder::new();

        if !channel_exists {
            let reader = XmlReader::from(&*feed_string).trim_text(true);
            for r in reader {
                match r {
                    Ok(Event::Start(ref e)) => {
                        match e.name() {
                            b"id" => {
                                name = "id";
                            }
                            b"title" => {
                                name = "title";
                            }
                            b"updated" => {
                                name = "updated";
                            }
                            _ => (),
                        }
                    }
                    Ok(Event::Text(e)) => {
                        match name {
                            "id" => {
                                feed_builder.id(e.into_string().unwrap().as_str());
                            }
                            "title" => {
                                feed_builder.title(e.into_string().unwrap().as_str());
                            }
                            "updated" => {
                                feed_builder.updated(e.into_string().unwrap().as_str());
                            }
                            _ => (),
                        }
                    }
                    Ok(Event::End(ref e)) => {
                        match e.name() {
                            _ => (),
                        }
                    }
                    Err((e, pos)) => panic!("{:?} at position {}", e, pos),
                    _ => (),
                }
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
