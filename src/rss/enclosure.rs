// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

#[derive(Clone)]
pub struct Enclosure {
    url:            String,
    length:         i64,
    enclosure_type: String,
}


impl Enclosure {
    pub fn url(&self) -> String {
        self.url.clone()
    }


    pub fn length(&self) -> i64 {
        self.length.clone()
    }


    pub fn enclosure_type(&self) -> String {
        self.enclosure_type.clone()
    }
}


pub struct EnclosureBuilder {
    url:            String,
    length:         i64,
    enclosure_type: String,
}


impl EnclosureBuilder {
    pub fn new() -> EnclosureBuilder {
        EnclosureBuilder {
            url:            String::new(),
            length:         0,
            enclosure_type: String::new(),
        }
    }


    pub fn url(&mut self, url: &str) -> &mut EnclosureBuilder {
        self.url = url.to_string();
        self
    }


    pub fn length(&mut self, length: i64) -> &mut EnclosureBuilder {
        self.length = length;
        self
    }


    pub fn enclosure_type(&mut self, enclosure_type: &str) -> &mut EnclosureBuilder {
        self.enclosure_type = enclosure_type.to_string();
        self
    }


    pub fn finalize(&self) -> Enclosure {
        Enclosure {
            url:            self.url.clone(),
            length:         self.length.clone(),
            enclosure_type: self.enclosure_type.clone(),
        }
    }
}
