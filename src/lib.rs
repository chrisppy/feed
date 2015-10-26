// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

extern crate syndication;
extern crate url;
extern crate curl;

use curl::http;
use url::{Url, ParseError};
use syndication::Feed;

mod rss;

pub fn new(url &str) -> rss::Channel_Feed {
    let feed = get_feed(&feed_url);
    Channel_Feed::new(url, feed)

}

fn get_feed(url: &str) -> &'static str {
    let feed_url = Url::parse(url).unwrap();

    let resp = http::handle().get(&feed_url).exec().unwrap();
    println!("code={}; headers={:?}; body={:?}", resp.get_code(), resp.get_headers(), resp.get_body());

    resp.get_body()
}
