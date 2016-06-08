// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

use chrono::*;
use errors;
use quick_xml::attributes::Attributes;
use std::{i64, str};
use std::str::FromStr;


pub fn attribute_to_string(attributes: Attributes, attr_name: &str) -> Option<String> {
    let mut name_vec: Vec<String> = Vec::new();
    let names = attributes.clone().map(|a| a.unwrap().0).collect::<Vec<_>>();
    for name in names.clone() {
        name_vec.push(str::from_utf8(name).expect(errors::utf8_to_str_error()).to_owned());
    }

    let attrs = attributes.clone().map(|a| a.unwrap().1).collect::<Vec<_>>();
    let index = match name_vec.binary_search(&attr_name.to_owned()) {
        Ok(value) => value,
        Err(err) => {
            return None;
        }
    };
    let attr = attrs.get(index).expect(errors::vec_get_error());
    let attr_str = str::from_utf8(attr).expect(errors::utf8_to_str_error());
    Some(attr_str.to_owned())
}


// Common code to convert attribute to i64.
pub fn attribute_to_i64(attributes: Attributes, attr_name: &str) -> Option<i64> {
    let attr = attribute_to_string(attributes, attr_name);
    if attr.is_none() {
        return None;
    }

    let attr_i64 = i64::from_str(attr.unwrap().as_str()).expect(errors::str_to_i64_error());

    Some(attr_i64)
}

// Common code to convert attribute to bool.
pub fn attribute_to_bool(attributes: Attributes, attr_name: &str) -> Option<bool> {
    let attr = attribute_to_string(attributes, attr_name);
    if attr.is_none() {
        return None;
    }

    let attr_bool = bool::from_str(attr.unwrap().as_str()).expect(errors::str_to_bool_error());
    Some(attr_bool)
}


// Common code to convert Option<String> to Option<DateTime<FixedOffset>>.
pub fn option_string_to_option_date(date_option: Option<String>) -> Option<DateTime<FixedOffset>> {
    if date_option.is_none() {
        return None;
    }
    let date_string = date_option.unwrap();
    let datetime = DateTime::parse_from_rfc2822(&date_string)
        .expect(errors::str_to_datetime_error());
    Some(datetime)
}
