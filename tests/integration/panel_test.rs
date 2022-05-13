use furnace::visor::{
    layout::Layout,
    view::{PanelBuilder, TextView},
};

use crate::helpers::expect_component;

#[test]
fn test_oneline_panel() {
    let hello = TextView::new(String::from("Hello"));
    let panel = PanelBuilder::default()
        .component(Box::new(hello))
        .build()
        .unwrap();
    let expected = r#"
┌─────┐
│Hello│
└─────┘"#
        .trim_start();
    expect_component(panel, expected);
}

#[test]
fn test_panel_with_padding() {
    let hello = TextView::new(String::from("Hello"));
    let panel = PanelBuilder::default()
        .component(Box::new(hello))
        .padding(1)
        .build()
        .unwrap();
    let expected = r#"
┌───────┐
│       │
│ Hello │
│       │
└───────┘"#
        .trim_start();
    expect_component(panel, expected);
}

#[test]
fn test_panel_with_longer_title_than_content_should_expand_width() {
    let hello = TextView::new(String::from("Hello"));
    let panel = PanelBuilder::default()
        .component(Box::new(hello))
        .name("Very long title")
        .build()
        .unwrap();
    let expected = r#"
┌| Very long title |┐
│Hello              │
└───────────────────┘"#
        .trim_start();
    expect_component(panel, expected);
}

#[test]
fn test_panel_with_name_odd() {
    let hello = TextView::new(String::from("Hello world"));
    let panel = PanelBuilder::default()
        .component(Box::new(hello))
        .name("x")
        .build()
        .unwrap();
    let expected = r#"
┌───| x |───┐
│Hello world│
└───────────┘"#
        .trim_start();
    expect_component(panel, expected);
}

/*
* Test if the panel has an even number of character, but the box width is uneven, the
* title becomes slightly off center, but it still lines up with the borders.
*/
#[test]
fn test_panel_with_name_even() {
    let hello = TextView::new(String::from("Hello world"));
    let panel = PanelBuilder::default()
        .component(Box::new(hello))
        .name("xx")
        .build()
        .unwrap();
    let expected = r#"
┌──| xx |───┐
│Hello world│
└───────────┘"#
        .trim_start();
    expect_component(panel, expected);
}

#[test]
fn test_two_panels_horizontally() {
    let hellos = ["Hello", "Hello"]
        .into_iter()
        .map(|s| {
            let panel = PanelBuilder::default()
                .component(Box::new(TextView::new(String::from(s))) as Box<_>)
                .build()
                .unwrap();
            Box::new(panel) as Box<_>
        })
        .collect();
    let panel = PanelBuilder::default()
        .component(Box::new(Layout::horizontal(0, hellos)))
        .build()
        .unwrap();
    let expected = r#"
┌──────────────┐
│┌─────┐┌─────┐│
││Hello││Hello││
│└─────┘└─────┘│
└──────────────┘"#
        .trim_start();
    expect_component(panel, expected);
}

#[test]
fn test_panel_in_layout_in_panel() {
    let hellos = ["Hello"]
        .into_iter()
        .map(|s| {
            let panel = PanelBuilder::default()
                .component(Box::new(TextView::new(String::from(s))) as Box<_>)
                .build()
                .unwrap();
            Box::new(panel) as Box<_>
        })
        .collect();
    let panel = PanelBuilder::default()
        .component(Box::new(Layout::horizontal(0, hellos)))
        .build()
        .unwrap();
    let expected = r#"
┌───────┐
│┌─────┐│
││Hello││
│└─────┘│
└───────┘"#
        .trim_start();
    expect_component(panel, expected);
}

#[test]
fn test_panel_in_panel() {
    let hello = PanelBuilder::default()
        .component(Box::new(TextView::new(String::from("Hello"))) as Box<_>)
        .name("i")
        .build()
        .unwrap();

    let panel = PanelBuilder::default()
        .component(Box::new(hello))
        .build()
        .unwrap();
    let expected = r#"
┌───────┐
│┌| i |┐│
││Hello││
│└─────┘│
└───────┘"#
        .trim_start();
    expect_component(panel, expected);
}

#[test]
fn test_vertical_layout_linebreaks() {
    let hellos: [Box<_>; 3] =
        ["Hello", "world", "again"].map(|s| Box::new(TextView::from(s)) as Box<_>);
    let panel = Layout::vertical(0, Vec::from(hellos));
    let expected = r#"Hello
world
again"#;

    expect_component(panel, expected);
}
