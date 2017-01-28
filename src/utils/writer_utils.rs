// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! writer utilities.



use channels::{Category, Channel, Cloud, Enclosure, Guid, Image, Item, Source, TextInput};
use enums::Day;
use rss;
use utils::string_utils;


/// Construct xml from a `Channel`.
pub fn write(channel: Channel) -> Vec<u8>
{
    let rss_channel = convert_channel(channel);
    let s = rss_channel.to_string();
    s.into_bytes()
}


// convert rss channel from feed channel
fn convert_channel(channel: Channel) -> rss::Channel
{
    rss::Channel {
        title: channel.title(),
        link: channel.link().into_string(),
        description: channel.description(),
        language: channel.language(),
        copyright: channel.copyright(),
        managing_editor: channel.managing_editor(),
        webmaster: channel.web_master(),
        pub_date: string_utils::option_date_to_option_string(channel.pub_date()),
        last_build_date: string_utils::option_date_to_option_string(channel.last_build_date()),
        categories: convert_categories(channel.categories()),
        generator: channel.generator(),
        docs: string_utils::option_url_to_option_string(channel.docs()),
        cloud: convert_cloud(channel.cloud()),
        ttl: string_utils::option_i64_to_option_string(channel.ttl()),
        image: convert_image(channel.image()),
        // rating: channel.rating(),
        text_input: convert_text_input(channel.text_input()),
        skip_hours: convert_skip_hours(channel.skip_hours()),
        skip_days: convert_skip_days(channel.skip_days()),
        items: convert_items(channel.items()),
        itunes_ext: None,
        dublin_core_ext: None,
        ..Default::default()
    }
}


// Convert rss categories from feed categories
fn convert_categories(cats_opt: Option<Vec<Category>>) -> Vec<rss::Category>
{
    let mut rss_cats = Vec::new();
    if cats_opt.is_some()
    {
        let cats = cats_opt.unwrap();
        for cat in cats
        {
            let domain = if cat.domain().is_none()
            {
                None
            }
            else
            {
                let url = cat.domain().unwrap().into_string();
                Some(url)
            };
            let rss_cat = rss::Category {
                name: cat.name(),
                domain: domain,
            };
            rss_cats.push(rss_cat);
        }
    }
    rss_cats
}


// Convert rss channel cloud from feed channel cloud
fn convert_cloud(cloud_opt: Option<Cloud>) -> Option<rss::Cloud>
{
    if cloud_opt.is_none()
    {
        None
    }
    else
    {
        let cloud = cloud_opt.unwrap();
        let rss_cloud = rss::Cloud {
            domain: cloud.domain().into_string(),
            port: cloud.port().to_string(),
            path: cloud.path(),
            register_procedure: cloud.register_procedure(),
            protocol: cloud.protocol().into_string(),
        };
        Some(rss_cloud)
    }
}


// Convert rss channel image from feed channel image
fn convert_image(image_opt: Option<Image>) -> Option<rss::Image>
{
    if image_opt.is_none()
    {
        None
    }
    else
    {
        let image = image_opt.unwrap();
        let rss_image = rss::Image {
            url: image.url().into_string(),
            title: image.title(),
            link: image.link().into_string(),
            width: string_utils::i64_to_option_string(image.width()),
            height: string_utils::i64_to_option_string(image.height()),
            description: image.description(),
        };
        Some(rss_image)
    }
}


// Convert rss channel text input from feed channel text input
fn convert_text_input(text_input_opt: Option<TextInput>) -> Option<rss::TextInput>
{
    if text_input_opt.is_none()
    {
        None
    }
    else
    {
        let text_input = text_input_opt.unwrap();
        let rss_text_input = rss::TextInput {
            title: text_input.title(),
            description: text_input.description(),
            name: text_input.name(),
            link: text_input.link().into_string(),
        };
        Some(rss_text_input)
    }
}


// Convert rss channel skip hours from feed channel skip hours
fn convert_skip_hours(skip_hours: Option<Vec<i64>>) -> Vec<String>
{
    let mut rss_skip_hours = Vec::new();
    if skip_hours.is_some()
    {
        let hours = skip_hours.unwrap();
        for hour in hours
        {
            rss_skip_hours.push(hour.to_string());
        }
    }
    rss_skip_hours
}



// Convert rss channel skip days from feed channel skip days
fn convert_skip_days(skip_days: Option<Vec<Day>>) -> Vec<String>
{
    let mut rss_skip_days = Vec::new();
    if skip_days.is_some()
    {
        let days = skip_days.unwrap();
        for day in days
        {
            rss_skip_days.push(day.into_string());
        }
    }
    rss_skip_days
}


// Convert rss channel items from feed channel items
fn convert_items(items_opt: Option<Vec<Item>>) -> Vec<rss::Item>
{
    let mut rss_items = Vec::new();
    if items_opt.is_some()
    {
        let items = items_opt.unwrap();
        for item in items
        {
            let rss_item = rss::Item {
                title: item.title(),
                link: string_utils::option_url_to_option_string(item.link()),
                description: item.description(),
                author: item.author(),
                categories: convert_categories(item.categories()),
                comments: string_utils::option_url_to_option_string(item.comments()),
                enclosure: convert_enclosure(item.enclosure()),
                guid: convert_guid(item.guid()),
                pub_date: string_utils::option_date_to_option_string(item.pub_date()),
                source: convert_source(item.source()),
                content: None,
                itunes_ext: None,
                dublin_core_ext: None,
                ..Default::default()
            };
            rss_items.push(rss_item);
        }
    }
    rss_items
}


// Convert rss item enclosure from feed item enclosure
fn convert_enclosure(enc_opt: Option<Enclosure>) -> Option<rss::Enclosure>
{
    if enc_opt.is_none()
    {
        None
    }
    else
    {
        let enc = enc_opt.unwrap();
        let rss_enclosure = rss::Enclosure {
            url: enc.url().into_string(),
            length: enc.length().to_string(),
            mime_type: enc.mime_type().to_string(),
        };
        Some(rss_enclosure)
    }
}


// Convert rss item guid from feed item guid
fn convert_guid(guid_opt: Option<Guid>) -> Option<rss::Guid>
{
    if guid_opt.is_none()
    {
        None
    }
    else
    {
        let guid = guid_opt.unwrap();
        let rss_guid = rss::Guid {
            value: guid.value(),
            is_permalink: guid.permalink(),
        };
        Some(rss_guid)
    }
}


// Convert rss item source from feed item source
fn convert_source(src_opt: Option<Source>) -> Option<rss::Source>
{
    if src_opt.is_none()
    {
        None
    }
    else
    {
        let src = src_opt.unwrap();
        let rss_src = rss::Source {
            url: src.url().into_string(),
            title: src.title(),
        };
        Some(rss_src)
    }
}
