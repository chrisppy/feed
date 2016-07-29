// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields under image can be retrieved by using the methods under `Image`.

use rss::Image;
use url::Url;

impl Image {
    /// Get the url that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ImageBuilder;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    /// let image = ImageBuilder::new()
    ///     .url(url)
    ///     .title("LAS 300 Logo")
    ///     .link("http://jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert_eq!(url.to_owned(), image.url().into_string());
    /// ```
    pub fn url(&self) -> Url {
        self.url.clone()
    }


    /// Get the title that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ImageBuilder;
    ///
    /// let title = "LAS 300 Logo";
    /// let image = ImageBuilder::new()
    ///     .title(title)
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .link("http://jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert_eq!(title.to_owned(), image.title());
    /// ```
    pub fn title(&self) -> String {
        self.title.clone()
    }


    /// Get the link that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ImageBuilder;
    ///
    /// let link = "http://www.jupiterbroadcasting.com/";
    /// let image = ImageBuilder::new()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .link(link)
    ///     .finalize();
    /// assert_eq!(link.to_owned(), image.link().into_string());
    /// ```
    pub fn link(&self) -> Url {
        self.link.clone()
    }


    /// Get the width that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ImageBuilder;
    ///
    /// let default: i64 = 88;
    /// let image = ImageBuilder::new()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .link("http://jupiterbroadcasting.com/")
    ///     .width(None)
    ///     .finalize();
    /// assert_eq!(default, image.width());
    /// ```
    ///
    /// ```
    /// use feed::rss::ImageBuilder;
    ///
    /// let width: i64 = 60;
    /// let image = ImageBuilder::new()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .link("http://jupiterbroadcasting.com/")
    ///     .width(Some(width))
    ///     .finalize();
    /// assert_eq!(width, image.width());
    /// ```
    ///
    /// ```
    /// use feed::rss::ImageBuilder;
    ///
    /// let width: i64 = 777;
    /// let max: i64 = 144;
    /// let image = ImageBuilder::new()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .link("http://jupiterbroadcasting.com/")
    ///     .width(Some(width))
    ///     .finalize();
    /// assert_eq!(max, image.width());
    /// ```
    pub fn width(&self) -> i64 {
        self.width
    }


    /// Get the height that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ImageBuilder;
    ///
    /// let default: i64 = 31;
    /// let image = ImageBuilder::new()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .link("http://jupiterbroadcasting.com/")
    ///     .height(None)
    ///     .finalize();
    /// assert_eq!(default, image.height());
    /// ```
    ///
    /// ```
    /// use feed::rss::ImageBuilder;
    ///
    /// let height: i64 = 60;
    /// let image = ImageBuilder::new()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .link("http://jupiterbroadcasting.com/")
    ///     .height(Some(height))
    ///     .finalize();
    /// assert_eq!(height, image.height());
    /// ```
    ///
    /// ```
    /// use feed::rss::ImageBuilder;
    ///
    /// let height: i64 = 777;
    /// let max: i64 = 400;
    /// let image = ImageBuilder::new()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .link("http://jupiterbroadcasting.com/")
    ///     .height(Some(height))
    ///     .finalize();
    /// assert_eq!(max, image.height());
    /// ```
    pub fn height(&self) -> i64 {
        self.height
    }


    /// Get the description that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ImageBuilder;
    ///
    /// let image = ImageBuilder::new()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .link("http://jupiterbroadcasting.com/")
    ///     .description(None)
    ///     .finalize();
    /// assert!(image.description().is_none());
    /// ```
    ///
    /// ```
    /// use feed::rss::ImageBuilder;
    ///
    /// let description_string = "This is a test".to_owned();
    /// let image = ImageBuilder::new()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .link("http://jupiterbroadcasting.com/")
    ///     .description(Some(description_string.clone()))
    ///     .finalize();
    /// let description_option = image.description();
    /// assert!(description_option.is_some());
    /// let description = description_option.unwrap();
    /// assert_eq!(description.clone(), description);
    /// ```
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }
}
