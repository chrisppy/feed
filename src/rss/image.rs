// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

//! The fields under image can be retrieved by using the methods under `Image`
//! and the fields can be set for image by using the methods under `ImageBuilder`.

/// This `Image` struct contains all the items that exist for the image field under 'Channel'.
#[derive(Clone)]
pub struct Image {
    url: String,
    title: String,
    link: String,
    width: i64,
    height: i64,
    description: Option<String>,
}


impl Image {
    /// Get the url that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    /// let image = ImageBuilder::new()
    ///     .url(url)
    ///     .finalize();
    /// assert_eq!(url.to_string(), image.url());
    /// ```
    pub fn url(&self) -> String {
        self.url.clone()
    }


    /// Get the title that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let title = "LAS 300 Logo";
    /// let image = ImageBuilder::new()
    ///     .title(title)
    ///     .finalize();
    /// assert_eq!(title.to_string(), image.title());
    /// ```
    pub fn title(&self) -> String {
        self.title.clone()
    }


    /// Get the link that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    /// let image = ImageBuilder::new()
    ///     .link(link)
    ///     .finalize();
    /// assert_eq!(link.to_string(), image.link());
    /// ```
    pub fn link(&self) -> String {
        self.link.clone()
    }


    /// Get the width that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let default: i64 = 88;
    /// let image = ImageBuilder::new()
    ///     .width(None)
    ///     .finalize();
    /// assert_eq!(default, image.width());
    /// ```
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let width: i64 = 60;
    /// let image = ImageBuilder::new()
    ///     .width(Some(width))
    ///     .finalize();
    /// assert_eq!(width, image.width());
    /// ```
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let width: i64 = 777;
    /// let max: i64 = 144;
    /// let image = ImageBuilder::new()
    ///     .width(Some(width))
    ///     .finalize();
    /// assert_eq!(max, image.width());
    /// ```
    pub fn width(&self) -> i64 {
        self.width.clone()
    }


    /// Get the height that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let default: i64 = 31;
    /// let image = ImageBuilder::new()
    ///     .height(None)
    ///     .finalize();
    /// assert_eq!(default, image.height());
    /// ```
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let height: i64 = 60;
    /// let image = ImageBuilder::new()
    ///     .height(Some(height))
    ///     .finalize();
    /// assert_eq!(height, image.height());
    /// ```
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let height: i64 = 777;
    /// let max: i64 = 400;
    /// let image = ImageBuilder::new()
    ///     .height(Some(height))
    ///     .finalize();
    /// assert_eq!(max, image.height());
    /// ```
    pub fn height(&self) -> i64 {
        self.height.clone()
    }


    /// Get the description that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let image = ImageBuilder::new()
    ///     .description(None)
    ///     .finalize();
    /// assert!(image.description().is_none());
    /// ```
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let description_string = "This is a test".to_string();
    /// let image = ImageBuilder::new()
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


/// This `ImageBuilder` struct creates the `Image`.
pub struct ImageBuilder {
    url: String,
    title: String,
    link: String,
    width: i64,
    height: i64,
    description: Option<String>,
}


impl ImageBuilder {
    /// Construct a new `ImageBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let image_builder = ImageBuilder::new();
    /// ```
    pub fn new() -> ImageBuilder {
        ImageBuilder {
            url: String::new(),
            title: String::new(),
            link: String::new(),
            width: 88,
            height: 31,
            description: None,
        }
    }


    /// Set the url that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg");
    /// ```
    pub fn url(&mut self, url: &str) -> &mut ImageBuilder {
        self.url = url.to_string();
        self
    }


    /// Set the title that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.title("LAS 300 Logo");
    /// ```
    pub fn title(&mut self, title: &str) -> &mut ImageBuilder {
        self.title = title.to_string();
        self
    }


    /// Set the link that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.link("http://www.jupiterbroadcasting.com");
    /// ```
    pub fn link(&mut self, link: &str) -> &mut ImageBuilder {
        self.link = link.to_string();
        self
    }


    /// Set the width that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
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
            }
            self.width = size;
        }
        self
    }


    /// Set the height that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
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
            }
            self.height = size;
        }
        self
    }


    /// Set the description that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.description(Some("This is a test".to_string()));
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
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let image = ImageBuilder::new()
    ///         .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///         .title("LAS 300 Logo")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .width(Some(88))
    ///         .height(Some(88))
    ///         .description(Some("This is a test".to_string()))
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Image {
        Image {
            url: self.url.clone(),
            title: self.title.clone(),
            link: self.link.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            description: self.description.clone(),
        }
    }
}
