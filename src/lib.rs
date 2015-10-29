// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

extern crate rss;
extern crate url;
extern crate curl;

use curl::http;
use url::Url;

use std::str;

pub mod channel;
pub mod item;

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
