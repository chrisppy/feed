// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

extern crate rss;

use rss::{Rss, Category};

/// Retrieve a String containing channel title
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_title = feedreader::channel::get_title(feed);
/// ```
pub fn get_title(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let title = rss_feed.title;
            return title;
        }
    }
}

/// Retrieve a String containing channel link
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_link = feedreader::channel::get_link(feed);
/// ```
pub fn get_link(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let link = rss_feed.link;
            return link;
        }
    }
}

/// Retrieve a String containing channel description
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_description = feedreader::channel::get_description(feed);
/// ```
pub fn get_description(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let description = rss_feed.description;
            return description;
        }
    }
}

/// Retrieve an Option<String> containing channel language
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_language = feedreader::channel::get_language(feed);
/// ```
pub fn get_language(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let language = rss_feed.language;
            return language;
        }
    }
}

/// Retrieve an Option<String> containing channel pub date
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_link = feedreader::channel::get_pub_date(feed);
/// ```
pub fn get_pub_date(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let pub_date = rss_feed.pub_date;
            return pub_date;
        }
    }
}

/// Retrieve an Option<String> containing channel copyright
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_copyright = feedreader::channel::get_copyright(feed);
/// ```
pub fn get_copyright(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let copyright = rss_feed.copyright;
            return copyright;
        }
    }
}

/// Retrieve an Option<String> containing channel managing editor
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_managing_editor = feedreader::channel::get_managing_editor(feed);
/// ```
pub fn get_managing_editor(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let managing_editor = rss_feed.managing_editor;
            return managing_editor;
        }
    }
}

/// Retrieve an Option<String> containing channel web master
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_web_master = feedreader::channel::get_web_master(feed);
/// ```
pub fn get_web_master(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let web_master = rss_feed.web_master;
            return web_master;
        }
    }
}

/// Retrieve an Option<String> containing channel last build date
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_last_build_date = feedreader::channel::get_last_build_date(feed);
/// ```
pub fn get_last_build_date(feed: String) -> Option<String> {
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
/// let channel_categories = feedreader::channel::get_categories(feed);
/// ```
pub fn get_categories(feed: String) -> Vec<Category> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let categories = rss_feed.categories;
            return categories;
        }
    }
}

/// Retrieve an Option<String> containing channel generator
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_generator = feedreader::channel::get_generator(feed);
/// ```
pub fn get_generator(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let generator = rss_feed.generator;
            return generator;
        }
    }
}

/// Retrieve an Option<String> containing channel docs
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_docs = feedreader::channel::get_docs(feed);
/// ```
pub fn get_docs(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let docs = rss_feed.docs;
            return docs;
        }
    }
}

/// Retrieve an Option<String> containing channel ttl
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_ttl = feedreader::channel::get_ttl(feed);
/// ```
pub fn get_ttl(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let ttl = rss_feed.ttl;
            return ttl;
        }
    }
}

/// Retrieve an Option<String> containing channel image
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_image = feedreader::channel::get_image(feed);
/// ```
pub fn get_image(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let image = rss_feed.image;
            return image;
        }
    }
}

/// Retrieve an Option<String> containing channel rating
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_ratingr = feedreader::channel::get_rating(feed);
/// ```
pub fn get_rating(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let rating = rss_feed.rating;
            return rating;
        }
    }
}

/// Retrieve an Option<String> containing channel skip hours
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_skip_hours = feedreader::channel::get_skip_hours(feed);
/// ```
pub fn get_skip_hours(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let skip_hours = rss_feed.skip_hours;
            return skip_hours;
        }
    }
}

/// Retrieve an Option<String> containing channel skip_days
///
/// # Example
///
/// ```
/// extern crate feedreader;
///
/// let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_skip_days = feedreader::channel::get_skip_days(feed);
/// ```
pub fn get_skip_days(feed: String) -> Option<String> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let skip_days = rss_feed.skip_hours;
            return skip_days;
        }
    }
}
