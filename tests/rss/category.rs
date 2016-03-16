// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use feed::rss::category::CategoryBuilder;

#[test]
fn category() {
    let category = "podcast";
    let category_obj = CategoryBuilder::new()
        .category(category)
        .finalize();
    assert_eq!(category.to_string(), category_obj.category());
}


#[test]
fn domain_some() {
    let domain_string = "http://jupiterbroadcasting.com".to_string();
    let category = CategoryBuilder::new()
        .domain(Some(domain_string.clone()))
        .finalize();
    let domain_option = category.domain();
    assert!(domain_option.is_some());
    let domain = domain_option.unwrap();
    assert_eq!(domain_string.clone(), domain);
}


#[test]
fn domain_none() {
    let category = CategoryBuilder::new()
        .domain(None)
        .finalize();
    let domain_option = category.domain();
    assert!(domain_option.is_none());
}
