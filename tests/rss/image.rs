// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use feed::rss::image::ImageBuilder;

#[test]
fn url() {
    let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    let image = ImageBuilder::new()
        .url(url)
        .finalize();
    assert_eq!(url.to_string(), image.url());
}


#[test]
fn title() {
    let title = "LAS 300 Logo";
    let image = ImageBuilder::new()
        .title(title)
        .finalize();
    assert_eq!(title.to_string(), image.title());
}


#[test]
fn link() {
    let link = "http://www.jupiterbroadcasting.com";
    let image = ImageBuilder::new()
        .link(link)
        .finalize();
    assert_eq!(link.to_string(), image.link());
}


#[test]
fn width_none() {
    let default: i64 = 88;
    let image = ImageBuilder::new()
        .width(None)
        .finalize();
    assert_eq!(default, image.width());
}


#[test]
fn width_some() {
    let width: i64 = 60;
    let image = ImageBuilder::new()
        .width(Some(width))
        .finalize();
    assert_eq!(width, image.width());
}


#[test]
fn width_max() {
    let width: i64 = 777;
    let max: i64 = 144;
    let image = ImageBuilder::new()
        .width(Some(width))
        .finalize();
    assert_eq!(max, image.width());
}


#[test]
fn height_none() {
    let default: i64 = 31;
    let image = ImageBuilder::new()
        .height(None)
        .finalize();
    assert_eq!(default, image.height());
}


#[test]
fn height_some() {
    let height: i64 = 60;
    let image = ImageBuilder::new()
        .height(Some(height))
        .finalize();
    assert_eq!(height, image.height());
}


#[test]
fn height_max() {
    let height: i64 = 777;
    let max: i64 = 400;
    let image = ImageBuilder::new()
        .height(Some(height))
        .finalize();
    assert_eq!(max, image.height());
}


#[test]
fn description_none() {
    let image = ImageBuilder::new()
        .description(None)
        .finalize();
    assert!(image.description().is_none());
}


#[test]
fn description_some() {
    let description_string = "This is a test".to_string();
    let image = ImageBuilder::new()
        .description(Some(description_string.clone()))
        .finalize();
    let description_option = image.description();
    assert!(description_option.is_some());
    let description = description_option.unwrap();
    assert_eq!(description.clone(), description);
}
