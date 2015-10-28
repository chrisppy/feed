// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

extern crate rss;
extern crate url;
extern crate curl;

use curl::http;
use url::Url;
use rss::{Rss, Category};

use std::str;

/// Retrieve a String containing the rss feed from an URL
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// ```
pub fn new(url: &str) -> String {
    let feed_url = Url::parse(url).unwrap();

    let resp = http::handle().get(&feed_url).exec().unwrap();
    let body = resp.get_body();
    let feed = str::from_utf8(body).unwrap().to_string();
    feed
}

/// Retrieve a String containing channel title
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_title = feedreader::get_channel_title(feed);
/// ```
pub fn get_channel_title(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let title = rss_feed.title;
            return title;
        }
    }
}

/// Retrieve a String containing channel language
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_language = feedreader::get_channel_language(feed);
/// ```
pub fn get_channel_language(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let language = rss_feed.language;
            return language;
        }
    }
}

/// Retrieve a String containing channel copyright
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_copyright = feedreader::get_channel_copyright(feed);
/// ```
pub fn get_channel_copyright(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let copyright = rss_feed.copyright;
            return copyright;
        }
    }
}

/// Retrieve a String containing channel managing editor
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_managing_editor = feedreader::get_channel_managing_editor(feed);
/// ```
pub fn get_channel_managing_editor(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let managing_editor = rss_feed.managing_editor;
            return managing_editor;
        }
    }
}

/// Retrieve a String containing channel web master
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_web_master = feedreader::get_channel_web_master(feed);
/// ```
pub fn get_channel_web_master(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let web_master = rss_feed.web_master;
            return web_master;
        }
    }
}

/// Retrieve a String containing channel last build date
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_last_build_date = feedreader::get_channel_last_build_date(feed);
/// ```
pub fn get_channel_last_build_date(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let last_build_date = rss_feed.last_build_date;
            return last_build_date;
        }
    }
}

/// Retrieve a Vec<Category> containing channel Rss Categories
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_categories = feedreader::get_channel_categories(feed);
/// ```
pub fn get_channel_categories(feed: String) -> Vec<Category> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let categories = rss_feed.categories;
            return categories;
        }
    }
}

/// Retrieve a String containing channel generator
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_generator = feedreader::get_channel_generator(feed);
/// ```
pub fn get_channel_generator(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let generator = rss_feed.generator;
            return generator;
        }
    }
}

/// Retrieve a String containing channel docs
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_docs = feedreader::get_channel_docs(feed);
/// ```
pub fn get_channel_docs(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let docs = rss_feed.docs;
            return docs;
        }
    }
}

/// Retrieve a String containing channel ttl
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_ttl = feedreader::get_channel_ttl(feed);
/// ```
pub fn get_channel_ttl(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let ttl = rss_feed.ttl;
            return ttl;
        }
    }
}

/// Retrieve a String containing channel image
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_image = feedreader::get_channel_image(feed);
/// ```
pub fn get_channel_image(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let image = rss_feed.image;
            return image;
        }
    }
}

/// Retrieve a String containing channel rating
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_ratingr = feedreader::get_channel_rating(feed);
/// ```
pub fn get_channel_rating(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let rating = rss_feed.rating;
            return rating;
        }
    }
}

/// Retrieve a String containing channel skip hours
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_skip_hours = feedreader::get_channel_skip_hours(feed);
/// ```
pub fn get_channel_skip_hours(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let skip_hours = rss_feed.skip_hours;
            return skip_hours;
        }
    }
}

/// Retrieve a String containing channel skip_days
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_skip_days = feedreader::get_channel_skip_days(feed);
/// ```
pub fn get_channel_skip_days(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let skip_days = rss_feed.skip_hours;
            return skip_days;
        }
    }
}

/// Retrieve a String containing item title
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_title = feedreader::get_item_title(feed, item_element);
/// ```
pub fn get_item_title(feed: String, element: usize) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let title = item.title;
            return title;
        }
    }
}

/// Retrieve a String containing item link
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_link = feedreader::get_item_link(feed, item_element);
/// ```
pub fn get_item_link(feed: String, element: usize) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let link = item.link;
            return link;
        }
    }
}

/// Retrieve a String containing item description
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_description = feedreader::get_item_description(feed, item_element);
/// ```
pub fn get_item_description(feed: String, element: usize) -> Option<String>
{
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let description = item.description;
            return description;
        }
    }
}

/// Retrieve a String containing item author
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_author = feedreader::get_item_title(feed, item_element);
/// ```
pub fn get_item_author(feed: String, element: usize) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let author = item.author;
            return author;
        }
    }
}

/// Retrieve a Vec<Category> containing item Rss Categories
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_categories = feedreader::get_item_categories(feed, item_element);
/// ```
pub fn get_item_categories(feed: String, element: usize) -> Vec<Category> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let categories = item.categories;
            return categories;
        }
    }
}

/// Retrieve a String containing item comments
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_comments = feedreader::get_item_comments(feed, item_element);
/// ```
pub fn get_item_comments(feed: String, element: usize) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let comments = item.comments;
            return comments;
        }
    }
}

/// Retrieve a String containing item pub date
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_pub_date = feedreader::get_item_pub_date(feed, item_element);
/// ```
pub fn get_item_pub_date(feed: String, element: usize) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let pub_date = item.pub_date;
            return pub_date;
        }
    }
}
