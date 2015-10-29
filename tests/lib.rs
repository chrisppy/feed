// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
extern crate feedreader;
extern crate rss;

use rss::Rss;

#[test]
fn get_channel_title() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_title(feed), "The Linux Action Show! OGG".to_string());
}

#[test]
fn get_channel_link() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_link(feed), "http://www.jupiterbroadcasting.com".to_string());
}

#[test]
fn get_channel_description() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_description(feed), "Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!".to_string());
}

#[test]
fn get_channel_generator() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_generator(feed).unwrap(), "Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34) http://reinventedsoftware.com/feeder/".to_string());
}

#[test]
fn get_channel_docs() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_docs(feed).unwrap(), "http://blogs.law.harvard.edu/tech/rss".to_string());
}

#[test]
fn get_channel_language() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_language(feed).unwrap(), "en".to_string());
}

#[test]
fn get_channel_pub_date() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_pub_date(feed).unwrap(), "Sun, 25 Oct 2015 08:30:31 -0700".to_string());
}

#[test]
fn get_channel_last_build_date() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_last_build_date(feed).unwrap(), "Sun, 25 Oct 2015 08:30:31 -0700".to_string());
}

#[test]
fn get_channel_image() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_image(feed).unwrap(), "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg".to_string());
}

#[test]
fn get_channel_copyright() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_copyright(feed).is_none(), true);
}

#[test]
fn get_channel_managing_editor() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_managing_editor(feed).is_none(), true);
}

#[test]
fn get_channel_web_master() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_web_master(feed).is_none(), true);
}

#[test]
fn get_channel_ttl() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_ttl(feed).is_none(), true);
}

#[test]
fn get_channel_rating() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_rating(feed).is_none(), true);
}

#[test]
fn get_channel_skip_hours() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_skip_hours(feed).is_none(), true);
}

#[test]
fn get_channel_skip_days() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_skip_days(feed).is_none(), true);
}

#[test]
fn get_item_title() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items;
            let size: usize = item.len() - 1;
            assert_eq!(feedreader::get_item_title(feed, size).unwrap(), "Linux Gaming for Everyone | LAS 311".to_string());
        }
    }
}

#[test]
fn get_item_link() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items;
            let size: usize = item.len() - 1;
            assert_eq!(feedreader::get_item_link(feed, size).unwrap(), "http://www.jupiterbroadcasting.com/56552/linux-gaming-for-everyone-las-311/".to_string());
        }
    }
}

#[test]
fn get_item_description() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items;
            let size: usize = item.len() - 1;
            assert_eq!(feedreader::get_item_description(feed, size).unwrap(), "<p>Chris and Matt showcase some fun games for casual or hardcore gamers. From space shooters to fun puzzles, if you\u{2019}ve been holding off on gaming for Linux we\u{2019}ll change your mind!</p>\n\n<p>Plus: Is Ubuntu for Android dead? Linus receives another award, the big update to Edward Snowden\u{2019}s favorite Linux distribution\u{2026}</p>\n\n<p>AND SO MUCH MORE!</p>\n\n<p>All this week on, the Linux Action Show!</p>".to_string());
        }
    }
}

#[test]
fn get_item_author() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items;
            let size: usize = item.len() - 1;
            assert_eq!(feedreader::get_item_author(feed, size).unwrap(), "chris@linuxactionshow.com".to_string());
        }
    }
}

#[test]
fn get_item_comments() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items;
            let size: usize = item.len() - 1;
            assert_eq!(feedreader::get_item_comments(feed, size).is_none(), true);
        }
    }
}

#[test]
fn get_item_pub_date() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items;
            let size: usize = item.len() - 1;
            assert_eq!(feedreader::get_item_pub_date(feed, size).unwrap(), "Mon, 05 May 2014 01:05:30 -0700".to_string());
        }
    }
}

#[test]
fn get_channel_categories() {
    assert!(false);
}

#[test]
fn get_item_categories() {
    assert!(false);
}

#[test]
fn new() {
    assert!(false);
}
