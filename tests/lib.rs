// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

extern crate feed-reader;

use feed-reader::Channel_Feed;

#[test]
fn test_channel_title() {
    let channel = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
    assert!(channel.get_title(), "The Linux Action Show! OGG".to_string());
}
