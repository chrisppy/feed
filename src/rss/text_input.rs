// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

#[derive(Clone)]
pub struct TextInput {
    title:       String,
    description: String,
    name:        String,
    link:        String,
}


impl TextInput {
    pub fn title(&self) -> String {
        self.title.clone()
    }


    pub fn description(&self) -> String {
        self.description.clone()
    }


    pub fn name(&self) -> String {
        self.name.clone()
    }


    pub fn link(&self) -> String {
        self.link.clone()
    }
}


pub struct TextInputBuilder {
    title:       String,
    description: String,
    name:        String,
    link:        String,
}


impl TextInputBuilder {
    pub fn new() -> TextInputBuilder {
        TextInputBuilder {
            title:       String::new(),
            description: String::new(),
            name:        String::new(),
            link:        String::new(),
        }
    }


    pub fn title(&mut self, title: &str) -> &mut TextInputBuilder {
        self.title = title.to_string();
        self
    }


    pub fn description(&mut self, description: &str) -> &mut TextInputBuilder {
        self.description = description.to_string();
        self
    }


    pub fn name(&mut self, name: &str) -> &mut TextInputBuilder {
        self.name = name.to_string();
        self
    }


    pub fn link(&mut self, link: &str) -> &mut TextInputBuilder {
        self.link = link.to_string();
        self
    }


    pub fn finalize(&self) -> TextInput {
        TextInput {
            title:       self.title.clone(),
            description: self.description.clone(),
            name:        self.name.clone(),
            link:        self.link.clone(),
        }
    }
}
