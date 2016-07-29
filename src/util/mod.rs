// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! All the utility functions.

use chrono::*;
use errors;
use quick_xml::attributes::Attributes;
use std::{i64, str};
use std::str::FromStr;

/// Common code to convert attribute to string
///
/// # Examples
///
/// ```
/// extern crate quick_xml;
/// extern crate feed;
///
/// use quick_xml::{XmlReader, Event};
/// use std::str;
/// use feed::util;
///
/// fn main() {
///     let xml = r#"<tag1 att1 = "test" att2 = "test2">
///                     <tag2><!--Test comment-->Test</tag2>
///                     <tag2>Test 2</tag2>
///                 </tag1>"#;
///     let reader = XmlReader::from(xml).trim_text(true);
///     let mut count = 0;
///     let mut txt = Vec::new();
///     for r in reader {
///         match r {
///             Ok(Event::Start(ref e)) => {
///                 match e.name() {
///                     b"tag1" => {
///                         let att2 = util::attribute_to_string(e.attributes(), "att2");
///                         assert_eq!("test2".to_owned(), att2.expect("att2 cannot be None"));
///                     }
///                     b"tag2" => count += 1,
///                     _ => (),
///                 }
///             }
///             Ok(Event::Text(e)) => txt.push(e.into_string()),
///             Err((e, pos)) => panic!("{:?} at position {}", e, pos),
///             _ => (),
///         }
///     }
/// }
/// ```
/// ```
/// extern crate quick_xml;
/// extern crate feed;
///
/// use quick_xml::{XmlReader, Event};
/// use std::str;
/// use feed::util;
///
/// fn main() {
///     let xml = r#"<enclosure url="http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jbmirror/linuxactionshowep409.ogg" length="69796480" type="audio/ogg" />"#;
///     let reader = XmlReader::from(xml).trim_text(true);
///     let mut count = 0;
///     let mut txt = Vec::new();
///     for r in reader {
///         match r {
///             Ok(Event::Start(ref e)) => {
///                 match e.name() {
///                     b"tag1" => {
///                         let url = util::attribute_to_string(e.attributes(), "url");
///                         assert_eq!("http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jbmirror/linuxactionshowep409.ogg".to_owned(), url.expect("url cannot be None"));
///                     }
///                     b"tag2" => count += 1,
///                     _ => (),
///                 }
///             }
///             Ok(Event::Text(e)) => txt.push(e.into_string()),
///             Err((e, pos)) => panic!("{:?} at position {}", e, pos),
///             _ => (),
///         }
///     }
/// }
/// ```
pub fn attribute_to_string(attributes: Attributes, attr_name: &str) -> Option<String> {
    let mut name_vec: Vec<String> = Vec::new();
    let names = attributes.clone().map(|a| a.unwrap().0).collect::<Vec<_>>();
    for name in names.clone() {
        name_vec.push(str::from_utf8(name).expect(errors::utf8_to_str_error()).to_owned());
    }

    let attrs = attributes.clone().map(|a| a.unwrap().1).collect::<Vec<_>>();

    // println!("name_vec {:?}", name_vec);

    // println!("attr_name {:?}", attr_name);

    if name_vec.binary_search(&attr_name.to_owned()).is_ok() {
        let index = name_vec.binary_search(&attr_name.to_owned()).unwrap();
        let attr = attrs.get(index).expect(errors::vec_get_error());
        let attr_str = str::from_utf8(attr).expect(errors::utf8_to_str_error());

        // println!("attr {:?}", attr_str);

        return Some(attr_str.to_owned());
    } else {
        return None;
    }
}


/// Common code to convert attribute to i64.
pub fn attribute_to_i64(attributes: Attributes, attr_name: &str) -> Option<i64> {
    let attr = attribute_to_string(attributes, attr_name);
    if attr.is_none() {
        return None;
    }

    let attr_i64 = i64::from_str(attr.unwrap().as_str()).expect(errors::str_to_i64_error());

    Some(attr_i64)
}

/// Common code to convert attribute to bool.
pub fn attribute_to_bool(attributes: Attributes, attr_name: &str) -> Option<bool> {
    let attr = attribute_to_string(attributes, attr_name);
    if attr.is_none() {
        return None;
    }

    let attr_bool = bool::from_str(attr.unwrap().as_str()).expect(errors::str_to_bool_error());
    Some(attr_bool)
}


/// Common code to convert Option<String> to Option<DateTime<FixedOffset>>.
pub fn option_string_to_option_date(date_option: Option<String>) -> Option<DateTime<FixedOffset>> {
    if date_option.is_none() {
        return None;
    }
    let date_string = date_option.unwrap();
    let datetime = DateTime::parse_from_rfc2822(&date_string)
        .expect(errors::str_to_datetime_error());
    Some(datetime)
}
