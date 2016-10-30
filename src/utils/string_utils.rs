// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

use chrono::*;
use errors;
use std::i64;
use std::str::FromStr;
use url::Url;


// Common code to convert Option<String> to Option<i64>
pub fn option_string_to_option_i64(o: Option<String>) -> Option<i64> {
    if o.is_some() {
        let s = o.unwrap();
        let i = string_to_i64(s.as_str());
        Some(i)
    } else {
        None
    }
}


// Common code to convert Option<String> from i64
pub fn i64_to_option_string(i: i64) -> Option<String> {
    let s = i.to_string();
    Some(s)
}


// Common code to convert Option<String> from Option<i64>
pub fn option_i64_to_option_string(o: Option<i64>) -> Option<String> {
    if o.is_none() {
        None
    } else {
        i64_to_option_string(o.unwrap())
    }
}


// Common code to convert String to i64
pub fn string_to_i64(s: &str) -> i64 {
    i64::from_str(s).expect(errors::str_to_i64_error().as_str())
}


// Common code to convert Option<String> to Option<DateTime<FixedOffset>>.
pub fn option_string_to_option_date(date_option: Option<String>) -> Option<DateTime<FixedOffset>> {
    if date_option.is_none() {
        None
    } else {
        let date_string = date_option.unwrap();
        let datetime = DateTime::parse_from_rfc2822(&date_string)
            .expect(errors::str_to_datetime_error().as_str());
        Some(datetime)
    }
}


// Common code to convert Option<String> from Option<DateTime<FixedOffset>>.
pub fn option_date_to_option_string(date_option: Option<DateTime<FixedOffset>>) -> Option<String> {
    if date_option.is_none() {
        None
    } else {
        let s = date_option.unwrap().to_rfc2822();
        Some(s)
    }
}


// Common code to convert str to Url.
pub fn str_to_url(s: &str, e: &str) -> Url {
    Url::parse(s).expect(errors::str_to_url_error(e).as_str())
}


// Common code to convert Option<Url> to Option<String>
pub fn option_url_to_option_string(o: Option<Url>) -> Option<String> {
    if o.is_none() {
        None
    } else {
        let s = o.unwrap().into_string();
        Some(s)
    }
}
