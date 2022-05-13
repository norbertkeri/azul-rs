use azulrs::visor::{layout::Layout, view::TextView};
use helpers::{expect_component, to_textviews};

mod bag_test;
mod factory_test;
mod helpers;
mod layout_test;
mod model_views;
mod panel_test;
mod renderer;
mod util;

fn testdata(data: &str) -> &str {
    data.trim_start()
}

#[test]
fn test_textview() {
    let hello = TextView::from("Hello");
    expect_component(hello, "Hello");
}

#[test]
fn test_horizontal_layout_nolbrk() {
    let stuff = to_textviews(["Hello", "World"]);

    let layout = Layout::horizontal(0, stuff);
    expect_component(layout, "HelloWorld");
}

#[test]
fn test_horizontal_layout_with_lbrk() {
    let components = to_textviews(["Second\ntext is very long", "Bye World"]);

    let layout = Layout::horizontal(0, components);
    let expected = testdata(
        "
Second           Bye World
text is very long",
    );

    expect_component(layout, expected);
}

#[test]
fn test_vertical_layout_with_lbrk() {
    let components = to_textviews(["Second\ntext is very long", "Bye World"]);

    let layout = Layout::vertical(0, components);
    let expected = testdata(
        r#"
Second
text is very long
Bye World"#,
    );

    expect_component(layout, expected);
}
