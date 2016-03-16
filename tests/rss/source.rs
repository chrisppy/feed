// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use feed::rss::source::SourceBuilder;

#[test]
fn url() {
    let url = "http://www.tomalak.org/links2.xml";
    let source = SourceBuilder::new()
        .url(url)
        .finalize();
    assert_eq!(url.to_string(), source.url());
}


#[test]
fn source() {
    let source = "Tomalak's Realm";
    let source_obj = SourceBuilder::new()
        .source(source)
        .finalize();
    assert_eq!(source.to_string(), source_obj.source());
}
