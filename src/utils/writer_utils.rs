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
pub fn write(channel: &Channel) -> Result<Vec<u8>, String>
{
    Ok(convert_channel(channel)?.to_string().into_bytes())
}


// convert rss channel from feed channel
fn convert_channel(channel: &Channel) -> Result<rss::Channel, String>
{
    Ok(rss::Channel {
           title: channel.title(),
           link: channel.link().into_string(),
           description: channel.description(),
           language: channel.language(),
           copyright: channel.copyright(),
           managing_editor: channel.managing_editor(),
           webmaster: channel.web_master(),
           pub_date: string_utils::option_date_to_option_string(channel.pub_date())?,
           last_build_date: string_utils::option_date_to_option_string(channel.last_build_date())?,
           categories: convert_categories(channel.categories())?,
           generator: channel.generator(),
           docs: string_utils::option_url_to_option_string(channel.docs())?,
           cloud: convert_cloud(channel.cloud())?,
           ttl: string_utils::option_i64_to_option_string(channel.ttl())?,
           image: convert_image(channel.image())?,
           // rating: channel.rating(),
           text_input: convert_text_input(channel.text_input())?,
           skip_hours: convert_skip_hours(channel.skip_hours())?,
           skip_days: convert_skip_days(channel.skip_days())?,
           items: convert_items(channel.items())?,
           itunes_ext: None,
           dublin_core_ext: None,
           ..Default::default()
       })
}


// Convert rss categories from feed categories
fn convert_categories(cats: Option<Vec<Category>>) -> Result<Vec<rss::Category>, String>
{
    let mut rss_cats = Vec::new();
    match cats
    {
        Some(val) =>
        {
            for cat in val
            {
                let domain = match cat.domain()
                {
                    Some(val) => Some(val.into_string()),
                    None => None,
                };

                rss_cats.push(rss::Category {
                                  name: cat.name(),
                                  domain: domain,
                              });
            }
            Ok(rss_cats)
        }
        None => Ok(rss_cats),
    }
}


// Convert rss channel cloud from feed channel cloud
fn convert_cloud(cloud: Option<Cloud>) -> Result<Option<rss::Cloud>, String>
{
    match cloud
    {
        Some(val) =>
        {
            Ok(Some(rss::Cloud {
                        domain: val.domain().into_string(),
                        port: val.port().to_string(),
                        path: val.path(),
                        register_procedure: val.register_procedure(),
                        protocol: val.protocol().into_string(),
                    }))
        }
        None => Ok(None),
    }
}


// Convert rss channel image from feed channel image
fn convert_image(image: Option<Image>) -> Result<Option<rss::Image>, String>
{
    match image
    {
        Some(val) =>
        {
            Ok(Some(rss::Image {
                        url: val.url().into_string(),
                        title: val.title(),
                        link: val.link().into_string(),
                        width: string_utils::i64_to_option_string(val.width())?,
                        height: string_utils::i64_to_option_string(val.height())?,
                        description: val.description(),
                    }))
        }
        None => Ok(None),
    }
}


// Convert rss channel text input from feed channel text input
fn convert_text_input(text_input: Option<TextInput>) -> Result<Option<rss::TextInput>, String>
{
    match text_input
    {
        Some(val) =>
        {
            Ok(Some(rss::TextInput {
                        title: val.title(),
                        description: val.description(),
                        name: val.name(),
                        link: val.link().into_string(),
                    }))
        }
        None => Ok(None),
    }
}


// Convert rss channel skip hours from feed channel skip hours
fn convert_skip_hours(skip_hours: Option<Vec<i64>>) -> Result<Vec<String>, String>
{
    let mut rss_skip_hours = Vec::new();
    match skip_hours
    {
        Some(val) =>
        {
            for hour in val
            {
                rss_skip_hours.push(hour.to_string());
            }
            Ok(rss_skip_hours)
        }
        None => Ok(rss_skip_hours),
    }
}



// Convert rss channel skip days from feed channel skip days
fn convert_skip_days(skip_days: Option<Vec<Day>>) -> Result<Vec<String>, String>
{
    let mut rss_skip_days = Vec::new();
    match skip_days
    {
        Some(val) =>
        {
            for day in val
            {
                rss_skip_days.push(day.into_string());
            }
            Ok(rss_skip_days)
        }
        None => Ok(rss_skip_days),
    }
}


// Convert rss channel items from feed channel items
fn convert_items(items_opt: Option<Vec<Item>>) -> Result<Vec<rss::Item>, String>
{
    let mut rss_items = Vec::new();
    if items_opt.is_some()
    {
        let items = items_opt.unwrap();
        for item in items
        {
            let rss_item = rss::Item {
                title: item.title(),
                link: string_utils::option_url_to_option_string(item.link())?,
                description: item.description(),
                author: item.author(),
                categories: convert_categories(item.categories())?,
                comments: string_utils::option_url_to_option_string(item.comments())?,
                enclosure: convert_enclosure(item.enclosure())?,
                guid: convert_guid(item.guid())?,
                pub_date: string_utils::option_date_to_option_string(item.pub_date())?,
                source: convert_source(item.source())?,
                content: None,
                itunes_ext: None,
                dublin_core_ext: None,
                ..Default::default()
            };
            rss_items.push(rss_item);
        }
    }
    Ok(rss_items)
}


// Convert rss item enclosure from feed item enclosure
fn convert_enclosure(enc: Option<Enclosure>) -> Result<Option<rss::Enclosure>, String>
{
    match enc
    {
        Some(val) =>
        {
            Ok(Some(rss::Enclosure {
                        url: val.url().into_string(),
                        length: val.length().to_string(),
                        mime_type: val.mime_type().to_string(),
                    }))
        }
        None => Ok(None),
    }
}


// Convert rss item guid from feed item guid
fn convert_guid(guid: Option<Guid>) -> Result<Option<rss::Guid>, String>
{
    match guid
    {
        Some(val) =>
        {
            Ok(Some(rss::Guid {
                        value: val.value(),
                        is_permalink: val.permalink(),
                    }))
        }
        None => Ok(None),
    }
}


// Convert rss item source from feed item source
fn convert_source(src: Option<Source>) -> Result<Option<rss::Source>, String>
{
    match src
    {
        Some(val) =>
        {
            Ok(Some(rss::Source {
                        url: val.url().into_string(),
                        title: val.title(),
                    }))
        }
        None => Ok(None),
    }
}
