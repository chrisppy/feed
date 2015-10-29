// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
#[cfg(test)]
mod test {

    extern crate feedreader;

    #[test]
    fn get_channel_title() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_title(feed), "The Linux Action Show! OGG".to_string());
    }

    #[test]
    fn get_channel_link() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_link(feed), "http://www.jupiterbroadcasting.com".to_string());
    }

    #[test]
    fn get_channel_description() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_description(feed), "Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!".to_string());
    }

    #[test]
    fn get_channel_generator() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_generator(feed).unwrap(), "Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34) http://reinventedsoftware.com/feeder/".to_string());
    }

    #[test]
    fn get_channel_docs() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_docs(feed).unwrap(), "http://blogs.law.harvard.edu/tech/rss".to_string());
    }

    #[test]
    fn get_channel_language() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_language(feed).unwrap(), "en".to_string());
    }

    #[test]
    fn get_channel_pub_date() {
        //let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        //assert_eq!(feedreader::channel::get_pub_date(feed).unwrap(), "Sun, 25 Oct 2015 08:30:31 -0700".to_string());
    }

    #[test]
    fn get_channel_last_build_date() {
        //let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        //assert_eq!(feedreader::channel::get_last_build_date(feed).unwrap(), "Sun, 25 Oct 2015 08:30:31 -0700".to_string());
    }

    #[test]
    fn get_channel_image() {
        //let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        //assert_eq!(feedreader::channel::get_image(feed).unwrap(), "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg".to_string());
    }

    #[test]
    fn get_channel_copyright() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_copyright(feed).is_none(), true);
    }

    #[test]
    fn get_channel_managing_editor() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_managing_editor(feed).is_none(), true);
    }

    #[test]
    fn get_channel_web_master() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_web_master(feed).is_none(), true);
    }

    #[test]
    fn get_channel_ttl() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_ttl(feed).is_none(), true);
    }

    #[test]
    fn get_channel_rating() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_rating(feed).is_none(), true);
    }

    #[test]
    fn get_channel_skip_hours() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_skip_hours(feed).is_none(), true);
    }

    #[test]
    fn get_channel_skip_days() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feedreader::channel::get_skip_days(feed).is_none(), true);
    }

    #[test]
    fn get_item_title() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        let title = feedreader::item::get_title(feed, 0).unwrap();
        assert_eq!(title.is_empty(), false);
    }

    #[test]
    fn get_item_link() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        let link = feedreader::item::get_link(feed, 0).unwrap();
        assert_eq!(link.is_empty(), false);
    }

    #[test]
    fn get_item_description() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        let description = feedreader::item::get_description(feed, 0).unwrap();
        assert_eq!(description.is_empty(), false);
    }

    #[test]
    fn get_item_author() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        let author = feedreader::item::get_author(feed, 0).unwrap();
        assert_eq!(author.is_empty(), false);
    }

    #[test]
    fn get_item_comments() {
        //assert!(false);
    }

    #[test]
    fn get_item_pub_date() {
        //assert!(false);
    }

    #[test]
    fn get_channel_categories() {
        //assert!(false);
    }

    #[test]
    fn get_item_categories() {
        //assert!(false);
    }

    #[test]
    fn new() {
        let feed = feedreader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
        assert_eq!(feed.is_empty(), false);
    }
}
