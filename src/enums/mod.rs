// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! All of the enum.


/// Enumerations of protocols for `Cloud`.
#[derive(Clone, Debug)]
pub enum CloudProtocol {
    /// http-post
    HttpPost,

    /// xml-rpc
    XmlRpc,

    /// soap
    Soap,
}


impl CloudProtocol {
    /// Convert `&str` to `CloudProtocol`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::enums::CloudProtocol;
    ///
    /// let s = "soap";
    /// let e = CloudProtocol::to_enum(s);
    ///
    /// assert_eq!(s.to_owned(), e.into_string());
    /// ```
    pub fn to_enum(s: &str) -> CloudProtocol {
        match s {
            "http-post" => CloudProtocol::HttpPost,
            "xml-rpc" => CloudProtocol::XmlRpc,
            "soap" => CloudProtocol::Soap,
            _ => panic!(format!("Invalid value: {}", s)),
        }
    }


    /// Convert `CloudProtocol` to `String`.
    pub fn into_string(self) -> String {
        match self {
            CloudProtocol::HttpPost => "http-post".to_owned(),
            CloudProtocol::XmlRpc => "xml-rpc".to_owned(),
            CloudProtocol::Soap => "soap".to_owned(),
        }
    }
}


/// Enumerations of protocols for `SkipDays`.
#[derive(Clone, Debug)]
pub enum Day {
    /// Monday
    Monday,

    /// Tuesday
    Tuesday,

    /// Wednesday
    Wednesday,

    /// Thursday
    Thursday,

    /// Friday
    Friday,

    /// Saturday
    Saturday,

    /// Sunday
    Sunday,
}

impl Day {
    /// Convert `&str` to `Day`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::enums::Day;
    ///
    /// let s = "Tuesday";
    /// let e = Day::to_enum(s);
    ///
    /// assert_eq!(s.to_owned(), e.into_string());
    /// ```
    pub fn to_enum(s: &str) -> Day {
        match s {
            "Monday" => Day::Monday,
            "Tuesday" => Day::Tuesday,
            "Wednesday" => Day::Wednesday,
            "Thursday" => Day::Thursday,
            "Friday" => Day::Friday,
            "Saturday" => Day::Saturday,
            "Sunday" => Day::Sunday,
            _ => panic!(format!("Invalid value: {}", s)),
        }
    }


    /// Convert `Day` to `String`.
    pub fn into_string(self) -> String {
        match self {
            Day::Monday => "Monday".to_owned(),
            Day::Tuesday => "Tuesday".to_owned(),
            Day::Wednesday => "Wednesday".to_owned(),
            Day::Thursday => "Thursday".to_owned(),
            Day::Friday => "Friday".to_owned(),
            Day::Saturday => "Saturday".to_owned(),
            Day::Sunday => "Sunday".to_owned(),
        }
    }
}
