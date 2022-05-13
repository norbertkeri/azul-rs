use azulrs::visor::{
    backend::TestBackend, layout::Layout, renderer::RootedRenderer, view::TextView, Component,
    Coords,
};

use crate::helpers::{assert_dimensions, expect_component, to_textviews};
use test_case::test_case;

#[test]
fn test_individual_layer_sizes() {
    let texts = to_textviews(["Hello", "World", "Bye"]);

    let mut backend = TestBackend::default();
    let mut writer = RootedRenderer::default_with_writer(&mut backend);

    let mut areas = vec![];
    for (i, t) in texts.into_iter().enumerate() {
        let root = Coords::new(i as u16, i as u16);
        let n = writer.render_into_layer(format!("level-{i}"), root, t.as_ref());
        areas.push(writer.get_drawn_area(n));
    }

    pretty_assertions::assert_eq!(writer.get_drawn_area_for_active_layer(), (6, 3));
    pretty_assertions::assert_eq!(areas, vec![(5, 1), (5, 1), (3, 1)]);
}

#[test_case(["Bye", "World"], Layout::vertical, (5,2))]
#[test_case(["Hello", "World"], Layout::horizontal, (10,1))]
#[test_case(["Bye\nWorld", "Come back"], Layout::horizontal, (14,2))]
#[test_case(["Bye\nWorld", "Come back"], Layout::vertical, (9,3))]
fn test_layout_dimensions_vertical<const N: usize>(
    words: [&str; N],
    layout: fn(Vec<Box<dyn Component>>) -> Layout<'static>,
    expected_area: (u16, u16),
) {
    let stuff = to_textviews(words);
    let layout = layout(stuff);
    assert_dimensions(&layout, expected_area);
}

#[test_case(["Hello", "world", "again", "and", "again"],
r#"Hello
world
again
and
again"#
, Layout::vertical)]
#[test_case(["Hello", "World"], "HelloWorld", Layout::horizontal)]
#[test_case(["Second\ntext is very long", "Bye World"],
r#"
Second
text is very long
Bye World"#
, Layout::vertical)]
#[test_case(["Second\ntext is very long", "Bye World"],
r#"
Second           Bye World
text is very long"#
, Layout::horizontal)]
fn test_layout_rendering<const N: usize>(
    texts: [&str; N],
    expected_output: &str,
    layout: fn(Vec<Box<dyn Component>>) -> Layout<'static>,
) {
    let hellos = to_textviews(texts);
    let panel = layout(hellos);
    expect_component(panel, expected_output.trim());
}

#[test]
fn test_mixed_layouts() {
    let layout = Layout::vertical(
        vec![
            Box::new(TextView::from("v1")),
            Box::new(Layout::horizontal(
                vec![
                    Box::new(TextView::from("h1")),
                    Box::new(TextView::from("h2")),
                ],
            )),
        ],
    );
    let expected = r#"
v1
h1h2
"#
    .trim();

    let mut backend = TestBackend::default();
    let mut writer = RootedRenderer::default_with_writer(&mut backend);
    layout.render(&mut writer);
    assert_eq!(writer.get_drawn_area_for_active_layer(), (4, 2));

    expect_component(layout, expected);
}
