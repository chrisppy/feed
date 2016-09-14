# Changelog

## master
+ New Features
    + Atom Feeds are now supported as an extension to rss
+ Bug Fixes
    + RSS Cloud Protocol has been fixed to only support the protocols noted in the SPEC
    + RSS SkipHours can now only by 0 - 23
    + RSS SkipDays can now only be Monday - Sunday
    + All integer values must be positive
+ Breaking Changes
    + In RSS Features where dealing with URLs the String is parsed into url::Url from the Builder
    + In RSS Features where dealing with dates the String is parsed into chrono::DateTime from the Builder
+ Tests
    + None
+ Maintenance
    + None

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
