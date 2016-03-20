// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
mod rss;

extern crate feed;
extern crate url;

use feed::FeedBuilder;
use url::Url;


#[test]
fn from_url() {
    let url = match Url::parse("http://feeds2.feedburner.com/TheLinuxActionShowOGG.xml") {
        Ok(result) => result,
        Err(err)   => panic!("Url parse Error: {}", err),
    };
    let feed = FeedBuilder::new().from_url(url).finalize();
    let channel = feed.channel();
    assert_eq!("The Linux Action Show! OGG".to_string(), channel.title());
    assert_eq!("http://www.jupiterbroadcasting.com".to_string(), channel.link());
    assert_eq!("Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!".to_string(), channel.description());
    assert_eq!(Some("Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34) http://reinventedsoftware.com/feeder/".to_string()), channel.generator());
    assert_eq!(Some("http://blogs.law.harvard.edu/tech/rss".to_string()), channel.docs());
    assert_eq!(Some("en".to_string()), channel.language());
    assert!(channel.copyright().is_none());
    assert!(channel.managing_editor().is_none());
    assert!(channel.web_master().is_none());
    assert_eq!("Sun, 13 Mar 2016 20:02:02 -0700".to_string(), channel.pub_date().unwrap().to_rfc2822());
    assert_eq!("Sun, 13 Mar 2016 20:02:02 -0700".to_string(), channel.last_build_date().unwrap().to_rfc2822());
}
