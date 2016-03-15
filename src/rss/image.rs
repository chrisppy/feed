// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

#[derive(Clone)]
pub struct Image {
    url:         String,
    title:       String,
    link:        String,
    width:       i64,
    height:      i64,
    description: Option<String>,
}


impl Image {
    pub fn url(&self) -> String {
        self.url.clone()
    }


    pub fn title(&self) -> String {
        self.title.clone()
    }


    pub fn link(&self) -> String {
        self.link.clone()
    }


    pub fn width(&self) -> i64 {
        self.width.clone()
    }


    pub fn height(&self) -> i64 {
        self.height.clone()
    }


    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }
}


pub struct ImageBuilder {
    url:         String,
    title:       String,
    link:        String,
    width:       i64,
    height:      i64,
    description: Option<String>
}


impl ImageBuilder {
    pub fn new() -> ImageBuilder {
        ImageBuilder {
            url:         String::new(),
            title:       String::new(),
            link:        String::new(),
            width:       88,
            height:      31,
            description: None,
        }
    }


    pub fn url(&mut self, url: &str) -> &mut ImageBuilder {
        self.url = url.to_string();
        self
    }


    pub fn title(&mut self, title: &str) -> &mut ImageBuilder {
        self.title = title.to_string();
        self
    }


    pub fn link(&mut self, link: &str) -> &mut ImageBuilder {
        self.link = link.to_string();
        self
    }


    pub fn width(&mut self, width: Option<i64>) -> &mut ImageBuilder {
        if width.is_some() {
            let max_width = 144;
            let mut size = width.unwrap();
            if size > max_width {
                size = max_width;
            }
            self.width = size;
        }
        self
    }


    pub fn height(&mut self, height: Option<i64>) -> &mut ImageBuilder {
        if height.is_some() {
            let max_height = 400;
            let mut size = height.unwrap();
            if size > max_height {
                size = max_height;
            }
            self.height = size;
        }
        self
    }


    pub fn description(&mut self, description: Option<String>) -> &mut ImageBuilder {
        self.description = description;
        self
    }


    pub fn finalize(&self) -> Image {
        Image {
            url:         self.url.clone(),
            title:       self.title.clone(),
            link:        self.link.clone(),
            width:       self.width.clone(),
            height:      self.height.clone(),
            description: self.description.clone(),
        }
    }
}
