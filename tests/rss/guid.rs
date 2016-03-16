// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use feed::rss::guid::GuidBuilder;

#[test]
fn is_permalink_none() {
    let guid = GuidBuilder::new()
        .is_permalink(None)
        .finalize();
    assert!(guid.is_permalink());
}


#[test]
fn is_permalink_true() {
    let is_permalink = true;
    let guid = GuidBuilder::new()
        .is_permalink(Some(is_permalink))
        .finalize();
    assert_eq!(is_permalink, guid.is_permalink());
}


#[test]
fn is_permalink_false() {
    let is_permalink = false;
    let guid = GuidBuilder::new()
        .is_permalink(Some(is_permalink))
        .finalize();
    assert_eq!(is_permalink, guid.is_permalink());
}


#[test]
fn guid() {
    let guid = "9DE46946-2F90-4D5D-9047-7E9165C16E7C";
    let guid_obj = GuidBuilder::new()
        .guid(guid)
        .finalize();
    assert_eq!(guid.to_string(), guid_obj.guid());
}
