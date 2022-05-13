use azulrs::visor::{
    backend::TestBackend,
    layout::Layout,
    renderer::RootedRenderer,
    view::{PanelBuilder, TextView},
    Component,
};

#[test]
fn test_drawn_area_textbox_linebreaks() {
    let hello = TextView::new(String::from("Hello\nWorld"));
    let mut backend = TestBackend::default();
    let mut writer = RootedRenderer::default_with_writer(&mut backend);

    hello.render(&mut writer);

    pretty_assertions::assert_eq!(writer.get_drawn_area_for_active_layer(), (5, 2));
}

#[test]
fn test_drawn_area2() {
    let hellos = ["Hello", "World"].map(TextView::from).map(|tview| {
        Box::new(
            PanelBuilder::default()
                .component(Box::new(tview) as Box<_>)
                .build()
                .unwrap(),
        ) as Box<_>
    });

    /*
    ┌─────┐
    │Hello│
    └─────┘
    ┌─────┐
    │Hello│
    └─────┘
    */
    let layout = Layout::vertical(Vec::from(hellos));
    let mut backend = TestBackend::default();
    let mut writer = RootedRenderer::default_with_writer(&mut backend);

    layout.render(&mut writer);

    pretty_assertions::assert_eq!(writer.get_drawn_area_for_active_layer(), (7, 6));
}

#[test]
fn test_drawn_area3() {
    let hellos = ["Hello", "World"].map(TextView::from).map(|tview| {
        Box::new(
            PanelBuilder::default()
                .component(Box::new(tview) as Box<_>)
                .build()
                .unwrap(),
        ) as Box<_>
    });

    /*
    ┌─────┐┌─────┐
    │Hello││Hello│
    └─────┘└─────┘
    */
    let layout = Layout::horizontal(Vec::from(hellos));
    let mut backend = TestBackend::default();
    let mut writer = RootedRenderer::default_with_writer(&mut backend);

    layout.render(&mut writer);

    pretty_assertions::assert_eq!(writer.get_drawn_area_for_active_layer(), (14, 3));
}

#[test]
fn test_drawn_area4() {
    let hellos = ["Hello", "World"].map(TextView::from).map(|tview| {
        Box::new(
            PanelBuilder::default()
                .component(Box::new(tview) as Box<_>)
                .build()
                .unwrap(),
        ) as Box<_>
    });

    /*
    ┌──────────────┐
    │┌─────┐┌─────┐│
    ││Hello││Hello││
    │└─────┘└─────┘│
    └──────────────┘
    */
    let layout = Layout::horizontal(Vec::from(hellos));
    let panel = PanelBuilder::default()
        .component(Box::new(layout))
        .build()
        .unwrap();
    let mut backend = TestBackend::default();
    let mut writer = RootedRenderer::default_with_writer(&mut backend);

    panel.render(&mut writer);

    pretty_assertions::assert_eq!(writer.get_drawn_area_for_active_layer(), (16, 5));
}
