// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields under image can be retrieved by using the methods under `Image`
//! and the fields can be set for image by using the methods under `ImageBuilder`.


use channels::{Image, ImageBuilder};
use errors;
use utils::string_utils;


impl ImageBuilder {
    /// Construct a new `ImageBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let image_builder = ImageBuilder::new();
    /// ```
    pub fn new() -> ImageBuilder {
        ImageBuilder::default()
    }


    /// Set the url that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg");
    /// ```
    pub fn url(&mut self, url: &str) -> &mut ImageBuilder {
        self.url = url.to_owned();
        self
    }


    /// Set the title that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.title("LAS 300 Logo");
    /// ```
    pub fn title(&mut self, title: &str) -> &mut ImageBuilder {
        self.title = title.to_owned();
        self
    }


    /// Set the link that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.link("http://www.jupiterbroadcasting.com/");
    /// ```
    pub fn link(&mut self, link: &str) -> &mut ImageBuilder {
        self.link = link.to_owned();
        self
    }


    /// Set the width that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.width(Some(88));
    /// ```
    pub fn width(&mut self, width: Option<i64>) -> &mut ImageBuilder {
        if width.is_some() {
            let max_width = 144;
            let mut size = width.unwrap();
            if size > max_width {
                size = max_width;
            } else if size < 0 {
                panic!(errors::negative_error("image width", size));
            }
            self.width = size;
        } else {
            self.width = 88;
        }
        self
    }


    /// Set the height that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.height(Some(88));
    /// ```
    pub fn height(&mut self, height: Option<i64>) -> &mut ImageBuilder {
        if height.is_some() {
            let max_height = 400;
            let mut size = height.unwrap();
            if size > max_height {
                size = max_height;
            } else if size < 0 {
                panic!(errors::negative_error("image height", size));
            }
            self.height = size;
        } else {
            self.height = 31;
        }
        self
    }


    /// Set the description that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.description(Some("This is a test".to_owned()));
    /// ```
    pub fn description(&mut self, description: Option<String>) -> &mut ImageBuilder {
        self.description = description;
        self
    }


    /// Construct the `Image` from the `ImageBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let image = ImageBuilder::new()
    ///         .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///         .title("LAS 300 Logo")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .width(Some(88))
    ///         .height(Some(88))
    ///         .description(Some("This is a test".to_owned()))
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Image {
        let url_string = self.url.clone();
        if !url_string.ends_with(".jpeg") && !url_string.ends_with(".jpg") &&
           !url_string.ends_with(".png") && !url_string.ends_with(".gif") {
            panic!(errors::image_url_error());
        }
        let url = string_utils::str_to_url(url_string.as_str(), "Image Url");

        let link_string = self.link.clone();
        let link = string_utils::str_to_url(link_string.as_str(), "Image Link");

        Image {
            url: url,
            title: self.title.clone(),
            link: link,
            width: self.width,
            height: self.height,
            description: self.description.clone(),
        }
    }
}
