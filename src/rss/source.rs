// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

#[derive(Clone)]
pub struct Source {
    url:    String,
    source: String,
}


impl Source {
    pub fn url(&self) -> String {
        self.url.clone()
    }


    pub fn source(&self) -> String {
        self.source.clone()
    }
}


pub struct SourceBuilder {
    url:    String,
    source: String,
}


impl SourceBuilder {
    pub fn new() -> SourceBuilder {
        SourceBuilder {
            url:    String::new(),
            source: String::new(),
        }
    }


    pub fn url(&mut self, url: &str) -> &mut SourceBuilder {
        self.url = url.to_string();
        self
    }


    pub fn source(&mut self, source: &str) -> &mut SourceBuilder {
        self.source = source.to_string();
        self
    }


    pub fn finalize(&self) -> Source {
        Source {
            url:    self.url.clone(),
            source: self.source.clone(),
        }
    }
}
