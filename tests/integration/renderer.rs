use azulrs::visor::{
    backend::TestBackend,
    layout::Layout,
    renderer::RootedRenderer,
    view::{PanelBuilder, TextView},
    Component, Coords,
};

#[test]
#[ignore]
fn test_drawn_area() {
    let hello = TextView::new(String::from("Hello"));
    let panel = PanelBuilder::default()
        .component(Box::new(hello))
        .build()
        .unwrap();
    /*
    ┌─────┐
    │Hello│
    └─────┘
    */
    let mut backend = TestBackend::default();
    let mut writer = RootedRenderer::new(&mut backend, Coords::new(1, 1));

    panel.render(&mut writer);

    //pretty_assertions::assert_eq!(writer.get_drawn_area(), (7, 3));
}

#[test]
#[ignore]
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
    let layout = Layout::vertical(0, Vec::from(hellos));
    let mut backend = TestBackend::default();
    let mut writer = RootedRenderer::new(&mut backend, Coords::new(1, 1));

    layout.render(&mut writer);

    //pretty_assertions::assert_eq!(writer.get_drawn_area(), (7, 6));
}

#[test]
#[ignore]
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
    ┌─────┐ ┌─────┐
    │Hello│ │Hello│
    └─────┘ └─────┘
    */
    let layout = Layout::horizontal(0, Vec::from(hellos));
    let mut backend = TestBackend::default();
    let mut writer = RootedRenderer::new(&mut backend, Coords::new(1, 1));

    layout.render(&mut writer);

    //pretty_assertions::assert_eq!(writer.get_drawn_area(), (14, 3));
}
