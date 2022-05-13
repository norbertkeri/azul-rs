use azulrs::visor::{
    layout::Layout,
    view::{PanelBuilder, TextView},
    Component,
};

use crate::helpers::{assert_dimensions, expect_component};
use test_case::test_case;

fn textview_in_panel(text: &str) -> Box<dyn Component> {
    let panel = PanelBuilder::default()
        .component(Box::new(TextView::new(String::from(text))) as Box<_>)
        .build()
        .unwrap();
    Box::new(panel) as Box<_>
}

#[test_case(
    r#"Hello
Hello
Hello"#,
    r#"
┌─────┐
│Hello│
│Hello│
│Hello│
└─────┘
"#,
    0,
    (7, 5)
)]
#[test_case(
    "Hello",
    r#"
┌───────┐
│       │
│ Hello │
│       │
└───────┘
"#,
    1,
(9, 5)
)]
fn test_panel_with_border(
    text: &str,
    expected_output: &str,
    padding: u16,
    expected_dimensions: (u16, u16),
) {
    let hello = TextView::new(String::from(text));
    let panel = PanelBuilder::default()
        .component(Box::new(hello))
        .padding(padding)
        .build()
        .unwrap();
    assert_dimensions(&panel, expected_dimensions);
    expect_component(panel, expected_output.trim());
}

#[test_case(
    "Very long title",
    "Hello",
    "
┌| Very long title |┐
│Hello              │
└───────────────────┘
"
)]
#[test_case(
    "x",
    "Hello world",
    "
┌───| x |───┐
│Hello world│
└───────────┘
"
)]
#[test_case(
    "xx",
    "Hello world",
    "
┌──| xx |───┐
│Hello world│
└───────────┘
"
)]
fn test_panel_title(title: &str, content: &str, expected: &str) {
    let hello = TextView::new(String::from(content));
    let panel = PanelBuilder::default()
        .component(Box::new(hello))
        .name(title)
        .build()
        .unwrap();
    expect_component(panel, expected.trim());
}

#[test]
fn test_two_panels_horizontally() {
    let hellos = ["Hello", "Hello"]
        .into_iter()
        .map(textview_in_panel)
        .collect();
    let panel = PanelBuilder::default()
        .component(Box::new(Layout::horizontal(hellos)))
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
    let hellos = ["Hello"].into_iter().map(textview_in_panel).collect();
    let panel = PanelBuilder::default()
        .component(Box::new(Layout::horizontal(hellos)))
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
        .component(Box::new(TextView::from("Hello")) as Box<_>)
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
