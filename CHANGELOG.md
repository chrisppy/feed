# Changelog

## master
+ New Features
    + None
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
        + Category
            + category -> name
    	+ Source
    	    + source -> title
    	    + title is now `Option<String>`
+ Refactor
    + reader and writer have been moved into utils
    + reader has been refactored to use rss crate
    + writer has been refactored to use rss crate
+ Tests
    + None
+ Maintenance
    + None

## feed 1.2.4
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

## feed 1.2.3
+ simplified Doc versioning

## feed 1.2.2
+ updated url to 1.1
+ updated quick-xml to 0.2

## feed 1.2.1
+ fixed build error

## feed 1.2.0
+ Restructured the structs to make more sense

## feed 1.1.3
+ fixed rust doc for to_xml

## feed 1.1.2
+ updated quick-xml to 0.1.9
+ updated url to 0.5.7

## feed 1.1.1
+ Fixed some typos
+ updated chrono to 0.2.21

## feed 1.1.0
+ added to_xml
+ updated quick-xml to 0.1.8
+ updated chrono to 0.2.20

## feed 1.0.5
+ changed println to debug
+ updated curl to 0.2.18

## feed 1.0.4
+ Minor fixes
+ updated dependencies

## feed 1.0.3
+ Minor fixes

## feed 1.0.2
+ Minor fixes

## feed 1.0.1
+ Ran rustfmt
+ Resolved all clippy warnings
+ Relicensed to LGPLv3

## feed 1.0.0
+ Renamed and complete rewrite for ease of use and to follow rust standards

## FeedReader 0.1.0
+ Initial Release
