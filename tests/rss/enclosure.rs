// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use feed::rss::enclosure::EnclosureBuilder;

#[test]
fn url() {
    let url = "http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    let enclosure = EnclosureBuilder::new()
        .url(url)
        .finalize();
    assert_eq!(url.to_string(), enclosure.url())
}


#[test]
fn length() {
    let length: i64 = 70772893;
    let enclosure = EnclosureBuilder::new()
        .length(length)
        .finalize();
    assert_eq!(length, enclosure.length())
}


#[test]
fn enclosure_type() {
    let enclosure_type = "audio/ogg";
    let enclosure = EnclosureBuilder::new()
        .enclosure_type(enclosure_type)
        .finalize();
    assert_eq!(enclosure_type.to_string(), enclosure.enclosure_type())
}
