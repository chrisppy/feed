// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! `FromUrl` Trait for `Channel`


use channels::FromUrl;
use curl::easy::Easy;
use rss::Channel;
use std::str::FromStr;
use utils::string_utils;

impl FromUrl for Channel
{
    /// Construct a `Channel` from a `Url`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate rss;
    /// extern crate feed;
    ///
    /// use feed::channels::FromUrl;
    /// use rss::Channel;
    ///
    /// fn main()
    /// {
    ///     let url = "https://feedpress.me/usererror.xml";
    ///     Channel::from_url(url).unwrap();
    /// }
    /// ```
    fn from_url(url: &str) -> Result<Channel, String>
    {
        let feed_url = string_utils::str_to_url(url)?;
        let mut xml = Vec::new();
        let mut handle = Easy::new();

        let handle_url = handle.url(feed_url.into_string().as_str());
        {
            let mut transfer = handle.transfer();
            transfer
                .write_function(|data| {
                                    xml.extend_from_slice(data);
                                    Ok(data.len())
                                })
                .unwrap();
            transfer.perform().unwrap();
        }

        if handle_url.is_err()
        {
            return Err(format!("Error: {:?}", handle_url.unwrap_err()));
        }

        let content_type = match handle.content_type()
        {
            Ok(val) => val.unwrap(),
            Err(err) => return Err(format!("Error: {}", err)),
        };

        if !content_type.contains("xml")
        {
            return Err("Url must end with .xml".to_owned());
        }

        match String::from_utf8(xml)
        {
            Ok(val) =>
            {
                match Channel::from_str(val.as_str())
                {
                    Ok(cval) => Ok(cval),
                    Err(err) => Err(format!("Error: {}", err)),
                }
            }
            Err(err) => Err(format!("Error: {}", err)),
        }
    }
}
