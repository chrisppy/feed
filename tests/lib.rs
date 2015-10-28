// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
extern crate feedreader;

#[test]
fn get_channel_title() {
    let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert_eq!(feedreader::get_channel_title(feed), "The Linux Action Show! OGG".to_string());
}
