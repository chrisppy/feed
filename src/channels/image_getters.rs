// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under image can be retrieved by using the methods under `Image`.


use channels::ImageGetters;
use rss::Image;


impl ImageGetters for Image
{
    /// Get the url that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .url(url)
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(url.to_owned(), image.url().into_string());
    /// ```
    fn url(&self) -> String
    {
        self.url.clone()
    }


    /// Get the title that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let title = "LAS 300 Logo";
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .title(title)
    ///     .url(url)
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(title.to_owned(), image.title());
    /// ```
    fn title(&self) -> String
    {
        self.title.clone()
    }


    /// Get the link that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com/";
    ///
    /// let image = ImageBuilder::new()
    ///     .url(url)
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(link.to_owned(), image.link().into_string());
    /// ```
    fn link(&self) -> String
    {
        self.link.clone()
    }


    /// Get the width that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let default: i64 = 88;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .width(None)
    ///     .url(url)
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(default, image.width());
    /// ```
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let width: i64 = 60;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .width(Some(width))
    ///     .url(url)
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(width, image.width());
    /// ```
    ///
    /// ```
    fn width(&self) -> Option<String>
    {
        self.width.clone()
    }


    /// Get the height that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let default: i64 = 31;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .height(None)
    ///     .url(url)
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(default, image.height());
    /// ```
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let height: i64 = 60;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .height(Some(height))
    ///     .url(url)
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(height, image.height());
    /// ```
    fn height(&self) -> Option<String>
    {
        self.height.clone()
    }


    /// Get the description that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .description(None)
    ///     .url(url)
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    /// assert!(image.description().is_none());
    /// ```
    ///
    /// ```
    /// use feed::channels::ImageBuilder;
    ///
    /// let description_string = "This is a test".to_owned();
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .description(Some(description_string.clone()))
    ///     .url(url)
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let description_option = image.description();
    /// assert!(description_option.is_some());
    ///
    /// let description = description_option.unwrap();
    /// assert_eq!(description.clone(), description);
    /// ```
    fn description(&self) -> Option<String>
    {
        self.description.clone()
    }
}
