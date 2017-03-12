// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


use channels::{Category, CategoryBuilder, Channel, ChannelBuilder, Cloud, CloudBuilder, Enclosure, EnclosureBuilder,
               Guid, GuidBuilder, Image, ImageBuilder, Item, ItemBuilder, Source, SourceBuilder, TextInput,
               TextInputBuilder};
use rss;
use std::str::FromStr;
use utils::string_utils;


// Construct a new `Channel` and return it.
pub fn read(feed: &str) -> Result<Channel, String>
{
    convert_channel(rss::Channel::from_str(feed).unwrap())
}


// convert rss channel to feed channel
fn convert_channel(rss_channel: rss::Channel) -> Result<Channel, String>
{
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
        .categories(convert_categories(rss_channel.categories)?)
        .generator(rss_channel.generator)
        .docs(rss_channel.docs)
        .cloud(convert_cloud(rss_channel.cloud)?)
        .ttl(string_utils::option_string_to_option_i64(rss_channel.ttl)?)
        .image(convert_image(rss_channel.image)?)
        .rating(None)
        .text_input(convert_text_input(rss_channel.text_input)?)
        .skip_hours(convert_skip_hours(rss_channel.skip_hours)?)
        .skip_days(convert_skip_days(rss_channel.skip_days)?)
        .items(convert_items(rss_channel.items)?)
        .finalize()
}


// Convert rss categories to feed categories
fn convert_categories(rss_cats: Vec<rss::Category>) -> Result<Option<Vec<Category>>, String>
{
    if rss_cats.is_empty()
    {
        Ok(None)
    }
    else
    {
        let mut cats: Vec<Category> = Vec::new();
        for rss_cat in rss_cats
        {
            cats.push(CategoryBuilder::new().name(rss_cat.name.as_str())
                          .domain(rss_cat.domain)
                          .finalize()?);
        }
        Ok(Some(cats))
    }
}


// Convert rss channel cloud to feed channel cloud
fn convert_cloud(rss_cloud: Option<rss::Cloud>) -> Result<Option<Cloud>, String>
{
    match rss_cloud
    {
        Some(val) =>
        {
            Ok(Some(CloudBuilder::new().domain(val.domain.as_str())
                        .port(string_utils::string_to_i64(val.port.as_str())?)
                        .path(val.path.as_str())
                        .register_procedure(val.register_procedure.as_str())
                        .protocol(val.protocol.as_str())
                        .finalize()?))
        }
        None => Ok(None),
    }
}


// Convert rss channel image to feed channel image
fn convert_image(rss_image: Option<rss::Image>) -> Result<Option<Image>, String>
{
    match rss_image
    {
        Some(val) =>
        {
            Ok(Some(ImageBuilder::new().url(val.url.as_str())
                        .title(val.title.as_str())
                        .link(val.link.as_str())
                        .width(string_utils::option_string_to_option_i64(val.width)?)
                        .height(string_utils::option_string_to_option_i64(val.height)?)
                        .description(val.description)
                        .finalize()?))
        }
        None => Ok(None),
    }
}


// Convert rss channel text input to feed channel text input
fn convert_text_input(rss_text_input: Option<rss::TextInput>) -> Result<Option<TextInput>, String>
{
    match rss_text_input
    {
        Some(val) =>
        {
            Ok(Some(TextInputBuilder::new().title(val.title.as_str())
                        .description(val.description.as_str())
                        .name(val.name.as_str())
                        .link(val.link.as_str())
                        .finalize()?))
        }
        None => Ok(None),
    }
}


// Convert rss channel skip hours to feed channel skip hours
fn convert_skip_hours(rss_skip_hours: Vec<String>) -> Result<Option<Vec<i64>>, String>
{
    if rss_skip_hours.is_empty()
    {
        Ok(None)
    }
    else
    {
        let mut hours: Vec<i64> = Vec::new();
        for rss_hour in rss_skip_hours
        {
            hours.push(string_utils::string_to_i64(rss_hour.as_str())?)
        }
        Ok(Some(hours))
    }
}


// Convert rss channel skip days to feed channel skip days
fn convert_skip_days(rss_skip_days: Vec<String>) -> Result<Option<Vec<String>>, String>
{
    if rss_skip_days.is_empty()
    {
        Ok(None)
    }
    else
    {
        Ok(Some(rss_skip_days))
    }
}


// Convert rss channel items to feed channel items
fn convert_items(rss_items: Vec<rss::Item>) -> Result<Option<Vec<Item>>, String>
{
    if rss_items.is_empty()
    {
        Ok(None)
    }
    else
    {
        let mut items: Vec<Item> = Vec::new();
        for rss_item in rss_items
        {
            items.push(ItemBuilder::new().title(rss_item.title)
                           .link(rss_item.link)
                           .description(rss_item.description)
                           .author(rss_item.author)
                           .categories(convert_categories(rss_item.categories)?)
                           .comments(rss_item.comments)
                           .enclosure(convert_enclosure(rss_item.enclosure)?)
                           .guid(convert_guid(rss_item.guid)?)
                           .pub_date(rss_item.pub_date)
                           .source(convert_source(rss_item.source)?)
                           .finalize()?);
        }
        Ok(Some(items))
    }
}


// Convert rss item enclosure to feed item enclosure
fn convert_enclosure(rss_enc: Option<rss::Enclosure>) -> Result<Option<Enclosure>, String>
{
    match rss_enc
    {
        Some(val) =>
        {
            Ok(Some(EnclosureBuilder::new().url(val.url.as_str())
                        .length(string_utils::string_to_i64(val.length.as_str())?)
                        .mime_type(val.mime_type.as_str())
                        .finalize()?))
        }
        None => Ok(None),
    }
}


// Convert rss item guid to feed item guid
fn convert_guid(rss_guid: Option<rss::Guid>) -> Result<Option<Guid>, String>
{
    match rss_guid
    {
        Some(val) =>
        {
            Ok(Some(GuidBuilder::new().value(val.value.as_str())
                        .permalink(Some(val.is_permalink))
                        .finalize()?))
        }
        None => Ok(None),
    }
}


// Convert rss item source to feed item source
fn convert_source(rss_src: Option<rss::Source>) -> Result<Option<Source>, String>
{
    match rss_src
    {
        Some(val) =>
        {
            Ok(Some(SourceBuilder::new().url(val.url.as_str())
                        .title(val.title)
                        .finalize()?))
        }
        None => Ok(None),
    }
}
