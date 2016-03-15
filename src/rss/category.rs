// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

#[derive(Clone)]
pub struct Category {
    category: String,
    domain:   Option<String>,
}


impl Category {
    pub fn category(&self) -> String {
        self.category.clone()
    }


    pub fn domain(&self) -> Option<String> {
        self.domain.clone()
    }



}


pub struct CategoryBuilder {
    category: String,
    domain:   Option<String>,
}


impl CategoryBuilder {
    pub fn new() -> CategoryBuilder {
        CategoryBuilder {
            category: String::new(),
            domain:   None,
        }
    }


    pub fn category(&mut self, category: &str) -> &mut CategoryBuilder {
        self.category = category.to_string();
        self
    }


    pub fn domain(&mut self, domain: Option<String>) -> &mut CategoryBuilder {
        self.domain = domain;
        self
    }


    pub fn finalize(&self) -> Category {
        Category {
            category: self.category.clone(),
            domain:   self.domain.clone(),
        }
    }
}
