// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

extern crate syndication;
extern crate url;
extern crate curl;

use curl::http;
use url::{Url, ParseError};
use syndication::Feed;

pub struct Channel_Feed {
    url: String,
    feed: String,
    title: String,
    items: Vec<Item>,
    language: Option<String>,
    copyright: Option<String>,
    managing_editor: Option<String>,
    web_master: Option<String>,
    pub_date: Option<String>,
    last_build_date: Option<String>,
    categories: Vec<Category>,
    generator: Option<String>,
    docs: Option<String>,
    ttl: Option<String>,
    image: Option<String>,
    rating: Option<String>,
    text_input: Option<TextInput>,
    skip_hours: Option<String>,
    skip_days: Option<String>,
}

pub impl Channel_Feed {
    fn new(url: &str, feed: &str) -> Channel_Feed {

        match feed.parse::<Feed>().unwrap() {
            Feed::RSS(feed) => {
                Channel_Feed {
                    url: url.to_string(),
                    feed: resp.get_body(),
                    title: feed.title,
                    items: items[],
                    language: feed.language,
                    copyright: feed.copyright,
                    managing_editor: feed.managing_editor,
                    web_master: feed.web_master,
                    last_build_date: feed.last_build_date,
                    categories: feed.categories[],
                    generator: feed.generator,
                    docs: feed.language,
                    ttl: feed.ttl,
                    image: feed.image,
                    rating: feed.rating,
                    text_input: feed.text_input,
                    skip_hours: feed.skip_hours,
                    skip_days: feed.skip_days,
                };
                println!("RSS title: {:?}", self.title);
            },
            _ => {}
        };
    }

    fn get_url(&self) -> String {
        self.url
    }

    fn get_feed(&self) -> String {
        self.feed
    }

    fn get_title(&self) -> String {
        self.title
    }

    fn get_item_title(&self, element: i32) -> Option<String> {
        self.items[element].title
    }

    fn get_item_link(&self, element: i32) -> Option<String> {
        self.items[element].link
    }

    fn get_item_description(&self, element: i32) -> Option<String> {
        self.items[element].description
    }

    fn get_item_author(&self, element: i32) -> Option<String> {
        self.items[element].author
    }

    fn get_item_categories(&self, element: i32) -> Vec<Category> {
        self.items[element].categories[]
    }

    fn get_item_comments(&self, element: i32) -> Option<String> {
        self.items[element].comments
    }

    fn get_item_pub_date(&self, element: i32) -> Option<String> {
        self.items[element].pub_date
    }

    fn get_language(&self) -> Option<String> {
        self.language
    }

    fn get_copyright(&self) -> Option<String> {
        self.copyright
    }

    fn get_managing_editor(&self) -> Option<String> {
        self.managing_editor
    }

    fn get_web_master(&self) -> Option<String> {
        self.web_master
    }

    fn get_last_build_date(&self) -> Option<String> {
        self.last_build_date
    }

    fn get_categories(&self) -> Vec<Category> {
        self.categories[]
    }

    fn get_generator(&self) -> Option<String> {
        self.generator
    }

    fn get_docs(&self) -> Option<String> {
        self.docs
    }

    fn get_ttl(&self) -> Option<String> {
        self.ttl
    }

    fn get_image(&self) -> Option<String> {
        self.image
    }

    fn get_rating(&self) -> Option<String> {
        self.rating
    }

    fn get_text_input(&self) -> Option<TextInput> {
        self.text_input
    }

    fn get_skip_hours(&self) -> Option<String> {
        self.skip_hours
    }

    fn get_skip_days(&self) -> Option<String> {
        self.skip_days
    }
}
