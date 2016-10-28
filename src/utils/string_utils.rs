// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

use chrono::*;
use errors;
use std::i64;
use std::str::FromStr;


// Common code to convert Option<String> to Option<i64>
pub fn option_string_to_option_i64(o: Option<String>) -> Option<i64> {
    if o.is_some() {
        let s = o.unwrap();
        let i = i64::from_str(s.as_str())
            .expect(errors::str_to_i64_error().as_str());
        Some(i)
    } else {
        None
    }
}


// Common code to convert Option<String> to Option<DateTime<FixedOffset>>.
pub fn option_string_to_option_date(date_option: Option<String>) -> Option<DateTime<FixedOffset>> {
    if date_option.is_none() {
        return None;
    }
    let date_string = date_option.unwrap();
    let datetime = DateTime::parse_from_rfc2822(&date_string)
        .expect(errors::str_to_datetime_error().as_str());
    Some(datetime)
}
