# Changelog

## feed master
+ New Features
    + None
+ Bug Fixes
    + None
+ Breaking Changes
    + None
+ Refactor
    + None
+ Tests
    + None
+ Maintenance
    + None  

## feed 2.0.2 (2017-03-10)
+ New Features
    + None
+ Bug Fixes
    + Clippy fixes
+ Breaking Changes
    + None
+ Refactor
    + None
+ Tests
    + None
+ Maintenance
    + Update chrono to 0.3  

## feed 2.0.2 (2017-03-10)
+ New Features
    + None
+ Bug Fixes
    + None
+ Breaking Changes
    + None
+ Refactor
    + Clippy fixes
+ Tests
    + None
+ Maintenance
    + Update chrono to 0.3  

## feed 2.0.0 (2017-01-21)
+ New Features
    + Added Day enum
    + Added CloudProtocol enum
+ Bug Fixes
    + Constrain image url to only accept JPEG, PNG, and GIF
+ Breaking Changes
    + read from url parameter has been changed from url to str
    + new() has been removed from FeedBuilder
    + The following values have been changed:
    	+ Guid
    	    + permalink -> is_permalink
    	    + guid -> value
        + Enclosure
            + enclosure_type -> mime_type
            + mime_type is now `Mime`
        + Category
            + category -> name
    	+ Source
    	    + source -> title
    	    + title is now `Option<String>`
        + Channel
            + skip_days is now `Option<Vec<Day>>`
        + Cloud
            + protocol is now `CloudProtocol`
+ Refactor
    + reader and writer have been moved into utils
    + reader has been refactored to use rss crate
    + writer has been refactored to use rss crate
+ Tests
    + None
+ Maintenance
    + None

## feed 1.2.4 (2016-09-16)
+ New Features
    + None
+ Bug Fixes
    + clippy cyclomatic complexity warning resolved
    + allow channels feeds that does not end with 'xml'
+ Breaking Changes
    + None
+ Tests
    + None
+ Maintenance
    + Refactored lib.rs to move impl code into feed.rs and feed_builder.rs
    + Added IntelliJ and Linux sections to gitignore
    + Updated README to use docs.rs
    + Update quick-xml to 0.4
    + Update url to 1.2
    + Update curl to 0.3

## feed 1.2.3 (2016-05-12)
+ simplified Doc versioning

## feed 1.2.2 (2016-05-06)
+ updated url to 1.1
+ updated quick-xml to 0.2

## feed 1.2.1 (2016-04-19)
+ fixed build error

## feed 1.2.0 (2016-04-19)
+ Restructured the structs to make more sense

## feed 1.1.3 (2016-04-08)
+ fixed rust doc for to_xml

## feed 1.1.2 (2016-04-07)
+ updated quick-xml to 0.1.9
+ updated url to 0.5.7

## feed 1.1.1 (2016-04-01)
+ Fixed some typos
+ updated chrono to 0.2.21

## feed 1.1.0 (2016-03-31)
+ added to_xml
+ updated quick-xml to 0.1.8
+ updated chrono to 0.2.20

## feed 1.0.5 (2016-03-26)
+ changed println to debug
+ updated curl to 0.2.18

## feed 1.0.4 (2016-03-26)
+ Minor fixes
+ updated dependencies

## feed 1.0.3 (2016-03-22)
+ Minor fixes

## feed 1.0.2 (2016-03-21)
+ Minor fixes

## feed 1.0.1 (2016-03-20)
+ Ran rustfmt
+ Resolved all clippy warnings
+ Relicensed to LGPLv3

## feed 1.0.0 (2016-03-20)
+ Renamed and complete rewrite for ease of use and to follow rust standards

## FeedReader 0.1.0 (2015-10-29)
+ Initial Release
