// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
mod rss;

extern crate feed;
extern crate url;

use feed::{Feed, FeedBuilder};
use url::Url;

#[test]
fn from_str() {
    let feed_str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <?xml-stylesheet type="text/xsl" media="screen" href="/~d/styles/rss2enclosuresfull.xsl"?>
    <?xml-stylesheet type="text/css" media="screen" href="http://feeds.feedburner.com/~d/styles/itemcontent.css"?>
    <rss xmlns:atom="http://www.w3.org/2005/Atom" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:media="http://search.yahoo.com/mrss/" version="2.0">
        <channel>
            <title>The Linux Action Show! OGG</title>
            <link>http://www.jupiterbroadcasting.com</link>
            <description>Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!</description>
            <generator>Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34) http://reinventedsoftware.com/feeder/</generator>
            <docs>http://blogs.law.harvard.edu/tech/rss</docs>
            <language>en</language>
            <copyright>2015 JupiterBroadcasting.com - All Rights Reserved</copyright>
            <managingEditor>chris@jupiterbroadcasting.com (Chris Fisher)</managingEditor>
            <webMaster>chris@jupiterbroadcasting.com (Chris Fisher)</webMaster>
            <pubDate>Sun, 13 Mar 2016 20:02:02 -0700</pubDate>
            <lastBuildDate>Sun, 13 Mar 2016 20:02:02 -0700</lastBuildDate>
            <category>Podcasts</category>
            <category>Linux</category>
            <cloud domain="rpc.sys.com" port="80" path="/RPC2" registerProcedure="pingMe" protocol="soap"/>
            <ttl>60</ttl>
            <image>
                <link>http://www.jupiterbroadcasting.com</link>
                <url>http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg</url>
                <title>LAS 300 Logo</title>
                <description>Logo for jupiterbroadcasting' Linux Action Show</description>
            </image>
            <rating>PG-13</rating>
            <skipHours>
                <hour>0</hour>
                <hour>1</hour>
                <hour>2</hour>
                <hour>3</hour>
            </skipHours>
            <skipDays>
                <day>Monday</day>
                <day>Tuesday</day>
            </skipDays>
            <textInput>
                <title>Enter Comment</title>
                <description>Provide Feedback</description>
                <name>Comment</name>
                <link>www.example.com/feedback</link>
            </textInput>
            <item>
                <title>Making Music with Linux | LAS 408</title>
                <link>http://www.jupiterbroadcasting.com/97561/making-music-with-linux-las-408/</link>
                <description><![CDATA[<p>What’s the best software under Linux to create music? We answer that & the best hardware that work together. We have something for the musician to casual hardware geek, we cover the dark art of creating music under Linux as we go inside Noah’s music studio and see what he uses.</p><p>Plus we chat with ownCloud’s Community Manager about the new features in version 9, our thoughts on Microsoft’s SQL for Linux announcement & more!</p>]]></description>
                <pubDate>Sun, 13 Mar 2016 19:58:31 -0700</pubDate>
                <enclosure url="http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/linuxactionshowep408.ogg" length="70772893" type="audio/ogg" />
                <guid isPermaLink="false">9DE46946-2F90-4D5D-9047-7E9165C16E7C</guid>
                <author>chris@linuxactionshow.com</author>
                <category>Podcasts</category>
                <category>Linux</category>
                <comments>test.com/comments</comments>
                <source url="http://www.tomalak.org/links2.xml">Tomalak's Realm</source>
            </item>
            <item>
                <title>3rd Slice of Pi with Extra Mycroft | LAS 407</title>
                <link>http://www.jupiterbroadcasting.com/97166/3rd-slice-of-pi-with-extra-mycroft-las-407/</link>
                <description><![CDATA[<p>In special Rasberry Pi 3 edition of the show we look at the new hardware, review & chat with Mycroft CTO Ryan Sipes on how important the Raspberry Pi is for development of their open artificial intelligence platform & get the latest news.</p><p>Plus replacing Spotify on Linux, the new Microsoft lock-in, our hosts face a moral quandary & more!</p>]]></description>
                <pubDate>Sun, 06 Mar 2016 23:00:05 -0800</pubDate>
                <enclosure url="http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/linuxactionshowep407.ogg" length="77861527" type="audio/ogg" />
                <guid isPermaLink="false">55EE8E2F-138B-431C-8D38-32A971F578BA</guid>
                <itunes:explicit>no</itunes:explicit>
                <author>chris@linuxactionshow.com</author>
                <category>Media</category>
                <category>Linux</category>
            </item>
            <item>
                <title>Entroware Apollo: Linux Macbook Killer | LAS 406</title>
                <link>http://www.jupiterbroadcasting.com/96836/entroware-apollo-linux-macbook-killer-las-406/</link>
                <description><![CDATA[<p>Has Entroware built the ultimate 13 inch Linux laptop? We put their Apollo through our battery of tests & review this good looking Skylake powered Linux portable.</p><p>Plus Microsoft buys the folks behind Mono, Canonical might have a ZFS fight on their hands & more!</p>]]></description>
                <pubDate>Mon, 29 Feb 2016 01:45:12 -0800</pubDate>
                <enclosure url="http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jupiterbroadcasting/linuxactionshowep406.ogg" length="98408612" type="audio/ogg" />
                <guid isPermaLink="false">DA4D168C-87AA-46B2-ABC6-C4C9BF26E03D</guid>
                <author>chris@linuxactionshow.com</author>
            </item>
        </channel>
    </rss>"#;

    //let feed = FeedBuilder::new().from_str(&feed_str).finalize();
}


#[test]
fn from_url() {
    let url = match Url::parse("http://feeds2.feedburner.com/TheLinuxActionShowOGG.xml") {
        Ok(result) => result,
        Err(err)   => panic!("Url parse Error: {}", err),
    };
    let feed = FeedBuilder::new().from_url(url).finalize();
    let channel = feed.channel();
    assert_eq!("The Linux Action Show! OGG".to_string(), channel.title());
}
