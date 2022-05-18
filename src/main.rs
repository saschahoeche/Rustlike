use bevy::prelude::*;

pub const CLEAR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor{
        width: 800.0,
        height: 800.0,
        title: "Rustlike".to_string(),
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
        .run();
}
