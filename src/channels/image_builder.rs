// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under image can be retrieved by using the methods under `Image`
//! and the fields can be set for image by using the methods under
//! `ImageBuilder`.


use channels::ImageBuilder;
use rss::Image;
use utils::string_utils;


impl ImageBuilder
{
    /// Construct a new `ImageBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let image_builder = ImageBuilder::new();
    /// ```
    pub fn new() -> ImageBuilder
    {
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
    /// image_builder.url("http://jupiterbroadcasting.com/images/LAS-300-Badge.
    /// jpg");
    /// ```
    pub fn url(&mut self, url: &str) -> &mut ImageBuilder
    {
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
    pub fn title(&mut self, title: &str) -> &mut ImageBuilder
    {
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
    pub fn link(&mut self, link: &str) -> &mut ImageBuilder
    {
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
    pub fn width(&mut self, width: Option<i64>) -> &mut ImageBuilder
    {
        self.width = width;
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
    pub fn height(&mut self, height: Option<i64>) -> &mut ImageBuilder
    {
        self.height = height;
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
    pub fn description(&mut self, description: Option<String>) -> &mut ImageBuilder
    {
        self.description = description;
        self
    }


    /// Validate the contents of `Image`.
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
    ///         .validate().unwrap()
    ///         .finalize().unwrap();
    /// ```
    pub fn validate(&mut self) -> Result<&mut ImageBuilder, String>
    {
        let url_string = self.url.clone();
        if !url_string.ends_with(".jpeg") && !url_string.ends_with(".jpg") && !url_string.ends_with(".png") &&
           !url_string.ends_with(".gif")
        {
            return Err("Image Url must end with .jpeg, .png, or .gif".to_owned());
        }

        string_utils::str_to_url(url_string.as_str())?;
        string_utils::str_to_url(self.link.as_str())?;

        let width_opt = self.width;
        if width_opt.is_some()
        {
            let width = width_opt.unwrap();
            if width > 144
            {
                return Err("Image width cannot be greater than 144.".to_owned());
            }
            else if width < 0
            {
                return Err("Image width cannot be a negative value.".to_owned());
            }
        }

        let height_opt = self.height;
        if height_opt.is_some()
        {
            let height = height_opt.unwrap();
            if height > 144
            {
                return Err("Image height cannot be greater than 400.".to_owned());
            }
            else if height < 0
            {
                return Err("Image height cannot be a negative value.".to_owned());
            }
        }

        Ok(self)
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
    pub fn finalize(&self) -> Result<Image, String>
    {

        let width = match self.width
        {
            Some(val) => string_utils::i64_to_option_string(val)?,
            None => string_utils::i64_to_option_string(88)?,
        };

        let height = match self.height
        {
            Some(val) => string_utils::i64_to_option_string(val)?,
            None => string_utils::i64_to_option_string(31)?,
        };

        Ok(Image {
               url: self.url.clone(),
               title: self.title.clone(),
               link: self.link.clone(),
               width: width,
               height: height,
               description: self.description.clone(),
           })
    }
}
