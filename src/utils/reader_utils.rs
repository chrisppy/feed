// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


use channels::{Channel, ChannelBuilder};
use rss;
use std::str::FromStr;
use utils::string_utils;

// Construct a new `Channel` and return it.
pub fn read(feed: &str) -> Channel {

    let rss_channel = rss::Channel::from_str(feed).unwrap();

    ChannelBuilder::new()
        .title(rss_channel.title.as_str())
        .link(rss_channel.link.as_str())
        .description(rss_channel.description.as_str())
        .language(rss_channel.language)
        .copyright(rss_channel.copyright)
        .managing_editor(rss_channel.managing_editor)
        .web_master(rss_channel.webmaster)
        .pub_date(rss_channel.pub_date)
        .last_build_date(rss_channel.last_build_date)
        .categories(None)
        .generator(rss_channel.generator)
        .docs(rss_channel.docs)
        .cloud(None)
        .ttl(string_utils::option_string_to_option_i64(rss_channel.ttl))
        .image(None)
        .rating(None)
        .text_input(None)
        .skip_hours(None)
        .skip_days(None)
        .items(None)
        .finalize()
}
