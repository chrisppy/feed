// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.

use chrono::*;
use std::i64;
use std::str::FromStr;
use url::Url;


// Common code to convert Option<String> to Option<i64>
pub fn option_string_to_option_i64(o: Option<String>) -> Result<Option<i64>, String>
{
    match o
    {
        Some(val) =>
        {
            match string_to_i64(val.as_str())
            {
                Ok(val) => Ok(Some(val)),
                Err(err) => Err(err),
            }
        }
        None => Ok(None),
    }
}


// Common code to convert Option<String> from i64
pub fn i64_to_option_string(i: i64) -> Result<Option<String>, String>
{
    Ok(Some(i.to_string()))
}


// Common code to convert Option<String> from Option<i64>
pub fn option_i64_to_option_string(o: Option<i64>) -> Result<Option<String>, String>
{
    match o
    {
        Some(val) => i64_to_option_string(val),
        None => Ok(None),
    }
}


// Common code to convert String to i64
pub fn string_to_i64(s: &str) -> Result<i64, String>
{
    match i64::from_str(s)
    {
        Ok(val) => Ok(val),
        Err(err) => Err(format!("Error: {}", err)),
    }
}


// Common code to convert Option<String> to Option<DateTime<FixedOffset>>.
pub fn option_string_to_option_date(date_option: Option<String>) -> Result<Option<DateTime<FixedOffset>>, String>
{
    match date_option
    {
        Some(val) =>
        {
            match DateTime::parse_from_rfc2822(val.as_str())
            {
                Ok(val) => Ok(Some(val)),
                Err(err) => Err(format!("Error: {}", err)),
            }
        }
        None => Ok(None),
    }
}


// Common code to convert Option<String> from Option<DateTime<FixedOffset>>.
pub fn option_date_to_option_string(date_option: Option<DateTime<FixedOffset>>) -> Result<Option<String>, String>
{
    match date_option
    {
        Some(val) => Ok(Some(val.to_rfc2822())),
        None => Ok(None),
    }
}


// Common code to convert str to Url.
pub fn str_to_url(s: &str) -> Result<Url, String>
{
    match Url::parse(s)
    {
        Ok(val) => Ok(val),
        Err(err) => Err(format!("Error: {}", err)),
    }
}


// Common code to convert Option<Url> to Option<String>
pub fn option_url_to_option_string(o: Option<Url>) -> Result<Option<String>, String>
{
    match o
    {
        Some(val) => Ok(Some(val.into_string())),
        None => Ok(None),
    }
}
