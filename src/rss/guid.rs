// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

#[derive(Clone)]
pub struct Guid {
    is_permalink: bool,
    guid:         String,
}


impl Guid {
    pub fn is_permalink(&self) -> bool {
        self.is_permalink.clone()
    }


    pub fn guid(&self) -> String {
        self.guid.clone()
    }
}


pub struct GuidBuilder {
    is_permalink: bool,
    guid:         String,
}


impl GuidBuilder {
    pub fn new() -> GuidBuilder {
        GuidBuilder {
            is_permalink: true,
            guid:         String::new(),
        }
    }


    pub fn is_permalink(&mut self, is_permalink: Option<bool>) -> &mut GuidBuilder {
        if is_permalink.is_some() {
            self.is_permalink = is_permalink.unwrap();
        }
        self
    }


    pub fn guid(&mut self, guid: &str) -> &mut GuidBuilder {
        self.guid = guid.to_string();
        self
    }


    pub fn finalize(&self) -> Guid {
        Guid {
            is_permalink: self.is_permalink.clone(),
            guid:         self.guid.clone(),
        }
    }
}
