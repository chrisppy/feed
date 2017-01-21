// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


use channels::{Category, CategoryBuilder, Channel, ChannelBuilder, Cloud, CloudBuilder, Enclosure,
               EnclosureBuilder, Image, ImageBuilder, Item, ItemBuilder, Guid, GuidBuilder, Source,
               SourceBuilder, TextInput, TextInputBuilder};
use rss;
use std::str::FromStr;
use utils::string_utils;


// Construct a new `Channel` and return it.
pub fn read(feed: &str) -> Channel {
    let rss_channel = rss::Channel::from_str(feed).unwrap();
    convert_channel(rss_channel)
}


// convert rss channel to feed channel
fn convert_channel(rss_channel: rss::Channel) -> Channel {
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
        .categories(convert_categories(rss_channel.categories))
        .generator(rss_channel.generator)
        .docs(rss_channel.docs)
        .cloud(convert_cloud(rss_channel.cloud))
        .ttl(string_utils::option_string_to_option_i64(rss_channel.ttl))
        .image(convert_image(rss_channel.image))
        .rating(None)
        .text_input(convert_text_input(rss_channel.text_input))
        .skip_hours(convert_skip_hours(rss_channel.skip_hours))
        .skip_days(convert_skip_days(rss_channel.skip_days))
        .items(convert_items(rss_channel.items))
        .finalize()
}


// Convert rss categories to feed categories
fn convert_categories(rss_cats: Vec<rss::Category>) -> Option<Vec<Category>> {
    if rss_cats.is_empty() {
        None
    } else {
        let mut cats: Vec<Category> = Vec::new();
        for rss_cat in rss_cats {
            let cat = CategoryBuilder::new()
                .name(rss_cat.name.as_str())
                .domain(rss_cat.domain)
                .finalize();
            cats.push(cat);
        }
        Some(cats)
    }
}


// Convert rss channel cloud to feed channel cloud
fn convert_cloud(rss_cloud_opt: Option<rss::Cloud>) -> Option<Cloud> {
    if rss_cloud_opt.is_none() {
        None
    } else {
        let rss_cloud = rss_cloud_opt.unwrap();
        let cloud = CloudBuilder::new()
            .domain(rss_cloud.domain.as_str())
            .port(string_utils::string_to_i64(rss_cloud.port.as_str()))
            .path(rss_cloud.path.as_str())
            .register_procedure(rss_cloud.register_procedure.as_str())
            .protocol(rss_cloud.protocol.as_str())
            .finalize();
        Some(cloud)
    }
}


// Convert rss channel image to feed channel image
fn convert_image(rss_image_opt: Option<rss::Image>) -> Option<Image> {
    if rss_image_opt.is_none() {
        None
    } else {
        let rss_image = rss_image_opt.unwrap();
        let image = ImageBuilder::new()
            .url(rss_image.url.as_str())
            .title(rss_image.title.as_str())
            .link(rss_image.link.as_str())
            .width(string_utils::option_string_to_option_i64(rss_image.width))
            .height(string_utils::option_string_to_option_i64(rss_image.height))
            .description(rss_image.description)
            .finalize();
        Some(image)
    }
}


// Convert rss channel text input to feed channel text input
fn convert_text_input(rss_text_input_opt: Option<rss::TextInput>) -> Option<TextInput> {
    if rss_text_input_opt.is_none() {
        None
    } else {
        let rss_text_input = rss_text_input_opt.unwrap();
        let text_input = TextInputBuilder::new()
            .title(rss_text_input.title.as_str())
            .description(rss_text_input.description.as_str())
            .name(rss_text_input.name.as_str())
            .link(rss_text_input.link.as_str())
            .finalize();
        Some(text_input)
    }
}


// Convert rss channel skip hours to feed channel skip hours
fn convert_skip_hours(rss_skip_hours: Vec<String>) -> Option<Vec<i64>> {
    if rss_skip_hours.is_empty() {
        None
    } else {
        let mut hours: Vec<i64> = Vec::new();
        for rss_hour in rss_skip_hours {
            let hour = string_utils::string_to_i64(rss_hour.as_str());
            hours.push(hour)
        }
        Some(hours)
    }
}


// Convert rss channel skip days to feed channel skip days
fn convert_skip_days(rss_skip_days: Vec<String>) -> Option<Vec<String>> {
    if rss_skip_days.is_empty() {
        None
    } else {
        Some(rss_skip_days)
    }
}


// Convert rss channel items to feed channel items
fn convert_items(rss_items: Vec<rss::Item>) -> Option<Vec<Item>> {
    if rss_items.is_empty() {
        None
    } else {
        let mut items: Vec<Item> = Vec::new();
        for rss_item in rss_items {
            let item = ItemBuilder::new()
                .title(rss_item.title)
                .link(rss_item.link)
                .description(rss_item.description)
                .author(rss_item.author)
                .categories(convert_categories(rss_item.categories))
                .comments(rss_item.comments)
                .enclosure(convert_enclosure(rss_item.enclosure))
                .guid(convert_guid(rss_item.guid))
                .pub_date(rss_item.pub_date)
                .source(convert_source(rss_item.source))
                .finalize();
            items.push(item);
        }
        Some(items)
    }
}


// Convert rss item enclosure to feed item enclosure
fn convert_enclosure(rss_enc_opt: Option<rss::Enclosure>) -> Option<Enclosure> {
    if rss_enc_opt.is_none() {
        None
    } else {
        let rss_enc = rss_enc_opt.unwrap();
        let enclosure = EnclosureBuilder::new()
            .url(rss_enc.url.as_str())
            .length(string_utils::string_to_i64(rss_enc.length.as_str()))
            .mime_type(rss_enc.mime_type.as_str())
            .finalize();
        Some(enclosure)
    }
}


// Convert rss item guid to feed item guid
fn convert_guid(rss_guid_opt: Option<rss::Guid>) -> Option<Guid> {
    if rss_guid_opt.is_none() {
        None
    } else {
        let rss_guid = rss_guid_opt.unwrap();
        let guid = GuidBuilder::new()
            .value(rss_guid.value.as_str())
            .permalink(Some(rss_guid.is_permalink))
            .finalize();
        Some(guid)
    }
}


// Convert rss item source to feed item source
fn convert_source(rss_src_opt: Option<rss::Source>) -> Option<Source> {
    if rss_src_opt.is_none() {
        None
    } else {
        let rss_src = rss_src_opt.unwrap();
        let src = SourceBuilder::new()
            .url(rss_src.url.as_str())
            .title(rss_src.title)
            .finalize();
        Some(src)
    }
}
