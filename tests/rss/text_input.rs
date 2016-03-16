// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use feed::rss::text_input::TextInputBuilder;

#[test]
fn title() {
    let title = "Enter Comment";
    let text_input = TextInputBuilder::new()
        .title(title)
        .finalize();
    assert_eq!(title.to_string(), text_input.title());
}


#[test]
fn description() {
    let description = "Provided Feedback";
    let text_input = TextInputBuilder::new()
        .description(description)
        .finalize();
    assert_eq!(description.to_string(), text_input.description());
}


#[test]
fn name() {
    let name = "Comment";
    let text_input = TextInputBuilder::new()
        .name(name)
        .finalize();
    assert_eq!(name.to_string(), text_input.name());
}


#[test]
fn link() {
    let link = "www.example.com/feedback";
    let text_input = TextInputBuilder::new()
        .link(link)
        .finalize();
    assert_eq!(link.to_string(), text_input.link());
}
