// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

extern crate rss;

use rss::{Rss, Category};

/// Retrieve an Option<String> containing item title
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// use feedreader::item;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_title = feedreader::item::get_title(feed, item_element);
/// ```
pub fn get_title(feed: String, element: usize) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let title = item.title;
            return title;
        }
    }
}

/// Retrieve an Option<String> containing item link
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// use feedreader::item;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_link = feedreader::item::get_link(feed, item_element);
/// ```
pub fn get_link(feed: String, element: usize) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let link = item.link;
            return link;
        }
    }
}

/// Retrieve an Option<String> containing item description
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// use feedreader::item;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_description = feedreader::item::get_description(feed, item_element);
/// ```
pub fn get_description(feed: String, element: usize) -> Option<String>
{
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let description = item.description;
            return description;
        }
    }
}

/// Retrieve an Option<String> containing item author
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// use feedreader::item;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_author = feedreader::item::get_title(feed, item_element);
/// ```
pub fn get_author(feed: String, element: usize) -> Option<String> {
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
/// use feedreader::item;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_categories = feedreader::item::get_categories(feed, item_element);
/// ```
pub fn get_categories(feed: String, element: usize) -> Vec<Category> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let categories = item.categories;
            return categories;
        }
    }
}

/// Retrieve an Option<String> containing item comments
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// use feedreader::item;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_comments = feedreader::item::get_comments(feed, item_element);
/// ```
pub fn get_comments(feed: String, element: usize) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let comments = item.comments;
            return comments;
        }
    }
}

/// Retrieve an Option<String> containing item pub date
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// use feedreader::item;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_pub_date = feedreader::item::get_pub_date(feed, item_element);
/// ```
pub fn get_pub_date(feed: String, element: usize) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let pub_date = item.pub_date;
            return pub_date;
        }
    }
}
